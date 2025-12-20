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
use crate::http::extractors::client_context::ClientContext;
use crate::shared::{api_codes, api_messages};

#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

pub async fn change_password(
    State(state): State<AppState>,
    client_ctx: ClientContext,
    AuthUser(auth): AuthUser,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let use_case = ChangePasswordUseCase::new(
        state.user_repo.clone(),
        state.refresh_token_repo.clone(),
        state.password_hasher.clone(),
        state.audit_logger.clone(),
    );


    use_case
        .execute(ChangePasswordCommand {
            user_id: auth.user_id,
            current_password: payload.current_password,
            new_password: payload.new_password,
            context: ClientContext {
                ip: client_ctx.ip,
                user_agent: client_ctx.user_agent,
            },
        })
        .await
        .map_err(|e| match e {
            ChangePasswordError::InvalidCurrentPassword => ApiError::Unauthorized {
                code: api_codes::validator::VALIDATION_ERROR,
                message: api_messages::validator::INVALID_CURRENT_PASSWORD,
            },
            ChangePasswordError::WeakPassword => ApiError::Validation {
                code: api_codes::users::WEAK_PASSWORD,
                message: api_messages::users::WEAK_PASSWORD,
                errors: Default::default(),
            },
            _ => ApiError::Internal {
                code: api_codes::users::CHANGE_PASSWORD_FAILED,
                message: api_messages::users::CHANGE_PASSWORD_FAILED,
            },
        })?;

    Ok(Json(ApiResponse::empty_success(
        api_codes::users::CHANGE_PASSWORD_SUCCESS,
        api_messages::users::CHANGE_PASSWORD_SUCCESS,
    )))
}
