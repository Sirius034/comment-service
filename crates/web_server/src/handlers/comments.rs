use crate::app::AppState;
use crate::errors::AppErrors;
use axum::{
    Json,
    extract::{Path, State},
};
use database::{create, delete as remove_comment, models::Comment, read, update};

pub async fn get_comments(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppErrors> {
    let comments = tokio::task::spawn_blocking(move || -> Result<Vec<Comment>, AppErrors> {
        let mut connection = state.db_comments.get()?;
        let json = payload.to_string();
        let comments = read::web_get_comments(Some(&json), &mut connection)?;

        Ok(comments)
    })
    .await??;

    Ok(Json(serde_json::json!(comments)))
}

pub async fn get_comment(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppErrors> {
    let comment = tokio::task::spawn_blocking(move || -> Result<Comment, AppErrors> {
        let mut connection = state.db_comments.get()?;
        let comment = read::web_get_comment(&id, &mut connection)?;
        Ok(comment)
    })
    .await??;

    Ok(Json(serde_json::json!(comment)))
}

pub async fn create_comment(
    State(state): State<AppState>,
    json: String,
) -> Result<Json<serde_json::Value>, AppErrors> {
    let comment = tokio::task::spawn_blocking(move || -> Result<Comment, AppErrors> {
        let mut connection = state.db_comments.get()?;
        let comment = create::web_create_comment(&json, &mut connection)?;

        Ok(comment)
    })
    .await??;

    Ok(Json(serde_json::json!(comment)))
}

pub async fn update_comment(
    State(state): State<AppState>,
    Path(id): Path<String>,
    json: String,
) -> Result<Json<serde_json::Value>, AppErrors> {
    let comment = tokio::task::spawn_blocking(move || -> Result<Comment, AppErrors> {
        let mut connection = state.db_comments.get()?;
        let comment = update::web_update_comment(&id, &json, &mut connection)?;
        Ok(comment)
    })
    .await??;

    Ok(Json(serde_json::json!(comment)))
}

pub async fn delete_comment(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<String, AppErrors> {
    let id = tokio::task::spawn_blocking(move || -> Result<String, AppErrors> {
        let mut connection = state.db_comments.get()?;
        let id = remove_comment::web_delete_comment(&id, &mut connection)?;
        Ok(id.to_string())
    })
    .await??;

    Ok(id)
}
