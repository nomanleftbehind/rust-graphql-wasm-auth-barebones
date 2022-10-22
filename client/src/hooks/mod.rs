use graphql_client::GraphQLQuery;
mod lazy_function;
mod use_query;

pub use lazy_function::lazy_function;
pub use use_query::use_query;
// mod use_user_context;

// pub use use_user_context::*;

type UUID = String;
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/queries.graphql",
    response_derives = "Debug, Clone, PartialEq"
)]
pub struct AllUsers;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/mutations.graphql",
    response_derives = "Debug, Clone, PartialEq"
)]
#[derive(Debug)]
pub struct LoginUser;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/queries.graphql",
    response_derives = "Debug, Clone, PartialEq, Eq, Serialize"
)]
#[derive(Debug)]
pub struct Me;
