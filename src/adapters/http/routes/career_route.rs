use crate::adapters::http::handlers::career_handler::*;
use crate::state::AppState;
use axum::{Router, routing::get};

pub fn career_router() -> Router<AppState> {
    Router::new().nest(
        "/careers",
        Router::new()
            .route("/", get(get_careers).post(create_career))
            .route(
                "/{id}",
                get(get_career_by_id)
                    .put(update_career)
                    .delete(delete_career),
            ),
    )
}
