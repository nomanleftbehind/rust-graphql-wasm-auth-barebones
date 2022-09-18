use crate::authentication::{
    cookies::sign_cookie_unchecked, register, session::SessionData, validate_credentials,
};
use crate::gql::{context::ContextExt, user::User};
use crate::startup::{HmacSecret, SessionCookieName};
use actix_web::web::Data;
use async_graphql::*;
use cookie::Cookie;
use nanoid::nanoid;
use secrecy::{ExposeSecret, Secret};

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn register(&self, ctx: &Context<'_>, email: String, password: String) -> Result<User> {
        let pool = ctx.db_pool();

        let mut transaction = pool.begin().await?;
        let result = register(&mut transaction, email, password).await?;
        transaction.commit().await?;
        Ok(result)
    }

    async fn login(&self, ctx: &Context<'_>, email: String, password: String) -> Result<bool> {
        let pool = ctx.db_pool();
        let key = ctx.data::<Data<HmacSecret>>().unwrap();
        let session_cookie_name = ctx.data::<Data<SessionCookieName>>().unwrap();

        if let Some(user_id) = validate_credentials(pool, email, Secret::new(password)).await {
            let session = SessionData {
                user_id,
                secret: nanoid!(),
            };
            let cookie = Cookie::build(
                (&***session_cookie_name).0.expose_secret().clone(),
                serde_json::to_string(&session)?,
            )
            .http_only(true)
            .secure(true)
            .same_site(cookie::SameSite::Strict)
            .finish();
            let cookie = sign_cookie_unchecked(cookie, (&***key).0.expose_secret().as_bytes());
            ctx.append_http_header("Set-Cookie", cookie.to_string());
            // insert_session(pool, session).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
