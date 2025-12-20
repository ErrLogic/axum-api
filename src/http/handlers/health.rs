use axum::Json;
use crate::shared::{api_codes, api_messages};
use crate::shared::response::ApiResponse;

pub async fn health_check() -> Json<ApiResponse<()>> {
    Json(ApiResponse::empty_success(
        api_codes::health::HEALTH_OK,
        api_messages::health::HEALTH_OK,
    ))
}
