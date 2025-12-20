use axum::{extract::State, Json};
use serde::Deserialize;

use crate::{
    application::user::update_profile::{
        UpdateProfileCommand, UpdateProfileError, UpdateProfileUseCase,
    },
    http::error::ApiError,
    shared::{response::ApiResponse, state::AppState},
};
use crate::http::extractors::auth_user::AuthUser;
use crate::shared::{api_codes, api_messages};

#[derive(Debug, Deserialize)]
pub struct UpdateMeRequest {
    pub name: String,
}

pub async fn update_me(
    AuthUser(auth): AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<UpdateMeRequest>,
) -> Result<Json<ApiResponse<()>>, ApiError> {

    let use_case = UpdateProfileUseCase::new(state.user_repo.clone());

    use_case.execute(UpdateProfileCommand {
        user_id: auth.user_id,
        name: payload.name,
    })
        .await
        .map_err(|e| match e {
            UpdateProfileError::UserNotFound => ApiError::NotFound {
                code: api_codes::users::USER_NOT_FOUND,
                message: api_messages::users::USER_NOT_FOUND,
            },
            UpdateProfileError::InvalidData => ApiError::Validation {
                code: api_codes::validator::VALIDATION_ERROR,
                message: api_messages::validator::INVALID_PROFILE_DATA,
                errors: Default::default(),
            },
            _ => ApiError::Internal {
                code: api_codes::users::UPDATE_PROFILE_FAILED,
                message: api_messages::users::UPDATE_PROFILE_FAILED,
            },
        })?;

    Ok(Json(ApiResponse::empty_success(
        api_codes::users::UPDATE_PROFILE_SUCCESS,
        api_messages::users::UPDATE_PROFILE_SUCCESS,
    )))
}
