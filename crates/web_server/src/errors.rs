use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use database::R2D2PoolError;
use database::errors::Errors as DbErrors;
use tokio::task::JoinError;

pub enum AppErrors {
    Database(DbErrors),
    Join(JoinError),
}

impl From<DbErrors> for AppErrors {
    fn from(error: DbErrors) -> Self {
        AppErrors::Database(error)
    }
}

impl From<R2D2PoolError> for AppErrors {
    fn from(error: R2D2PoolError) -> Self {
        AppErrors::Database(DbErrors::Pool(error))
    }
}

impl From<JoinError> for AppErrors {
    fn from(error: JoinError) -> Self {
        AppErrors::Join(error)
    }
}

impl IntoResponse for AppErrors {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppErrors::Database(error) => match error {
                DbErrors::InvalidUuid { .. } | DbErrors::Database(_) => {
                    (StatusCode::BAD_REQUEST, error.to_string())
                }
                DbErrors::InvalidRequest(error) => (StatusCode::BAD_REQUEST, error.to_string()),
                DbErrors::Pool(error) => (StatusCode::BAD_REQUEST, error.to_string()),
            },
            AppErrors::Join(error) => (StatusCode::BAD_REQUEST, error.to_string()),
        };

        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
