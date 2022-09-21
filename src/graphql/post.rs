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
    pub user_id: Uuid,
    pub body: String,
    pub topic: String,
    pub rank: Option<i32>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

#[ComplexObject]
impl Post {
    async fn user(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let loader = ctx.get_loader::<DataLoader<UserLoader>>();
        let user = loader.load_one(self.user_id).await?;

        Ok(user)
    }
}
