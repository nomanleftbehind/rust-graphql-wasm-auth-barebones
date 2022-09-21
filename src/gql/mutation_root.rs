// use crate::authentication::{
//     cookies::sign_cookie_unchecked, register, session::SessionData, validate_credentials,
// };
use crate::authentication::validate_credentials;
use crate::authentication::Credentials;
use crate::gql::user::LoginUser;
use crate::gql::{context::ContextExt, user::User};
use crate::startup::{HmacSecret, SessionCookieName};
use actix_web::web::Data;
use async_graphql::*;
use cookie::Cookie;
use nanoid::nanoid;
use secrecy::{ExposeSecret, Secret};
use uuid::Uuid;

fn logged_in_err() -> Error {
    Error::new("Already logged in")
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    // async fn register(&self, ctx: &Context<'_>, email: String, password: String) -> Result<User> {
    //     let pool = ctx.db_pool();

    //     let mut transaction = pool.begin().await?;
    //     let result = register(&mut transaction, email, password).await?;
    //     transaction.commit().await?;
    //     Ok(result)
    // }

    async fn login(&self, ctx: &Context<'_>, login_user: LoginUser) -> Result<Uuid, Error> {
        let pool = ctx.db_pool();

        if ctx.get_cookie().is_ok() {
            return Err(logged_in_err());
        }

        let LoginUser { email, password } = login_user;

        let credentials = Credentials {
            email,
            password: Secret::new(password),
        };

        let user_id = validate_credentials(credentials, pool).await?;

        let session_manager = ctx.get_session_manager()?;

        // let user = login_user.login_user(ctx.db_pool, ctx.argon2).await?;

        session_manager
            .create_session(user_id)
            .await?
            .set_cookie(ctx)
            .await?;

        Ok(user_id)
    }

    // async fn login(&self, ctx: &Context<'_>, email: String, password: String) -> Result<bool> {
    //     let pool = ctx.db_pool();
    //     let key = ctx.data::<Data<HmacSecret>>().unwrap();
    //     let session_cookie_name = ctx.data::<Data<SessionCookieName>>().unwrap();

    //     if let Some(user_id) = validate_credentials(pool, email, Secret::new(password)).await {
    //         let session = SessionData {
    //             user_id,
    //             secret: nanoid!(),
    //         };
    //         let cookie = Cookie::build(
    //             (&***session_cookie_name).0.expose_secret().clone(),
    //             serde_json::to_string(&session)?,
    //         )
    //         .http_only(true)
    //         .secure(true)
    //         .same_site(cookie::SameSite::Strict)
    //         .finish();
    //         let cookie = sign_cookie_unchecked(cookie, (&***key).0.expose_secret().as_bytes());
    //         ctx.append_http_header("Set-Cookie", cookie.to_string());
    //         // insert_session(pool, session).await?;
    //         Ok(true)
    //     } else {
    //         Ok(false)
    //     }
    // }
}
