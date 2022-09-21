# Barebones GraphQL in Rust

Barebones Rust, GraphQL, Cookie authentication, Yew frontend project.

# Technology stack:
 - [Async-graphql](https://github.com/async-graphql/async-graphql) integration with [actix-web](https://actix.rs/)
 - [PostgreSQL](https://www.postgresql.org/) relational database
 - [SQLx](https://github.com/launchbadge/sqlx) async crate for querying databases featuring compile-time checked queries without a DSL



### Install SQLx CLI
`cargo install sqlx-cli --no-default-features --features native-tls,postgres`

### Create/drop the database at DATABASE_URL
```
sqlx database create
sqlx database drop
```

### Create and run migrations
`sqlx migrate add <name>`