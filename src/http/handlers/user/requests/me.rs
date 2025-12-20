use axum::Json;
use axum::extract::State;

use crate::{
    application::user::get_current_user::GetCurrentUserUseCase,
    http::{extractors::auth_user::AuthUser, error::ApiError},
    shared::response::ApiResponse,
};
use crate::application::user::get_current_user::GetCurrentUserError;
use crate::http::handlers::user::responses::me_response::MeResponse;
use crate::shared::{api_codes, api_messages};
use crate::shared::state::AppState;

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
                code: api_codes::users::USER_NOT_FOUND,
                message: api_messages::users::USER_NOT_FOUND,
            },
            _ => ApiError::Internal {
                code: api_codes::users::GET_CURRENT_USER_FAILED,
                message: api_messages::users::GET_CURRENT_USER_FAILED,
            },
        })?;

    Ok(Json(ApiResponse::success(
        api_codes::users::GET_CURRENT_USER_SUCCESS,
        api_messages::users::GET_CURRENT_USER_SUCCESS,
        user.into(),
    )))
}
