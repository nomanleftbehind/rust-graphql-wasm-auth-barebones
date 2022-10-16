use crate::configuration::{DatabaseSettings, Settings};
use crate::graphql::{
    dataloaders::{get_loaders, LoaderRegistry},
    MutationRoot, QueryRoot,
};
use crate::routes::{graphql, graphql_playground};
use actix_cors::Cors;
use actix_web::{
    cookie::Key,
    dev::Server,
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use actix_web_flash_messages::{storage::CookieMessageStore, FlashMessagesFramework};
use async_graphql::{EmptySubscription, Schema};
use async_redis_session::RedisSessionStore;
use secrecy::{ExposeSecret, Secret};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
// use tracing::log::LevelFilter;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            connection_pool,
            configuration.application.base_url,
            configuration.application.hmac_secret,
            configuration.application.session_cookie_name,
            configuration.redis_uri,
        )
        .await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

pub struct ApplicationBaseUrl(pub String);

pub async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    base_url: String,
    hmac_secret: Secret<String>,
    session_cookie_name: Secret<String>,
    redis_uri: Secret<String>,
) -> Result<Server, anyhow::Error> {
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    env_logger::init();

    let db_pool = Data::new(db_pool);
    let loaders = get_loaders(db_pool.clone()).await;
    let loader_registry_data = Data::new(LoaderRegistry { loaders });

    let base_url = Data::new(ApplicationBaseUrl(base_url));
    let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());
    let message_store = CookieMessageStore::builder(secret_key.clone()).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();
    let redis_store = RedisSessionStore::new(redis_uri.expose_secret().as_str())
        .expect("Failed to connect to Redis");

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .extension(async_graphql::extensions::Tracing)
        .limit_complexity(1024)
        .data(loader_registry_data)
        .data(db_pool.clone())
        .data(base_url.clone())
        .data(Data::new(HmacSecret(hmac_secret.clone())))
        .data(Data::new(SessionCookieName(session_cookie_name.clone())))
        .data(redis_store)
        .finish();

    log::info!("starting HTTP server on port 8080");
    log::info!("GraphiQL playground: http://localhost:8080/graphiql");

    let server = HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(message_framework.clone())
            .app_data(web::Data::new(schema.clone()))
            .service(graphql)
            .service(graphql_playground)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

#[derive(Clone)]
pub struct HmacSecret(pub Secret<String>);

#[derive(Clone)]
pub struct SessionCookieName(pub Secret<String>);
