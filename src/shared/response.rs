use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub code: String,
    pub message: String,
    pub data: Option<T>,
    pub meta: Option<Meta>,
}

#[derive(Serialize)]
pub struct Meta {
    pub pagination: Option<PaginationMeta>,
}

#[derive(Serialize)]
pub struct PaginationMeta {
    pub page: u32,
    pub per_page: u32,
    pub total: u64,
}

impl<T> ApiResponse<T> {
    pub fn success(code: impl Into<String>, message: impl Into<String>, data: T) -> Self {
        Self {
            success: true,
            code: code.into(),
            message: message.into(),
            data: Some(data),
            meta: None,
        }
    }

    pub fn success_with_meta(
        code: impl Into<String>,
        message: impl Into<String>,
        data: T,
        meta: Meta,
    ) -> Self {
        Self {
            success: true,
            code: code.into(),
            message: message.into(),
            data: Some(data),
            meta: Some(meta),
        }
    }

    pub fn empty_success(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            success: true,
            code: code.into(),
            message: message.into(),
            data: None,
            meta: None,
        }
    }
}
