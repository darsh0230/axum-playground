use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::{auth_handler, health_handler::health_check_handler};

pub fn create_router() -> Router {
    Router::new()
        .route("/api/healthcheck", get(health_check_handler))
        .route("/api/auth/register", post(auth_handler::user_register))
        .route("/api/auth/login", post(auth_handler::user_login))
}
