use axum::{Json, extract::State};
use serde::Deserialize;

use crate::shared::state::AppState;
use crate::shared::response::ApiResponse;
use crate::http::error::ApiError;
use crate::application::auth::logout::{LogoutUseCase, LogoutError};
use crate::shared::{api_codes, api_messages};

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
            code: api_codes::auth::INVALID_REFRESH_TOKEN,
            message: api_messages::auth::INVALID_REFRESH_TOKEN,
        },
        _ => ApiError::Internal {
            code: api_codes::auth::LOGOUT_FAILED,
            message: api_messages::auth::LOGOUT_FAILED,
        },
    })?;

    Ok(Json(ApiResponse::empty_success(
        api_codes::auth::LOGOUT_SUCCESS,
        api_messages::auth::LOGOUT_SUCCESS,
    )))
}
