use crate::error::AppError;
use axum::{
    Json, Router,
    extract::Path,
    routing::{delete, get, post, put},
};
use database::{create, delete as remove_comment, read, update};

async fn get_comments(
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let json = payload.to_string();
    let comments = read::web_get_comments(Some(&json)).map_err(AppError)?;

    Ok(Json(serde_json::json!(comments)))
}

async fn get_comment(Path(id): Path<String>) -> Json<serde_json::Value> {
    let comment = read::web_get_comment(&id).unwrap();
    Json(serde_json::json!(comment))
}

async fn create_comment(json: String) -> Result<Json<serde_json::Value>, AppError> {
    let comment = create::web_create_comment(&json)?;
    Ok(Json(serde_json::json!(comment)))
}

async fn update_comment(
    Path(id): Path<String>,
    json: String,
) -> Result<Json<serde_json::Value>, AppError> {
    let comment = update::web_update_comment(&id, &json)?;
    Ok(Json(serde_json::json!(comment)))
}

async fn delete_comment(Path(id): Path<String>) -> Result<String, AppError> {
    remove_comment::web_delete_comment(&id)?;

    Ok(id)
}

pub fn routes() -> Router {
    Router::new()
        .route("/comments", post(get_comments))
        .route("/comments/{id}", get(get_comment))
        .route("/comments/create", post(create_comment))
        .route("/comments/update/{id}", put(update_comment))
        .route("/comments/delete/{id}", delete(delete_comment))
}
