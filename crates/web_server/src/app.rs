use crate::routes::comments;
use axum::Router;
use database::{DbPool, establish_connection_manager};

#[derive(Clone)]
pub struct AppState {
    pub db_comments: DbPool,
}

pub fn create_app() -> Router {
    Router::new()
        .nest("/api", comments::routes())
        .with_state(AppState {
            db_comments: establish_connection_manager(),
        })
}
