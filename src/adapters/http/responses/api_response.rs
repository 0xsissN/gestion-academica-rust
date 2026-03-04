use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    #[serde(skip)]
    pub status: StatusCode,

    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let status = self.status;
        (status, Json(self)).into_response()
    }
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            status: StatusCode::OK,
            success: true,
            data: Some(data),
            message: None,
        }
    }

    pub fn created(data: T) -> Self {
        Self {
            status: StatusCode::CREATED,
            success: true,
            data: Some(data),
            message: None,
        }
    }

    pub fn no_content() -> Self {
        Self {
            status: StatusCode::NO_CONTENT,
            success: true,
            data: None,
            message: None,
        }
    }

    pub fn error(message: &str, status: StatusCode) -> Self {
        Self {
            status,
            success: false,
            data: None,
            message: Some(message.to_string()),
        }
    }
}
