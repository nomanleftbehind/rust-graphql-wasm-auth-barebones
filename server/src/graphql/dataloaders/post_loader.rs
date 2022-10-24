use crate::graphql::post::Post;
use actix_web::web::Data;
use async_graphql::{dataloader::*, *};
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub struct CreatedPostsLoader {
    pool: Data<PgPool>,
}

impl CreatedPostsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedPostsLoader {
    type Value = Vec<Post>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut posts =
            sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE created_by_id = ANY($1)")
                .bind(keys)
                .fetch_all(&**self.pool)
                .await?;
        posts.sort_by_key(|post| post.created_by_id);

        let created_posts = posts
            .into_iter()
            .group_by(|post| post.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        // println!("{:?}", &user_posts);
        Ok(created_posts)
    }
}

pub struct UpdatedPostsLoader {
    pool: Data<PgPool>,
}

impl UpdatedPostsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedPostsLoader {
    type Value = Vec<Post>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut posts =
            sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE updated_by_id = ANY($1)")
                .bind(keys)
                .fetch_all(&**self.pool)
                .await?;
        posts.sort_by_key(|post| post.created_by_id);

        let updated_posts = posts
            .into_iter()
            .group_by(|post| post.created_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(updated_posts)
    }
}
