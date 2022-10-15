use crate::authentication::AUTH_COOKIE_NAME;
use crate::authentication::{register, validate_credentials, Credentials};
use crate::graphql::{
    context::ContextExt,
    user::{LoginUser, User},
};
use ::http::header::SET_COOKIE;
use async_graphql::*;
use secrecy::Secret;
use uuid::Uuid;

fn logged_in_err() -> Error {
    Error::new("Already logged in")
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn register(&self, ctx: &Context<'_>, email: String, password: String) -> Result<User> {
        let pool = ctx.db_pool();

        let result = register(pool, email, password).await?;
        Ok(result)
    }

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

        session_manager
            .create_session(user_id)
            .await?
            .set_cookie(ctx)
            .await?;

        // let session_manager2 = ctx.get_session_manager()?;
        // println!("login user: {:?}", &session_manager2);

        Ok(user_id)
    }

    async fn logout(&self, ctx: &Context<'_>) -> Result<bool, Error> {
        ctx.insert_http_header(
            SET_COOKIE,
            format!("{}=deleted; Max-Age=-1", AUTH_COOKIE_NAME),
        );
        Ok(true)
    }
}
