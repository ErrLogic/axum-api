use axum::extract::ConnectInfo;
use axum::http::HeaderMap;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use crate::application::user::login_user::LoginContext;
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

fn extract_login_context(headers: &HeaderMap, remote_addr: SocketAddr) -> LoginContext {
    let ip = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or(s).to_string())
        .unwrap_or_else(|| remote_addr.ip().to_string());

    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    LoginContext {
        ip: Some(ip),
        user_agent,
    }
}

pub async fn login_user(
    State(state): State<AppState>,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
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
        context: extract_login_context(&headers, addr),
    };

    let result = use_case.execute(cmd).await.map_err(|err| match err {
        LoginUserError::InvalidCredentials => ApiError::Unauthorized {
            code: "INVALID_CREDENTIALS",
            message: "invalid email or password",
        },
        _ => ApiError::Internal {
            code: "LOGIN_FAILED",
            message: "failed to login",
        },
    })?;

    let access_token =
        state
            .jwt_service
            .generate(result.user_id)
            .map_err(|_| ApiError::Internal {
                code: "TOKEN_GENERATION_FAILED",
                message: "failed to generate access token",
            })?;

    Ok(Json(ApiResponse::success(
        "LOGIN_SUCCESS",
        "login successful",
        LoginResponse {
            access_token,
            refresh_token: result.refresh_token,
            token_type: "Bearer".to_string(),
        },
    )))
}
