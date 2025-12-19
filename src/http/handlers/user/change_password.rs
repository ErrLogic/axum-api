use axum::{extract::State, Json};
use serde::Deserialize;

use crate::{
    application::user::change_password::{
        ChangePasswordCommand, ChangePasswordError, ChangePasswordUseCase,
    },
    http::error::ApiError,
    shared::{response::ApiResponse, state::AppState},
};
use crate::http::extractors::auth_user::AuthUser;

#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

pub async fn change_password(
    State(state): State<AppState>,
    AuthUser(auth): AuthUser,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let use_case = ChangePasswordUseCase::new(
        state.user_repo.clone(),
        state.refresh_token_repo.clone(),
        state.password_hasher.clone(),
    );


    use_case
        .execute(ChangePasswordCommand {
            user_id: auth.user_id,
            current_password: payload.current_password,
            new_password: payload.new_password,
        })
        .await
        .map_err(|e| match e {
            ChangePasswordError::InvalidCurrentPassword => ApiError::Unauthorized {
                code: "INVALID_PASSWORD",
                message: "current password is incorrect",
            },
            ChangePasswordError::WeakPassword => ApiError::Validation {
                code: "WEAK_PASSWORD",
                message: "password does not meet requirements",
                errors: Default::default(),
            },
            _ => ApiError::Internal {
                code: "CHANGE_PASSWORD_FAILED",
                message: "failed to change password",
            },
        })?;

    Ok(Json(ApiResponse::empty_success(
        "PASSWORD_CHANGED",
        "password updated successfully",
    )))
}
