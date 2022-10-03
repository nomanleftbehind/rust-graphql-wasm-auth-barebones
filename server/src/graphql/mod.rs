use async_graphql::{EmptySubscription, Schema};

pub mod mutation_root;
pub mod query_root;

pub mod context;
pub mod dataloaders;
pub mod post;
pub mod sql;
pub mod user;

pub use mutation_root::MutationRoot;
pub use query_root::QueryRoot;

pub type SchemaRoot = Schema<QueryRoot, MutationRoot, EmptySubscription>;
