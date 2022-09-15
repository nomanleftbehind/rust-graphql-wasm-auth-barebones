use crate::gql::{context::ContextExt, dataloaders::PostLoader, post::Post};
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
        let result = posts.ok_or_else(|| "Not found".into());

        result
    }
}
