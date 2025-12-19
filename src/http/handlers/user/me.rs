use axum::Json;
use axum::extract::State;

use crate::{
    application::user::get_current_user::GetCurrentUserUseCase,
    http::{extractors::auth_user::AuthUser, error::ApiError},
    shared::response::ApiResponse,
};
use crate::application::user::get_current_user::GetCurrentUserError;
use crate::shared::state::AppState;
use super::me_response::MeResponse;

pub async fn me(
    AuthUser(auth): AuthUser,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<MeResponse>>, ApiError> {

    let use_case = GetCurrentUserUseCase::new(
        state.user_repo.clone(),
    );

    let user = use_case
        .execute(auth.user_id)
        .await
        .map_err(|err| match err {
            GetCurrentUserError::NotFound => ApiError::NotFound {
                code: "USER_NOT_FOUND",
                message: "user not found",
            },
            _ => ApiError::Internal {
                code: "GET_ME_FAILED",
                message: "failed to fetch user",
            },
        })?;

    Ok(Json(ApiResponse::success(
        "ME",
        "current user",
        user.into(),
    )))
}
