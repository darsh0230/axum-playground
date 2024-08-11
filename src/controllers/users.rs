use std::sync::Arc;

use crate::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub async fn create_user(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    Ok((
        StatusCode::CREATED,
        Json(json!({"status": "success", "message": "User created"})),
    ))
}
