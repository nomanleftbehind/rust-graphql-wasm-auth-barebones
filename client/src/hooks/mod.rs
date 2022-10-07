use graphql_client::GraphQLQuery;
// mod use_user_context;

// pub use use_user_context::*;

pub mod use_query;

type UUID = String;
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/all_users.graphql",
    response_derives = "Debug, Clone, PartialEq"
)]
pub struct AllUsers;
