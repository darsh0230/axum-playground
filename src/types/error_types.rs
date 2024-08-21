use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
pub struct APIError {
    pub error_message: String,
}

impl IntoResponse for APIError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error_message": self.error_message})),
        )
            .into_response()
    }
}
