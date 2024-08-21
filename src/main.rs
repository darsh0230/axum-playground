mod handlers;
mod models;
mod routes;
mod schema;
mod types;
mod utils;

use thiserror::Error;

use axum::{
    http::{header::CONTENT_TYPE, Method, StatusCode},
    middleware,
    response::IntoResponse,
    Extension, Router,
};

use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use dotenv::dotenv;
use tokio::net::TcpListener;

use routes::create_router;
use tower_http::cors::{Any, CorsLayer};

use sea_orm::Database;

// use sqlx::postgres::{PgPool, PgPoolOptions};

pub struct AppState {
    // db: PgPool,
}

// TODO: Setup caching

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    println!("🌟 API Service 🌟");

    let conn_str = (*utils::constants::DATABASE_URL).clone();
    let db = match Database::connect(conn_str).await {
        Ok(db) => {
            println!("✅ Connection to the database is successful!");
            db
        }
        Err(err) => {
            println!("❌ Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let app: Router = Router::new()
        .merge(create_router())
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .layer(Extension(db));

    println!("✅ Server started successfully at 0.0.0.0:5000");

    let listener = TcpListener::bind("0.0.0.0:5000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
