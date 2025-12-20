use axum::{
    extract::{ConnectInfo, FromRequestParts},
    http::request::Parts,
};
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct ClientContext {
    pub ip: Option<String>,
    pub user_agent: Option<String>,
}

impl<S> FromRequestParts<S> for ClientContext
where
    S: Send + Sync,
{
    type Rejection = ();

    fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> impl Future<Output = Result<Self, <Self as FromRequestParts<S>>::Rejection>> + Send {
        let headers = parts.headers.clone();
        let connect_info = parts.extensions.get::<ConnectInfo<SocketAddr>>().cloned();

        Box::pin(async move {
            let ip = headers
                .get("x-forwarded-for")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.split(',').next().unwrap_or(s).to_string())
                .or_else(|| connect_info.map(|ci| ci.0.ip().to_string()));

            let user_agent = headers
                .get("user-agent")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            Ok(ClientContext { ip, user_agent })
        })
    }
}
