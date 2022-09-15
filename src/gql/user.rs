use crate::gql::dataloaders::PostLoader;
use async_graphql::dataloader::DataLoader;
use async_graphql::*;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::gql::post::Post;

use super::dataloaders::ContextExt;
// use crate::gql::sql::query_posts_by_user_id;
// use crate::core::session::UserCredential;

// use super::{
//     // post::Post,
//     session::Role,
//     sql::{
//         query_post_by_id, query_role, query_topic_by_id, query_user_post_ids, query_user_topic_ids,
//     },
//     // topic::Topic,
// };

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
    //     async fn role(&self, ctx: &Context<'_>) -> Result<Role> {
    //         let pool = ctx.data::<PgPool>().unwrap();
    //         query_role(pool, self.id)
    //             .await?
    //             .ok_or(Error::new("user does not exist"))
    //     }
    // #[graphql(complexity = "limit as usize * child_complexity")]
    // async fn topics(
    //     &self,
    //     ctx: &Context<'_>,
    //     #[graphql(default = 10)] limit: i64,
    //     #[graphql(default = 0)] offset: i64,
    // ) -> Result<Vec<Topic>> {
    //     let pool = ctx.data::<PgPool>().unwrap();
    //     let cred = ctx.data::<UserCredential>().unwrap();
    //     let topic_ids = query_user_topic_ids(pool, cred, self.id, limit, offset).await?;
    //     // N+1 query here
    //     let mut v = Vec::new();
    //     for id in topic_ids {
    //         if let Some(topic) = query_topic_by_id(pool, cred, id).await? {
    //             v.push(topic);
    //         }
    //     }
    //     Ok(v)
    // }
    // #[graphql(complexity = "limit as usize * child_complexity")]
    // async fn posts2(
    //     &self,
    //     ctx: &Context<'_>,
    //     // #[graphql(default = 10)] limit: i64,
    //     // #[graphql(default = 0)] offset: i64,
    // ) -> Result<Vec<Post>> {
    //     let my_uuid = Uuid::parse_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8")?;
    //     let pool = ctx.data::<Data<PgPool>>().unwrap();
    //     let posts = query_posts_by_user_id(pool, my_uuid).await?;

    //     Ok(posts)
    // }
    async fn posts(&self, ctx: &Context<'_>) -> Result<Vec<Post>> {
        let loader = ctx.get_loader::<DataLoader<PostLoader>>();
        let posts = loader.load_one(self.id).await?;
        let result = posts.ok_or_else(|| "Not found".into());

        result
    }
}
