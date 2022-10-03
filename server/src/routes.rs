use crate::authentication::cookie::SessionCookie;
use crate::graphql::SchemaRoot;
use actix_web::{get, route, web::Data, Responder};
use actix_web_lab::respond::Html;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

/// GraphiQL playground UI
#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
    ))
}

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(
    schema: Data<SchemaRoot>,
    req: GraphQLRequest,
    auth_cookies: Option<SessionCookie>,
) -> GraphQLResponse {
    schema
        .execute(req.into_inner().data(auth_cookies))
        .await
        .into()
}
