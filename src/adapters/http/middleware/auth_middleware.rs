use crate::auth::jwt::verify_jwt;
use crate::errors::app_error::AppError;
use crate::state::AppState;
use axum::{
    body::Body,
    http::{Request, header},
    middleware::Next,
    response::Response,
};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .ok_or(AppError::BadRequest("Missing token".into()))?;

    let auth_str = auth_header
        .to_str()
        .map_err(|_| AppError::BadRequest("Invalid header".into()))?;

    let token = auth_str.replace("Bearer ", "");

    let claims = verify_jwt(&token, &state.jwt_secret)
        .map_err(|_| AppError::BadRequest("Invalid token".into()))?;

    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}
