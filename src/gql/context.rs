use crate::gql::dataloaders::LoaderRegistry;
use actix_web::web::Data;
use async_graphql::Context;
use sqlx::PgPool;

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
