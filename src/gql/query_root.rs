use super::{
    sql::query_user,
    user::{User, UserBy},
};
use crate::gql::context::ContextExt;
use async_graphql::*;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn user(&self, ctx: &Context<'_>, by: UserBy) -> Result<Option<User>> {
        let pool = ctx.db_pool();
        query_user(pool, by).await.map_err(Error::from)
    }

    // async fn topic(&self, ctx: &Context<'_>, topic_id: i64) -> Result<Option<topic::Topic>> {
    //     let pool = ctx.data::<PgPool>().unwrap();
    //     let cred = ctx.data::<UserCredential>().unwrap();

    //     query_topic_by_id(pool, cred, topic_id)
    //         .await
    //         .map_err(Error::from)
    // }
}
