use crate::graphql::{context::ContextExt, dataloaders::PostLoader, post::Post};
use async_graphql::dataloader::DataLoader;
use async_graphql::*;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
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

#[derive(InputObject, Debug)]
/// Input from GraphQL, consume with login_user() to get a User.
pub struct LoginUser {
    pub email: String,
    pub password: String,
}
