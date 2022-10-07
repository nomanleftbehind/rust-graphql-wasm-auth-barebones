use crate::graphql::post::Post;
use actix_web::web::Data;
use async_graphql::{dataloader::*, *};
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

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
        let mut posts = sqlx::query_as::<_, Post>("SELECT * FROM post WHERE user_id = ANY($1)")
            .bind(keys)
            .fetch_all(&**self.pool)
            .await?;
        posts.sort_by_key(|post| post.user_id);

        let user_posts = posts
            .into_iter()
            .group_by(|post| post.user_id)
            .into_iter()
            .map(|(user_id, group)| (user_id, group.collect()))
            .collect();

        // println!("{:?}", &user_posts);
        Ok(user_posts)
    }
}