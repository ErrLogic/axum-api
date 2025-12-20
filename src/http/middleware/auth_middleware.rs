use axum::{
    extract::State,
    http::Request,
    middleware::Next,
    response::Response,
};
use axum::body::Body;
use crate::{
    http::{auth_context::AuthContext, error::ApiError},
    shared::state::AppState,
};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, ApiError> {

    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .ok_or(ApiError::Unauthorized {
            code: "UNAUTHORIZED",
            message: "missing authorization header",
        })?;

    let auth_header = auth_header
        .to_str()
        .map_err(|_| ApiError::Unauthorized {
            code: "UNAUTHORIZED",
            message: "invalid authorization header",
        })?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(ApiError::Unauthorized {
            code: "UNAUTHORIZED",
            message: "invalid authorization header",
        })?;

    let claims = state
        .jwt_service
        .verify(token)
        .map_err(|_| ApiError::Unauthorized {
            code: "INVALID_TOKEN",
            message: "invalid or expired token",
        })?;

    // 🔐 HARDENING: attach user_id to request context
    req.extensions_mut().insert(AuthContext {
        user_id: claims.sub,
    });

    Ok(next.run(req).await)
}