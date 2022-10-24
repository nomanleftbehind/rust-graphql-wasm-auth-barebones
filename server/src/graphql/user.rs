use crate::graphql::{
    context::ContextExt,
    dataloaders::{CreatedPostsLoader, UpdatedPostsLoader},
    post::Post,
};
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
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[ComplexObject]
impl User {
    async fn created_posts(&self, ctx: &Context<'_>) -> Result<Vec<Post>> {
        let loader = ctx.get_loader::<DataLoader<CreatedPostsLoader>>();
        let posts = loader.load_one(self.id).await?;
        // Need to return empty vector if user has no written posts
        let result = posts.unwrap_or(vec![]);

        Ok(result)
    }

    async fn updated_posts(&self, ctx: &Context<'_>) -> Result<Vec<Post>> {
        let loader = ctx.get_loader::<DataLoader<UpdatedPostsLoader>>();
        let posts = loader.load_one(self.id).await?;
        // Need to return empty vector if user has no updated posts
        let result = posts.unwrap_or(vec![]);

        Ok(result)
    }
}

#[derive(InputObject, Debug)]
pub struct RegisterUserInput {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(InputObject, Debug)]
pub struct LoginUserInput {
    pub email: String,
    pub password: String,
}
