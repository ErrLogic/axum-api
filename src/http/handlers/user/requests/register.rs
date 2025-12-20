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
use crate::shared::{api_codes, api_messages};

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
    PasswordPolicy::validate(&payload.password)
        .map_err(|_| ApiError::Validation {
            code: api_codes::users::WEAK_PASSWORD,
            message: api_messages::users::WEAK_PASSWORD,
            errors: [(
                        "password".to_string(),
                        vec![api_messages::validator::INVALID_PASSWORD_FORMAT.to_string()],
                    )]
                .into_iter()
                .collect(),
        })?;

    let password_hash = state
        .password_hasher
        .hash(&payload.password)
        .map_err(|_| ApiError::Internal {
            code: api_codes::auth::HASHING_FAILED,
            message: api_messages::auth::HASHING_FAILED,
        })?;

    let use_case = RegisterUserUseCase::new(
        state.user_repo.clone(),
    );
    
    let cmd = RegisterUserCommand {
        name: payload.name,
        email: payload.email,
        password_hash,
    };

    use_case.execute(cmd).await.map_err(|err| match err {
        RegisterUserError::EmailAlreadyExists => ApiError::BadRequest {
            code: api_codes::auth::EMAIL_ALREADY_EXISTS,
            message: api_messages::auth::EMAIL_ALREADY_EXISTS,
        },
        RegisterUserError::InvalidUserData => ApiError::Validation {
            code: api_codes::validator::VALIDATION_ERROR,
            message: api_messages::validator::INVALID_USER_DATA,
            errors: std::collections::HashMap::new(),
        },
        _ => ApiError::Internal {
            code: api_codes::auth::REGISTER_USER_FAILED,
            message: api_messages::auth::REGISTER_USER_FAILED,
        },
    })?;

    Ok(Json(ApiResponse::empty_success(
        api_codes::auth::REGISTER_USER_SUCCESS,
        api_messages::auth::REGISTER_USER_SUCCESS,
    )))
}