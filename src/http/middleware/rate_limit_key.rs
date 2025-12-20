use axum::body::Body;
use axum::http::Request;

pub fn extract_client_identifier(req: &Request<Body>) -> String {
    if let Some(value) = req.headers().get("x-user-id") {
        if let Ok(v) = value.to_str() {
            return format!("user:{v}");
        }
    }

    if let Some(value) = req.headers().get("x-forwarded-for") {
        if let Ok(v) = value.to_str() {
            return format!("ip:{v}");
        }
    }

    "unknown".to_string()
}
