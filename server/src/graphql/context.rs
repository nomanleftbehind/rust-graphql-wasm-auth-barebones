use crate::graphql::dataloaders::LoaderRegistry;
use crate::authentication::{cookie::SessionCookie, SessionManager};
use actix_web::web::Data;
use async_graphql::{Context, Error};
use async_redis_session::RedisSessionStore;
use sqlx::PgPool;

// Sugar that helps make things neater and avoid errors that would only crop up at runtime.
pub trait ContextExt {
    fn get_loader<T: anymap::any::Any + Send + Sync>(&self) -> &T;
    fn db_pool(&self) -> &PgPool;
    fn get_cookie(&self) -> Result<&SessionCookie, Error>;
    fn get_session_manager(&self) -> Result<SessionManager, Error>;
}

impl<'a> ContextExt for Context<'a> {
    fn get_loader<T: anymap::any::Any + Send + Sync>(&self) -> &T {
        self.data_unchecked::<Data<LoaderRegistry>>().get::<T>()
    }

    fn db_pool(&self) -> &PgPool {
        self.data_unchecked::<Data<PgPool>>()
    }

    /// Gets the SessionCookie or errors if no cookie is found.
    fn get_cookie(&self) -> Result<&SessionCookie, Error> {
        let session_cookie = self
            .data::<Option<SessionCookie>>()
            .expect("Auth Cookie Option not found in Context");

        session_cookie
            .as_ref()
            .ok_or_else(|| Error::new("Not logged in"))
    }

    fn get_session_manager(&self) -> Result<SessionManager, Error> {
        let store = self
            .data::<RedisSessionStore>()
            .expect("Session store not found in Context");
        let authorizer = SessionManager::new(store);
        Ok(authorizer)
    }
}
