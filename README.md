# Barebones GraphQL in Rust

Barebones Rust, GraphQL, Cookie authentication, Yew frontend project.

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

### Server

`cargo install sqlx-cli --no-default-features --features native-tls,postgres`

Rename `server/.env.example` file to `server/.env`. Inside, replace the word 'password' of DATABASE_URL with Postgres superuser password.

`sqlx database create`

`sqlx migrate run`



### Client

Install [Trunk](https://trunkrs.dev/), a WASM web application bundler for Rust, and [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/), a Rust library and CLI tool that facilitate high-level interactions between WASM modules and JavaScript by running `cargo install trunk wasm-bindgen-cli`.

Navigate to `./frontend/` folder in your terminal and run `trunk build`.

Run `trunk serve` to start the web app and open http://localhost:4001 in your browser.