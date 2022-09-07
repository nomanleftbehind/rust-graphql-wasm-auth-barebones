### Install SQLx CLI
`cargo install sqlx-cli --no-default-features --features native-tls,postgres`

### Create/drop the database at DATABASE_URL
```
sqlx database create
sqlx database drop
```

### Create and run migrations
`sqlx migrate add <name>`