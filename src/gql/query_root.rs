use super::{
    // session::Session,
    sql::query_user,
    // topic,
    user::{User, UserBy},
};
use actix_web::web::Data;
use async_graphql::*;
use sqlx::PgPool;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    // async fn session(&self, ctx: &Context<'_>) -> Result<Option<Session>> {
    //     let pool = ctx.data::<PgPool>().unwrap();
    //     let cred = ctx.data::<UserCredential>().unwrap();
    //     if let Some(id) = cred.user_id() {
    //         let user = query_user(pool, cred, UserBy::Id(id)).await?;
    //         let f = || Some(Session { user: user? });
    //         Ok(f())
    //     } else {
    //         Ok(None)
    //     }
    // }
    async fn user(&self, ctx: &Context<'_>, by: UserBy) -> Result<Option<User>> {
        let pool = ctx.data::<Data<PgPool>>().unwrap();
        // let cred = ctx.data::<UserCredential>().unwrap();
        query_user(&***pool, /* cred, */ by)
            .await
            .map_err(Error::from)
    }
    // #[graphql(complexity = "limit as usize * child_complexity")]
    // async fn board_topics(
    //     &self,
    //     ctx: &Context<'_>,
    //     #[graphql(default = 10)] limit: i64,
    //     #[graphql(default = 0)] offset: i64,
    // ) -> Result<Vec<topic::Topic>> {
    //     let pool = ctx.data::<PgPool>().unwrap();
    //     let cred = ctx.data::<UserCredential>().unwrap();
    //     let topic_ids = query_board_topic_ids(pool, cred, limit, offset).await?;
    //     // N+1 query here
    //     let mut v = Vec::new();
    //     for id in topic_ids {
    //         if let Some(topic) = query_topic_by_id(pool, cred, id).await? {
    //             v.push(topic);
    //         }
    //     }
    //     Ok(v)
    // }

    // async fn topic(&self, ctx: &Context<'_>, topic_id: i64) -> Result<Option<topic::Topic>> {
    //     let pool = ctx.data::<PgPool>().unwrap();
    //     let cred = ctx.data::<UserCredential>().unwrap();

    //     query_topic_by_id(pool, cred, topic_id)
    //         .await
    //         .map_err(Error::from)
    // }
}
