use axum::{Json, extract::State};
use serde::Deserialize;

use crate::shared::state::AppState;
use crate::shared::response::ApiResponse;
use crate::http::error::ApiError;
use crate::application::auth::logout::{LogoutUseCase, LogoutError};

#[derive(Debug, Deserialize)]
pub struct LogoutRequest {
    pub refresh_token: String,
}

pub async fn logout(
    State(state): State<AppState>,
    Json(payload): Json<LogoutRequest>,
) -> Result<Json<ApiResponse<()>>, ApiError> {

    let use_case = LogoutUseCase::new(
        state.refresh_token_repo.clone(),
    );

    use_case.execute(payload.refresh_token).await.map_err(|e| match e {
        LogoutError::InvalidToken => ApiError::Unauthorized {
            code: "INVALID_REFRESH_TOKEN",
            message: "invalid refresh token",
        },
        _ => ApiError::Internal {
            code: "LOGOUT_FAILED",
            message: "failed to logout",
        },
    })?;

    Ok(Json(ApiResponse::empty_success(
        "LOGOUT_SUCCESS",
        "logout successful",
    )))
}
