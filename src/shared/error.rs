use serde::Serialize;

#[derive(Serialize)]
pub struct ApiErrorResponse {
    pub success: bool,
    pub code: String,
    pub message: String,
    pub errors: Option<ErrorDetails>,
}

#[derive(Serialize)]
pub struct ErrorDetails {
    pub fields: Option<std::collections::HashMap<String, Vec<String>>>,
}

impl ApiErrorResponse {
    pub fn new(
        code: impl Into<String>,
        message: impl Into<String>,
        errors: Option<ErrorDetails>,
    ) -> Self {
        Self {
            success: false,
            code: code.into(),
            message: message.into(),
            errors,
        }
    }

    pub fn simple(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            success: false,
            code: code.into(),
            message: message.into(),
            errors: None,
        }
    }
}
