use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;
use std::env;

pub mod create;
pub mod delete;
pub mod errors;
pub mod models;
pub mod read;
pub mod schema;
pub mod update;
pub mod web_request;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type R2D2PoolError = r2d2::PoolError;

fn get_db_url() -> String {
    dotenv().ok();

    env::var("DATABASE_URL")
        .unwrap_or_else(|error| panic!("DATABASE_URL must be set. Error: {error}"))
}

#[must_use]
pub fn establish_connection() -> PgConnection {
    let database_url = get_db_url();

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}

pub fn establish_connection_manager() -> DbPool {
    let database_url = get_db_url();

    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    r2d2::Pool::builder()
        .max_size(20)
        .build(manager)
        .unwrap_or_else(|_| panic!("Error establish connection manager to {database_url}"))
}
