mod adapters;
mod infrastructure;
mod state;

use adapters::http::routes::{user_route::user_router};
use axum::{Router, routing::get};
use infrastructure::{config::AppConfig, database::create_pool};
use state::AppState;
use std::net::SocketAddr;

async fn health_check() -> &'static str {
    "All good"
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = AppConfig::from_env();

    let pool = create_pool(&config.database_url)
        .await
        .expect("Failed to create database pool");

    let state = AppState { db: pool };

    let app = Router::new()
        .route("/health", get(health_check))
        .merge(user_router())
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app)
        .await
        .unwrap();

    Ok(())
}
