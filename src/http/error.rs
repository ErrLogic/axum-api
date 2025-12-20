use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::shared::error::ApiErrorResponse;

#[derive(Debug)]
pub enum ApiError {
    BadRequest {
        code: &'static str,
        message: &'static str,
    },
    Unauthorized {
        code: &'static str,
        message: &'static str,
    },
    Forbidden {
        code: &'static str,
        message: &'static str,
    },
    NotFound {
        code: &'static str,
        message: &'static str,
    },
    Validation {
        code: &'static str,
        message: &'static str,
        errors: std::collections::HashMap<String, Vec<String>>,
    },
    Internal {
        code: &'static str,
        message: &'static str,
    },
    TooManyRequests {
        code: &'static str,
        message: &'static str,
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::BadRequest { code, message } => {
                let body = ApiErrorResponse::simple(code, message);
                (StatusCode::BAD_REQUEST, Json(body)).into_response()
            }

            ApiError::Unauthorized { code, message } => {
                let body = ApiErrorResponse::simple(code, message);
                (StatusCode::UNAUTHORIZED, Json(body)).into_response()
            }

            ApiError::Forbidden { code, message } => {
                let body = ApiErrorResponse::simple(code, message);
                (StatusCode::FORBIDDEN, Json(body)).into_response()
            }

            ApiError::NotFound { code, message } => {
                let body = ApiErrorResponse::simple(code, message);
                (StatusCode::NOT_FOUND, Json(body)).into_response()
            }

            ApiError::Validation {
                code,
                message,
                errors,
            } => {
                let body = ApiErrorResponse::new(
                    code,
                    message,
                    Some(crate::shared::error::ErrorDetails {
                        fields: Some(errors),
                    }),
                );

                (StatusCode::UNPROCESSABLE_ENTITY, Json(body)).into_response()
            }

            ApiError::Internal { code, message } => {
                let body = ApiErrorResponse::simple(code, message);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
            }

            ApiError::TooManyRequests { code, message } => {
                let body = ApiErrorResponse::simple(code, message);
                (StatusCode::TOO_MANY_REQUESTS, Json(body)).into_response()
            }
        }
    }
}
