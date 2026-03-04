use crate::adapters::http::handlers::user_handler::*;
use crate::state::AppState;
use axum::{Router, routing::get};

pub fn user_router() -> Router<AppState> {
    Router::new().nest(
        "/users",
        Router::new()
            .route("/", get(get_users).post(create_user))
            .route(
                "/{id}",
                get(get_user_by_id).put(update_user).delete(delete_user),
            ),
    )
}
