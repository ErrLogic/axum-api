use axum::{extract::State, http::Request, middleware::Next, response::IntoResponse};

use crate::http::error::ApiError;
use crate::http::middleware::rate_limit_key::extract_client_identifier;
use crate::http::middleware::rate_limit_policy::policy_for_path;
use crate::shared::state::AppState;
use axum::body::Body;

pub async fn rate_limit_middleware(
    State(state): State<AppState>,
    req: Request<Body>,
    next: Next,
) -> impl IntoResponse {
    let path = req.uri().path();

    let Some(rule) = policy_for_path(path) else {
        return next.run(req).await;
    };

    let key = format!("rl:{}:{}", path, extract_client_identifier(&req));

    let allowed = state
        .rate_limit_store
        .check(key, rule.limit, rule.window)
        .await
        .unwrap_or(false);

    if !allowed {
        return ApiError::TooManyRequests {
            code: "RATE_LIMIT_EXCEEDED",
            message: "too many requests",
        }
        .into_response();
    }

    next.run(req).await
}
