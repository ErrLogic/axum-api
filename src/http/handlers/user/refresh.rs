use axum::Json;
use serde::{Deserialize, Serialize};
use axum::extract::State;

use crate::shared::state::AppState;
use crate::shared::response::ApiResponse;
use crate::http::error::ApiError;
use crate::application::auth::refresh_access_token::{
    RefreshAccessTokenUseCase, RefreshAccessTokenError,
};

#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct RefreshResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
}

pub async fn refresh_token(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<ApiResponse<RefreshResponse>>, ApiError> {

    let use_case = RefreshAccessTokenUseCase::new(
        state.refresh_token_repo.clone(),
        state.jwt_service.clone(),
        state.config.refresh_token_ttl_seconds,
    );

    let result = use_case.execute(payload.refresh_token).await.map_err(|e| match e {
        RefreshAccessTokenError::InvalidToken => ApiError::Unauthorized {
            code: "INVALID_REFRESH_TOKEN",
            message: "invalid or expired refresh token",
        },
        _ => ApiError::Internal {
            code: "REFRESH_FAILED",
            message: "failed to refresh token",
        },
    })?;

    let access_token = state
        .jwt_service
        .generate(result.user_id)
        .map_err(|_| ApiError::Internal {
            code: "TOKEN_GENERATION_FAILED",
            message: "failed to generate access token",
        })?;

    Ok(Json(ApiResponse::success(
        "TOKEN_REFRESHED",
        "token refreshed",
        RefreshResponse {
            access_token,
            refresh_token: result.refresh_token,
            token_type: "Bearer".to_string(),
        },
    )))
}
