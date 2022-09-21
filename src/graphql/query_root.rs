use super::{
    sql::query_user,
    user::{User, UserBy},
};
use crate::graphql::context::ContextExt;
use async_graphql::*;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn user(&self, ctx: &Context<'_>, by: UserBy) -> Result<Option<User>> {
        let pool = ctx.db_pool();
        query_user(pool, by).await.map_err(Error::from)
    }
}
