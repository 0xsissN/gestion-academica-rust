use crate::adapters::http::handlers::auth_handler::*;
use crate::state::AppState;
use axum::{Router, routing::post};

pub fn auth_router() -> Router<AppState> {
    Router::new().nest("/login", Router::new().route("/", post(login)))
}
