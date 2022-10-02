use super::{
    sql::query_user,
    user::{User, UserBy},
};
use crate::graphql::sql::query_user_posts;
use crate::graphql::{context::ContextExt, post::Post};
use async_graphql::*;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn user(&self, ctx: &Context<'_>, by: UserBy) -> Result<Option<User>> {
        let pool = ctx.db_pool();
        query_user(pool, by).await.map_err(Error::from)
    }

    async fn user_posts(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 10)] limit: i64,
        #[graphql(default = 0)] offset: i64,
    ) -> Result<Vec<Post>> {
        let pool = ctx.db_pool();

        let cookie = ctx.get_cookie()?;
        let session_manager = ctx.get_session_manager()?;
        let user_id = session_manager.user_id(cookie).await?;

        let ass = query_user_posts(pool, user_id, limit, offset)
            .await
            .map_err(Error::from);

        ass
    }
}
