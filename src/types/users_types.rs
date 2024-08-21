use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug)]
pub struct CreateUserRes {
    pub message: String,
    pub data: String,
}

impl IntoResponse for CreateUserRes {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::CREATED,
            Json(json!({ "name": self.data,"Email": self.message })),
        )
            .into_response()
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserReq {
    pub name: String,
    pub email: String,
    pub password: String,
}
