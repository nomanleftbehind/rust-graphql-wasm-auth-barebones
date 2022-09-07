use async_graphql::*;
use sqlx::{postgres::PgRow, prelude::*, types::time::PrimitiveDateTime /* , PgPool */};
use uuid::Uuid;

// use crate::core::session::UserCredential;

use super::user::User;

#[derive(SimpleObject, Clone)]
// #[graphql(complex)]
pub struct Post {
    pub id: Uuid,
    pub user_id: Uuid,
    pub body: String,
    pub topic: String,
    pub rank: Option<i32>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

impl<'r> FromRow<'r, PgRow> for Post {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            body: row.try_get("body")?,
            topic: row.try_get("topic")?,
            rank: row.try_get("rank").ok(),
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

// #[ComplexObject]
// impl Post {
//     async fn topic(&self, ctx: &Context<'_>) -> Result<Option<Topic>> {
//         let pool = ctx.data::<PgPool>().unwrap();
//         let cred = ctx.data::<UserCredential>().unwrap();
//         Ok(query_topic_by_id(pool, cred, self.topic_id).await?)
//     }
// }

#[derive(SimpleObject, Debug, Clone)]
pub struct PostMeta {
    pub author: User,
}

impl<'r> FromRow<'r, PgRow> for PostMeta {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let id: Option<Uuid> = row.try_get("user_id")?;
        let email = row.try_get("email")?;
        let password_hash = row.try_get("password_hash")?;
        let post_signature = row.try_get("post_signature")?;
        Ok(Self {
            author: User {
                id: id.ok_or(sqlx::Error::RowNotFound)?,
                email,
                password_hash,
                post_signature,
            },
        })
    }
}

#[derive(SimpleObject, Debug, Clone)]
pub struct PostContent {
    pub body: String,
}

impl<'r> FromRow<'r, PgRow> for PostContent {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let body: Option<String> = row.try_get("body")?;
        let f = || -> Option<Self> { Some(Self { body: body? }) };
        f().ok_or(sqlx::Error::RowNotFound)
    }
}
