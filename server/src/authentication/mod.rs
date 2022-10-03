pub mod cookie;
mod manager;
mod passwords;

pub use manager::*;
pub use passwords::{register, validate_credentials, Credentials};

pub const AUTH_COOKIE_NAME: &str = "auth";
pub const USER_ID_SESSION_KEY: &str = "user_id";
