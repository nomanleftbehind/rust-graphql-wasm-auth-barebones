use crate::gql::post::Post;
use actix_web::web::Data;
use async_graphql::dataloader::*;
use async_graphql::*;
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

/// Builds a list of UUIDs for use in a SQL query
// fn uuid_list(keys: &[Uuid]) -> String {
//     keys.iter()
//         .map(|key| format!("'{}'", key.to_string()))
//         .enumerate()
//         .fold(String::new(), |mut acc, (i, key)| {
//             if i == 0 {
//                 key.to_string()
//             } else {
//                 acc.push_str(", ");
//                 acc.push_str(key.as_str());
//                 acc
//             }
//         })
// }

pub struct PostLoader {
    pool: Data<PgPool>,
}

impl PostLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for PostLoader {
    type Value = Vec<Post>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        // let key_list = uuid_list(keys);
        // let query = format!("SELECT * FROM post WHERE user_id IN ({})", key_list);

        let mut posts = sqlx::query_as::<_, Post>("SELECT * FROM post WHERE user_id = ANY($1)")
            .bind(keys)
            .fetch_all(&**self.pool)
            .await?;
        posts.sort_by_key(|post| post.user_id);

        let user_posts: HashMap<Uuid, Self::Value> = posts
            .iter()
            .group_by(|post| post.user_id)
            .into_iter()
            .map(|(user_id, group)| (user_id, group.cloned().collect()))
            .collect();

        Ok(user_posts)
    }
}
