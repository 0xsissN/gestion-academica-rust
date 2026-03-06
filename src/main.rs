mod adapters;
mod auth;
mod domain;
mod errors;
mod infrastructure;
mod state;

use adapters::http::routes::{
    auth_route::auth_router, career_route::career_router, course_route::course_router,
    enrollment_route::enrollment_router, user_route::user_router,
};
use axum::{Router, routing::get};
use infrastructure::{config::AppConfig, database::create_pool};
use state::AppState;
use std::net::SocketAddr;

async fn health_check() -> &'static str {
    "All good"
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = AppConfig::from_env();

    let pool = create_pool(&config.database_url).await?;

    let state = AppState { db: pool };

    let app = Router::new()
        .route("/health", get(health_check))
        .merge(auth_router())
        .merge(user_router())
        .merge(career_router())
        .merge(course_router())
        .merge(enrollment_router())
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}
