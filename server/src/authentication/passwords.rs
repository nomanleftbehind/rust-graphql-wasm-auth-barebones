use crate::graphql::user::{RegisterUserInput, User};
use crate::telemetry::spawn_blocking_with_tracing;
use anyhow::Context;
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials.")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[derive(Debug)]
pub struct Credentials {
    pub email: String,
    pub password: Secret<String>,
}

#[tracing::instrument(name = "Get stored credentials", skip(email, pool))]
async fn get_stored_credentials(
    email: String,
    pool: &PgPool,
) -> Result<Option<(uuid::Uuid, Secret<String>)>, anyhow::Error> {
    let row = sqlx::query_as::<_, User>(
        r#"
        SELECT id, email, password, first_name, last_name
        FROM users
        WHERE email = $1
        "#,
    )
    .bind(email)
    .fetch_optional(pool)
    .await
    .context("Failed to performed a query to retrieve stored credentials.")?
    .map(|row| (row.id, Secret::new(row.password)));
    Ok(row)
}

#[tracing::instrument(name = "Validate credentials", skip(credentials, pool))]
pub async fn validate_credentials(
    credentials: Credentials,
    pool: &PgPool,
) -> Result<uuid::Uuid, AuthError> {
    let mut user_id = None;
    let mut expected_password_hash = Secret::new(
        "$argon2id$v=19$m=15000,t=2,p=1$\
        gZiV/M1gPc22ElAH/Jh1Hw$\
        CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
            .to_string(),
    );

    if let Some((stored_user_id, stored_password_hash)) =
        get_stored_credentials(credentials.email, pool).await?
    {
        user_id = Some(stored_user_id);
        expected_password_hash = stored_password_hash;
    }

    spawn_blocking_with_tracing(move || {
        verify_password_hash(expected_password_hash, credentials.password)
    })
    .await
    .context("Failed to spawn blocking task.")??;

    user_id
        .ok_or_else(|| anyhow::anyhow!("Unknown email."))
        .map_err(AuthError::InvalidCredentials)
}

#[tracing::instrument(
    name = "Validate credentials",
    skip(expected_password_hash, password_candidate)
)]
fn verify_password_hash(
    expected_password_hash: Secret<String>,
    password_candidate: Secret<String>,
) -> Result<(), AuthError> {
    let expected_password_hash = PasswordHash::new(expected_password_hash.expose_secret())
        .context("Failed to parse hash in PHC string format.")?;

    Argon2::default()
        .verify_password(
            password_candidate.expose_secret().as_bytes(),
            &expected_password_hash,
        )
        .context("Invalid password.")
        .map_err(AuthError::InvalidCredentials)
}

// #[tracing::instrument(name = "Change password", skip(password, pool))]
// pub async fn change_password(
//     user_id: uuid::Uuid,
//     password: Secret<String>,
//     pool: &PgPool,
// ) -> Result<(), anyhow::Error> {
//     let password = spawn_blocking_with_tracing(move || compute_password_hash(password))
//         .await?
//         .context("Failed to hash password")?;
//     sqlx::query!(
//         r#"
//         UPDATE users
//         SET password = $1
//         WHERE id = $2
//         "#,
//         password.expose_secret(),
//         user_id
//     )
//     .execute(pool)
//     .await
//     .context("Failed to change user's password in the database.")?;
//     Ok(())
// }

// fn compute_password_hash(password: Secret<String>) -> Result<Secret<String>, anyhow::Error> {
//     let salt = SaltString::generate(&mut rand::thread_rng());
//     let password = Argon2::new(
//         Algorithm::Argon2id,
//         Version::V0x13,
//         Params::new(15000, 2, 1, None).unwrap(),
//     )
//     .hash_password(password.expose_secret().as_bytes(), &salt)?
//     .to_string();
//     Ok(Secret::new(password))
// }

pub async fn register(
    pool: &PgPool,
    RegisterUserInput {
        email,
        password,
        first_name,
        last_name,
    }: RegisterUserInput,
) -> Result<User, sqlx::Error> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    // Match production parameters
    let password = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    )
    .hash_password(password.as_bytes(), &salt)
    .unwrap()
    .to_string();
    let user = sqlx::query_as(
        r#"INSERT INTO users (email, password, first_name, last_name)
        VALUES ($1, $2, $3, $4)
        RETURNING email, id, password, first_name, last_name;
        "#,
    )
    .bind(email)
    .bind(password)
    .bind(first_name)
    .bind(last_name)
    .fetch_one(pool)
    .await;

    user
}
