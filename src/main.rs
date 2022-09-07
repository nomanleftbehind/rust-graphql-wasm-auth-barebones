use emissionsapp::gql::dataloaders::PostLoader;
use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{get, middleware::Logger, route, web, App, HttpServer, Responder};
use actix_web_lab::respond::Html;
use async_graphql::dataloader::DataLoader;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use sqlx::{postgres::PgConnectOptions, ConnectOptions, PgPool};
use std::str::FromStr;

use emissionsapp::gql::{QueryRoot, SchemaRoot};
use emissionsapp::configuration::get_configuration;
use tracing::log::LevelFilter;

#[derive(Clone)]
pub struct HmacSecret(pub String);

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

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    env_logger::init();

    let configuration = get_configuration().expect("Failed to read configuration");

    let mut options = PgConnectOptions::from_str(&configuration.database.connection)
        .expect("Failed to create SqlitePoolOptions");
    options.log_statements(LevelFilter::Trace);
    // let pool = Arc::new(PgPool::connect_with(options)
    //     .await
    //     .expect("Postgres connection error"));
    let pool = Data::new(
        PgPool::connect_with(options)
            .await
            .expect("Postgres connection error"),
    );

    sqlx::migrate!("./migrations")
        .run(&**pool)
        .await
        .expect("Migration error");

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .extension(async_graphql::extensions::Tracing)
        .limit_complexity(1024)
        .data(HmacSecret(configuration.hmac_secret.clone()))
        .data(SessionCookieName(configuration.session_cookie_name.clone()))
        .data(DataLoader::new(PostLoader::new(pool.clone()), tokio::spawn))
        .data(pool.clone())
        .finish();

    log::info!("starting HTTP server on port 8080");
    log::info!("GraphiQL playground: http://localhost:8080/graphiql");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(graphql)
            .service(graphql_playground)
            .wrap(Cors::permissive())
            .wrap(Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
