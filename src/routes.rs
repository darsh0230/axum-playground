use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::handlers::{auth_handler, health_handler::health_check_handler};
use crate::utils::guards::jwt_guard;

fn guarded_routes() -> Router {
    Router::new()
}

fn unguarded_routes() -> Router {
    Router::new()
        .route("/api/health", get(health_check_handler))
        .route("/api/auth/register", post(auth_handler::user_register))
        .route("/api/auth/login", post(auth_handler::user_login))
}

pub fn create_router() -> Router {
    Router::new()
        .merge(guarded_routes())
        .layer(middleware::from_fn(jwt_guard))
        .merge(unguarded_routes())
}
