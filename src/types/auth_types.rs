use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct UserRegisterReq {
    pub name: String,
    pub email: String,
    pub password: String,
}
#[derive(Debug)]
pub struct UserRegisterRes {
    pub name: String,
    pub email: String,
}

impl IntoResponse for UserRegisterRes {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::CREATED,
            Json(json!({ "name": self.name,"email": self.email })),
        )
            .into_response()
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserLoginReq {
    pub email: String,
    pub password: String,
}
#[derive(Debug)]
pub struct UserLoginRes {
    pub token: String,
}

impl IntoResponse for UserLoginRes {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(json!({ "token": self.token }))).into_response()
    }
}
