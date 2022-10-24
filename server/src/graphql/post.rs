use crate::graphql::{
    context::ContextExt, dataloaders::UserLoader, post::dataloader::DataLoader, user::User,
};
use async_graphql::*;
use sqlx::{types::time::PrimitiveDateTime, FromRow};
use uuid::Uuid;

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct Post {
    pub id: Uuid,
    pub body: String,
    pub topic: String,
    pub rank: Option<i32>,
    pub created_by_id: Uuid,
    pub created_at: PrimitiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: PrimitiveDateTime,
}

#[ComplexObject]
impl Post {
    async fn creator(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let loader = ctx.get_loader::<DataLoader<UserLoader>>();
        let created_by = loader.load_one(self.created_by_id).await?;

        Ok(created_by)
    }

    async fn modifier(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let loader = ctx.get_loader::<DataLoader<UserLoader>>();
        let updated_by = loader.load_one(self.updated_by_id).await?;

        Ok(updated_by)
    }
}
