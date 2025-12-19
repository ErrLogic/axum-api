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
                code: "USER_NOT_FOUND",
                message: "user not found",
            },
            UpdateProfileError::InvalidData => ApiError::Validation {
                code: "INVALID_DATA",
                message: "invalid profile data",
                errors: Default::default(),
            },
            _ => ApiError::Internal {
                code: "UPDATE_PROFILE_FAILED",
                message: "failed to update profile",
            },
        })?;

    Ok(Json(ApiResponse::empty_success(
        "PROFILE_UPDATED",
        "profile updated",
    )))
}
