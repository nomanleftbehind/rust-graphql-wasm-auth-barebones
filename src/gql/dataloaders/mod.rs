use crate::gql::{dataloaders::loader_registry::LoaderRegistry, post::Post, user::User};
use actix_web::web::Data;
use async_graphql::{dataloader::*, Context, *};
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub mod loader_registry;

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

        Ok(user_posts)
    }
}

pub struct UserLoader {
    pool: Data<PgPool>,
}

impl UserLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UserLoader {
    type Value = User;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let users = sqlx::query_as::<_, User>(
            r#"SELECT id, email, password_hash, post_signature FROM "user" WHERE id = ANY($1)"#,
        )
        .bind(keys)
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|user| (user.id, user))
        .collect();

        Ok(users)
    }
}

// Sugar that helps make things neater and avoid errors that would only crop up at runtime.
pub trait ContextExt {
    fn get_loader<T: anymap::any::Any + Send + Sync>(&self) -> &T;
    fn db_pool(&self) -> &PgPool;
}

impl<'a> ContextExt for Context<'a> {
    fn get_loader<T: anymap::any::Any + Send + Sync>(&self) -> &T {
        self.data_unchecked::<Data<LoaderRegistry>>().get::<T>()
    }

    fn db_pool(&self) -> &PgPool {
        self.data_unchecked::<Data<PgPool>>()
    }
}
