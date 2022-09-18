use crate::gql::user::User;
use crate::telemetry::spawn_blocking_with_tracing;
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use secrecy::{ExposeSecret, Secret};
use sqlx::{postgres::PgRow, FromRow, PgExecutor, Postgres, Row, Transaction};
use uuid::Uuid;

pub struct Credentials {
    id: Uuid,
    password_hash: Secret<String>,
}

impl<'r> FromRow<'r, PgRow> for Credentials {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            password_hash: Secret::new(row.try_get("password_hash")?),
        })
    }
}


async fn fetch_user_credentials<'e, E: PgExecutor<'e>>(
    pool: E,
    email: String,
) -> Result<Option<Credentials>, sqlx::Error> {
    let cred = sqlx::query_as("SELECT id, password_hash FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await?;
    Ok(cred)
}

pub async fn validate_credentials<'e, E: PgExecutor<'e>>(
    pool: E,
    email: String,
    password: Secret<String>,
) -> Option<Uuid> {
    let cred = fetch_user_credentials(pool, email).await.ok()??;
    spawn_blocking_with_tracing(move || verify_password_hash(cred, password))
        .await
        .ok()?
}

#[tracing::instrument(name = "Verify password hash", skip(credentials, password))]
fn verify_password_hash(credentials: Credentials, password: Secret<String>) -> Option<Uuid> {
    let hash = PasswordHash::new(credentials.password_hash.expose_secret()).ok()?;
    Argon2::default()
        .verify_password(password.expose_secret().as_bytes(), &hash)
        .ok()?;
    Some(credentials.id)
}


// #[tracing::instrument(name = "Change password", skip(password, pool))]
// pub async fn change_password(
//     user_id: uuid::Uuid,
//     password: Secret<String>,
//     pool: &PgPool,
// ) -> Result<(), anyhow::Error> {
//     let password_hash = spawn_blocking_with_tracing(move || compute_password_hash(password))
//         .await?
//         .context("Failed to hash password")?;
//     sqlx::query!(
//         r#"
//         UPDATE users
//         SET password_hash = $1
//         WHERE id = $2
//         "#,
//         password_hash.expose_secret(),
//         user_id
//     )
//     .execute(pool)
//     .await
//     .context("Failed to change user's password in the database.")?;
//     Ok(())
// }

// fn compute_password_hash(password: Secret<String>) -> Result<Secret<String>, anyhow::Error> {
//     let salt = SaltString::generate(&mut rand::thread_rng());
//     let password_hash = Argon2::new(
//         Algorithm::Argon2id,
//         Version::V0x13,
//         Params::new(15000, 2, 1, None).unwrap(),
//     )
//     .hash_password(password.expose_secret().as_bytes(), &salt)?
//     .to_string();
//     Ok(Secret::new(password_hash))
// }

pub async fn register(
    transaction: &mut Transaction<'_, Postgres>,
    email: String,
    password: String,
) -> Result<User, sqlx::Error> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    // Match production parameters
    let password_hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    )
    .hash_password(password.as_bytes(), &salt)
    .unwrap()
    .to_string();
    let user = sqlx::query_as(
        r#"INSERT INTO users (email, password_hash)
        VALUES ($1, $2)
        RETURNING email, id, password_hash, post_signature;
        "#,
    )
    .bind(email)
    .bind(password_hash)
    .fetch_one(transaction)
    .await;

    user
}
