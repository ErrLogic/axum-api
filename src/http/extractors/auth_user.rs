use std::future::{ready, Future};
use axum::{
    extract::FromRequestParts,
    http::request::Parts,
};

use crate::http::auth_context::AuthContext;
use crate::http::error::ApiError;

pub struct AuthUser(pub AuthContext);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> impl Future<Output = Result<Self, <Self as FromRequestParts<S>>::Rejection>> + Send {
        let ctx = parts
            .extensions
            .get::<AuthContext>()
            .cloned()
            .ok_or(ApiError::Unauthorized {
                code: "UNAUTHORIZED",
                message: "authentication required",
            });

        ready(ctx.map(AuthUser))
    }
}
