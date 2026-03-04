use crate::adapters::http::handlers::course_handler::*;
use crate::state::AppState;
use axum::{Router, routing::get};

pub fn course_router() -> Router<AppState> {
    Router::new().nest(
        "/courses",
        Router::new()
            .route("/", get(get_courses).post(create_course))
            .route(
                "/{id}",
                get(get_course_by_id)
                    .put(update_course)
                    .delete(delete_course),
            ),
    )
}
