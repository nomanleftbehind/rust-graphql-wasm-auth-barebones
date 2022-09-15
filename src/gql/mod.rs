use async_graphql::{EmptyMutation, EmptySubscription, Schema};

pub mod context;
pub mod dataloaders;
pub mod post;
pub mod query_root;
pub mod session;
pub mod sql;
pub mod user;

pub use query_root::QueryRoot;

pub type SchemaRoot = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
