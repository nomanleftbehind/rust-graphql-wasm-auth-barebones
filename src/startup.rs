use crate::configuration::{DatabaseSettings, Settings};
use crate::gql::{
    dataloaders::{PostLoader, UserLoader},
    QueryRoot, SchemaRoot,
};
use actix_cors::Cors;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{
    cookie::Key,
    dev::Server,
    get,
    middleware::Logger,
    route,
    web::{self, Data},
    App, HttpServer, Responder,
};
use actix_web_flash_messages::{storage::CookieMessageStore, FlashMessagesFramework};
use actix_web_lab::respond::Html;
use async_graphql::{
    dataloader::DataLoader,
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use secrecy::{ExposeSecret, Secret};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
// use tracing::log::LevelFilter;

#[derive(Clone)]
pub struct SessionCookieName(pub String);

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(schema: web::Data<SchemaRoot>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

/// GraphiQL playground UI
#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
    ))
}

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);
        // let email_client = configuration.email_client.client();

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            connection_pool,
            // email_client,
            configuration.application.base_url,
            configuration.application.hmac_secret,
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
    // email_client: EmailClient,
    base_url: String,
    hmac_secret: Secret<String>,
    redis_uri: Secret<String>,
) -> Result<Server, anyhow::Error> {
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    env_logger::init();

    // let configuration = get_configuration().expect("Failed to read configuration");

    // let mut options = PgConnectOptions::from_str(&configuration.database.connection)
    //     .expect("Failed to create SqlitePoolOptions");
    // options.log_statements(LevelFilter::Trace);
    // let pool = Arc::new(PgPool::connect_with(options)
    //     .await
    //     .expect("Postgres connection error"));
    // let pool = Data::new(
    //     PgPool::connect_with(options)
    //         .await
    //         .expect("Postgres connection error"),
    // );

    // sqlx::migrate!("./migrations")
    //     .run(&**pool)
    //     .await
    //     .expect("Migration error");

    let db_pool = Data::new(db_pool);
    // let email_client = Data::new(email_client);
    let base_url = Data::new(ApplicationBaseUrl(base_url));
    let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());
    let message_store = CookieMessageStore::builder(secret_key.clone()).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();
    let redis_store = RedisSessionStore::new(redis_uri.expose_secret()).await?;

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .extension(async_graphql::extensions::Tracing)
        .limit_complexity(1024)
        // .data(SessionCookieName(configuration.session_cookie_name.clone()))
        .data(DataLoader::new(
            PostLoader::new(db_pool.clone()),
            tokio::spawn,
        ))
        .data(DataLoader::new(
            UserLoader::new(db_pool.clone()),
            tokio::spawn,
        ))
        .data(db_pool.clone())
        // .data(email_client.clone())
        .data(base_url.clone())
        .data(Data::new(HmacSecret(hmac_secret.clone())))
        .finish();

    log::info!("starting HTTP server on port 8080");
    log::info!("GraphiQL playground: http://localhost:8080/graphiql");

    let server = HttpServer::new(move || {
        App::new()
            .wrap(message_framework.clone())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .app_data(web::Data::new(schema.clone()))
            .service(graphql)
            .service(graphql_playground)
            .wrap(Cors::permissive())
            .wrap(Logger::default())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

#[derive(Clone)]
pub struct HmacSecret(pub Secret<String>);
