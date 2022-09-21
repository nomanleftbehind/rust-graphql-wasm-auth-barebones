use crate::graphql::dataloaders::{PostLoader, UserLoader};
use actix_web::web::Data;
use anymap::{any::Any, Map};
use async_graphql::dataloader::DataLoader;
use sqlx::PgPool;

pub type LoaderMap = Map<AnyLoader>;
pub type AnyLoader = dyn Any + Send + Sync;

pub struct LoaderRegistry {
    pub loaders: LoaderMap,
}

impl LoaderRegistry {
    pub fn get<T: anymap::any::Any + Send + Sync>(&self) -> &T {
        match self.loaders.get::<T>() {
            Some(loader) => loader,
            None => unreachable!("{} not found", std::any::type_name::<T>()),
        }
    }
}

pub async fn get_loaders(pool: Data<PgPool>) -> LoaderMap {
    let mut loaders: LoaderMap = LoaderMap::new();

    let user_by_post_id_loader = DataLoader::new(UserLoader::new(pool.clone()), tokio::spawn);

    let posts_by_user_id_loader = DataLoader::new(PostLoader::new(pool.clone()), tokio::spawn);

    loaders.insert(user_by_post_id_loader);
    loaders.insert(posts_by_user_id_loader);

    loaders
}
