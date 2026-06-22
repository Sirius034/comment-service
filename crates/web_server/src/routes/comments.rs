use crate::app::AppState;
use crate::handlers::comments::*;
use axum::{
    Router,
    routing::{delete, get, post, put},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/comments", post(get_comments))
        .route("/comments/{id}", get(get_comment))
        .route("/comments/create", post(create_comment))
        .route("/comments/update/{id}", put(update_comment))
        .route("/comments/delete/{id}", delete(delete_comment))
}
