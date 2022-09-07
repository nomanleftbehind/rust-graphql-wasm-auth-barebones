use sqlx::postgres::PgRow;

use async_graphql::*;
use sqlx::{FromRow, Row};

use super::user::User;

#[derive(SimpleObject)]
pub struct Session {
    pub user: User,
    // pub role: Role,
}

#[derive(Enum, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Role {
    Administrator,
    Regular,
}

impl<'r> FromRow<'r, PgRow> for Role {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(match row.try_get("role")? {
            "ADMINISTRATOR" => Self::Administrator,
            _ => Self::Regular,
        })
    }
}
