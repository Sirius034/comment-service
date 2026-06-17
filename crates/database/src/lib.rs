use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod create;
pub mod delete;
pub mod models;
pub mod read;
pub mod schema;
pub mod update;
pub mod web_request;

#[must_use]
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|error| panic!("DATABASE_URL must be set. Error: {error}"));

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}
