use crate::graphql::post::Post;
use crate::graphql::user::{User, UserBy};
use sqlx::PgExecutor;
use uuid::Uuid;

pub async fn query_user<'e, E: PgExecutor<'e>>(
    executor: E,
    by: UserBy,
) -> Result<Option<User>, sqlx::Error> {
    let user =
        match by {
            UserBy::Id(id) => {
                sqlx::query_as!(
                    User,
                    r#"SELECT id, email, password, first_name, last_name FROM "users" WHERE id = $1"#,
                    id
                )
                .fetch_optional(executor)
                .await?
            }
            UserBy::Email(email) => sqlx::query_as!(
                User,
                r#"SELECT id, email, password, first_name, last_name FROM "users" WHERE email = $1"#,
                email
            )
            .fetch_optional(executor)
            .await?,
        };
    Ok(user)
}

pub async fn query_user_posts<'e, E: PgExecutor<'e>>(
    executor: E,
    user_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<Post>, sqlx::Error> {
    let posts = sqlx::query_as!(
        Post,
        r#"SELECT p.* FROM "posts" p WHERE p."created_by_id" = $1 LIMIT $2 OFFSET $3"#,
        user_id,
        limit,
        offset
    )
    .fetch_all(executor)
    .await;

    posts
    // posts
}

pub async fn query_user_by_id<'e, E: PgExecutor<'e>>(
    executor: E,
    user_id: Uuid,
) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"SELECT id, email, password, first_name, last_name FROM users WHERE id = $1"#,
        user_id
    )
    .fetch_optional(executor)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(user)
}

pub async fn query_all_users<'e, E: PgExecutor<'e>>(executor: E) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(
        User,
        r#"SELECT id, email, password, first_name, last_name FROM users"#,
    )
    .fetch_all(executor)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    });

    users
}

// pub async fn query_user_post_ids<'e, E: PgExecutor<'e>>(
//     executor: E,
//     // cred: &UserCredential,
//     user_id: Uuid,
//     limit: i64,
//     offset: i64,
// ) -> Result<Vec<i64>, sqlx::Error> {
//     query(include_str!("user_posts.sql"))
//         // .bind(cred.user_id())
//         .bind(user_id)
//         .bind(limit)
//         .bind(offset)
//         .map(|row: PgRow| row.get("id"))
//         .fetch_all(executor)
//         .await
// }

// pub async fn query_posts_by_user_id(
//     pool: &PgPool,
//     user_id: Uuid,
// ) -> Result<Vec<Post>, sqlx::Error> {
//     let posts = sqlx::query_as!(
//         Post,
//         r#"
//         SELECT * FROM post
//         WHERE
//         user_id = $1
//         "#,
//         user_id
//     )
//     .fetch_all(pool)
//     .await?;
//     Ok(posts)
// }

// pub async fn query_role<'e, E: PgExecutor<'e>>(
//     _pool: E,
//     user_id: i64,
// ) -> Result<Option<Role>, sqlx::Error> {
//     if user_id == 1 {
//         return Ok(Some(Role::Administrator));
//     }
//     Ok(Some(Role::Regular))
// }
