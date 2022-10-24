# Barebones Blog web app

Fullstack web app built 100% with Rust. GraphQL API, WebAssembly client and PostgreSQL.

When my Rust learning jurney first started I found it difficult to find begginer friendly examples or tutorials that implemented all the featues neccessary to build real world web application.

This example is simple enough for Rust begginers to understand and pick appart the parts they need. It's also rich with features that are regularly used in production like cookie authentication, several levels of user role authorization, and post change tracking.

Work in progress.

## Technology:

### Server
- [Rust](https://www.rust-lang.org/) programming language
- [Async-graphql](https://github.com/async-graphql/async-graphql) integration with [Actix](https://actix.rs/) web framework for Rust
- [SQLx](https://github.com/launchbadge/sqlx) async crate for querying databases featuring compile-time checked queries without a DSL

### Database
- [PostgreSQL](https://www.postgresql.org/) object-relational database

### Client
- [Yew](https://yew.rs/) Rust framework for creating multi-threaded front-end web apps using WebAssembly


## Setup

### Database

`cargo install sqlx-cli --no-default-features --features native-tls,postgres`

Rename `.env.example` file to `.env`. Inside, replace the word 'password' of DATABASE_URL with Postgres superuser password.

`sqlx database create`

`sqlx migrate run`

### Server

Rename `./server/configuration/base.example.yaml` file to `./server/configuration/base.yaml`. Inside, replace password with your Postgres superuser password.

Run:
```
cd server

cargo run
```

Open GraphQL playground at http://localhost:8080/graphiql in your Browser where you can test queries and mutations.

### Client

Install [Trunk](https://trunkrs.dev/), a WASM web application bundler for Rust, and [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/), a Rust library and CLI tool that facilitate high-level interactions between WASM modules and JavaScript by running `cargo install trunk wasm-bindgen-cli`.

Navigate to `./client/` folder with `cd client` in your terminal and run `trunk build`.

Run `trunk serve` to start the web app and open http://localhost:4001 in your browser.