use crate::utils::SessionTokenExtractorError;
use actix_web::{dev::Payload, FromRequest, HttpRequest};
use async_graphql::{Context, Error};
use async_redis_session::RedisSessionStore;
use async_session::{Session, SessionStore};
use http::header::SET_COOKIE;
use std::future::{ready, Ready};

use super::AUTH_COOKIE_NAME;

#[derive(Debug)]
pub struct SessionCookie {
    pub value: String,
}

/// the main goal of this file is to create a cookie
/// cookies in this program help us to know if a user is logged in
/// and if the logged in user has admin priveleges or not

impl FromRequest for SessionCookie {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let session_cookie = req
            .cookie(AUTH_COOKIE_NAME)
            .map(|cookie| {
                ready(Ok(Self {
                    value: cookie.value().to_string(),
                }))
            })
            .unwrap_or_else(move || {
                let e = SessionTokenExtractorError::NoSessionToken;
                ready(Err(e.into()))
            });

        session_cookie
    }
}

impl SessionCookie {
    /// Uses GraphQL Context to set session cookie on the browser.
    pub async fn set_cookie(&self, ctx: &Context<'_>) -> Result<(), Error> {
        ctx.append_http_header(
            SET_COOKIE,
            format!("{}={}; SameSite=None; Secure", AUTH_COOKIE_NAME, self.value),
        );

        Ok(())
    }

    /// Load actual session from Redis/Session Store.
    pub async fn load_session(&self, session_store: &RedisSessionStore) -> Result<Session, Error> {
        session_store
            .load_session(self.value.clone())
            .await
            .map_err(|e| Error::new(e.to_string()))?
            .ok_or_else(|| Error::new("Session present but not found on Redis"))
    }
}
