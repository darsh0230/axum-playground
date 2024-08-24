use axum::{http::status::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
pub struct APIError {
    pub status_code: StatusCode,
    pub error_message: String,
    pub user_message: String,
}

impl Default for APIError {
    fn default() -> APIError {
        APIError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_message: "Something really went wrong.".to_string(),
            user_message: "Something went wrong. Please try again after sometime.".to_string(),
        }
    }
}

impl IntoResponse for APIError {
    fn into_response(self) -> axum::response::Response {
        (
            self.status_code,
            Json(json!({"status_code":self.status_code.to_string(), "error_message": self.error_message, "user_message": self.user_message})),
        )
            .into_response()
    }
}
