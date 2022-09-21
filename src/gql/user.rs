use crate::authentication::Credentials;
use crate::gql::{context::ContextExt, dataloaders::PostLoader, post::Post};
use argon2::{Argon2, PasswordHash};
use async_graphql::dataloader::DataLoader;
use async_graphql::*;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, OneofObject)]
pub enum UserBy {
    Email(String),
    Id(Uuid),
}

#[derive(Serialize, Deserialize, SimpleObject, Debug, Clone, FromRow)]
#[graphql(complex)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub post_signature: Option<String>,
}

#[ComplexObject]
impl User {
    async fn posts(&self, ctx: &Context<'_>) -> Result<Vec<Post>> {
        let loader = ctx.get_loader::<DataLoader<PostLoader>>();
        let posts = loader.load_one(self.id).await?;
        // Need to return empty vector if user has no written posts
        let result = posts.unwrap_or(vec![]);

        Ok(result)
    }
}

#[derive(InputObject)]
/// Input from GQL, consume with login_user() to get a User.
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

// impl LoginUser {
//     //log the user in by validating their credentials from the database
//     pub async fn login_user(
//         self,
//         pool: &PgPool,
//         argon2: &Argon2<'_>,
//     ) -> Result<Option<Credentials>, Error> {
//         let cred = sqlx::query_as::<_, Credentials>(
//             "SELECT id, password_hash FROM users WHERE email = $1",
//         )
//         .bind(self.email)
//         .fetch_optional(pool)
//         .await?;

//         let user = cred.ok_or_else(|| Error::new("Invalid user"))?;

//         let valid = user.verify_password(argon2, self.password).map_err(|e| {
//             tracing::info!("Failed to verify password");
//             e
//         })?;

//         match valid {
//             false => Err(Error::new("Invalid password")),
//             true => Ok(user),
//         }
//     }
// }
