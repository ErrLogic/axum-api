use axum::Json;

use crate::shared::response::ApiResponse;

pub async fn health_check() -> Json<ApiResponse<()>> {
    Json(ApiResponse::empty_success(
        "HEALTH_OK",
        "Service is running",
    ))
}
