use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::http::extractors::client_context::ClientContext;
use crate::shared::{api_codes, api_messages};
use crate::{
    application::user::login_user::{LoginUserCommand, LoginUserError, LoginUserUseCase},
    http::error::ApiError,
    shared::{response::ApiResponse, state::AppState},
};

#[derive(Debug, Deserialize)]
pub struct LoginUserRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
}

pub async fn login_user(
    State(state): State<AppState>,
    client_ctx: ClientContext,
    Json(payload): Json<LoginUserRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, ApiError> {
    let use_case = LoginUserUseCase::new(
        state.user_repo.clone(),
        state.refresh_token_repo.clone(),
        state.password_hasher.clone(),
        state.audit_logger.clone(),
        state.config.refresh_token_ttl_seconds,
    );

    let cmd = LoginUserCommand {
        email: payload.email,
        password: payload.password,
        context: ClientContext {
            ip: client_ctx.ip,
            user_agent: client_ctx.user_agent,
        },
    };

    let result = use_case.execute(cmd).await.map_err(|err| match err {
        LoginUserError::InvalidCredentials => ApiError::Unauthorized {
            code: api_codes::auth::INVALID_CREDENTIALS,
            message: api_messages::auth::INVALID_CREDENTIALS,
        },
        _ => ApiError::Internal {
            code: api_codes::auth::LOGIN_FAILED,
            message: api_messages::auth::LOGIN_FAILED,
        },
    })?;

    let access_token =
        state
            .jwt_service
            .generate(result.user_id)
            .map_err(|_| ApiError::Internal {
                code: api_codes::auth::TOKEN_GENERATION_FAILED,
                message: api_messages::auth::TOKEN_GENERATION_FAILED,
            })?;

    Ok(Json(ApiResponse::success(
        api_codes::auth::LOGIN_SUCCESS,
        api_messages::auth::LOGIN_SUCCESS,
        LoginResponse {
            access_token,
            refresh_token: result.refresh_token,
            token_type: "Bearer".to_string(),
        },
    )))
}
