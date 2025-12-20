use axum::{
    extract::State,
    http::Request,
    middleware::Next,
    response::IntoResponse,
};

use std::time::Duration;
use axum::body::Body;
use crate::shared::state::AppState;
use crate::http::error::ApiError;

pub async fn rate_limit_middleware(
    State(state): State<AppState>,
    req: Request<Body>,
    next: Next,
) -> impl IntoResponse {
    let path = req.uri().path();

    let (limit, window) = match path {
        "/auth/login" => (5, Duration::from_secs(60)),
        "/auth/logout" => (5, Duration::from_secs(60)),
        "/auth/refresh" => (10, Duration::from_secs(60)),
        "/auth/register" => (10, Duration::from_secs(60)),
        "/users/me/change-password" => (10, Duration::from_secs(60)),
        _ => return next.run(req).await,
    };

    let ip = req
        .extensions()
        .get::<std::net::SocketAddr>()
        .map(|a| a.ip().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let key = format!("{}:{}", path, ip);

    let allowed = state
        .rate_limit_store
        .check(key, limit, window);

    if !allowed {
        return ApiError::TooManyRequests {
            code: "RATE_LIMITED",
            message: "too many requests, slow down",
        }
            .into_response();
    }

    next.run(req).await
}
