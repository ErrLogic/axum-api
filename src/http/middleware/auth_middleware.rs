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
use crate::shared::{api_codes, api_messages};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, ApiError> {

    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .ok_or(ApiError::Unauthorized {
            code: api_codes::auth::UNAUTHORIZED,
            message: api_messages::auth::UNAUTHORIZED_MISSING_HEADER,
        })?;

    let auth_header = auth_header
        .to_str()
        .map_err(|_| ApiError::Unauthorized {
            code: api_codes::auth::UNAUTHORIZED,
            message: api_messages::auth::UNAUTHORIZED_INVALID_HEADER,
        })?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(ApiError::Unauthorized {
            code: api_codes::auth::UNAUTHORIZED,
            message: api_messages::auth::UNAUTHORIZED_INVALID_HEADER,
        })?;

    let claims = state
        .jwt_service
        .verify(token)
        .map_err(|_| ApiError::Unauthorized {
            code: api_codes::auth::UNAUTHORIZED,
            message: api_messages::auth::UNAUTHORIZED_INVALID_TOKEN,
        })?;

    req.extensions_mut().insert(AuthContext {
        user_id: claims.sub,
    });

    Ok(next.run(req).await)
}