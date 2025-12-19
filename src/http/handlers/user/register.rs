use axum::Json;
use serde::Deserialize;

use crate::http::error::ApiError;
use crate::shared::response::ApiResponse;
use crate::shared::state::AppState;

use axum::extract::State;
use crate::application::security::password_policy::PasswordPolicy;
use crate::application::user::register_user::{
    RegisterUserCommand, RegisterUserError, RegisterUserUseCase,
};

#[derive(Debug, Deserialize)]
pub struct RegisterUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub async fn register_user(
    State(state): State<AppState>,
    Json(payload): Json<RegisterUserRequest>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    // 1️. Password policy (PLAINTEXT, BOUNDARY)
    PasswordPolicy::validate(&payload.password)
        .map_err(|_| ApiError::Validation {
            code: "WEAK_PASSWORD",
            message: "password does not meet security requirements",
            errors: [(
                "password".to_string(),
                vec!["password must be at least 8 characters".to_string()],
            )]
                .into_iter()
                .collect(),
        })?;

    let password_hash = state
        .password_hasher
        .hash(&payload.password)
        .map_err(|_| ApiError::Internal {
            code: "HASHING_FAILED",
            message: "failed to process password",
        })?;

    // 3. Build use case
    let use_case = RegisterUserUseCase::new(
        state.user_repo.clone(),
    );
    
    // 4. Execute
    let cmd = RegisterUserCommand {
        name: payload.name,
        email: payload.email,
        password_hash,
    };

    use_case.execute(cmd).await.map_err(|err| match err {
        RegisterUserError::EmailAlreadyExists => ApiError::BadRequest {
            code: "EMAIL_ALREADY_EXISTS",
            message: "email already registered",
        },
        RegisterUserError::InvalidUserData => ApiError::Validation {
            code: "VALIDATION_ERROR",
            message: "invalid user data",
            errors: std::collections::HashMap::new(),
        },
        _ => ApiError::Internal {
            code: "REGISTER_USER_FAILED",
            message: "failed to register user",
        },
    })?;

    Ok(Json(ApiResponse::empty_success(
        "USER_REGISTERED",
        "user registered successfully",
    )))
}