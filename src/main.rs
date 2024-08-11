mod controllers;
mod model;
mod routes;
mod schema;

use std::sync::Arc;

use axum::http::{header::CONTENT_TYPE, Method};

use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use dotenv::dotenv;
use tokio::net::TcpListener;

use routes::create_router;
use tower_http::cors::{Any, CorsLayer};

use sqlx::postgres::{PgPool, PgPoolOptions};

pub struct AppState {
    db: PgPool,
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

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
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

    let app = create_router(Arc::new(AppState { db: pool.clone() }))
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    println!("✅ Server started successfully at 0.0.0.0:5000");

    let listener = TcpListener::bind("0.0.0.0:5000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
