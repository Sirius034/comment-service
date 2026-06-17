use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use database::web_request::ErrorRequest;

pub struct AppError(pub ErrorRequest);

impl From<ErrorRequest> for AppError {
    fn from(e: ErrorRequest) -> Self {
        AppError(e)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self.0 {
            ErrorRequest::InvalidUuid { .. } | ErrorRequest::Database(_) => {
                (StatusCode::BAD_REQUEST, self.0.to_string())
            }
            ErrorRequest::InvalidRequest(error) => (StatusCode::BAD_REQUEST, error.to_string()),
        };

        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
