use crate::adapters::http::handlers::enrollment_handler::*;
use crate::state::AppState;
use axum::{Router, routing::get};

pub fn enrollment_router() -> Router<AppState> {
    Router::new().nest(
        "/enrollments",
        Router::new()
            .route("/", get(get_enrollments).post(create_enrollment))
            .route(
                "/{id}",
                get(get_enrollment_by_id)
                    .put(update_enrollment)
                    .delete(delete_enrollment),
            ),
    )
}
