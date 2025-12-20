use axum::{
    routing::{get, post, put},
    Router,
};

use crate::http::handlers::health;
use crate::http::handlers::user::{login, logout, refresh, register};
use crate::shared::state::AppState;

use crate::http::handlers::user::{change_password, me, update_me};
use crate::http::middleware::auth_middleware;
use axum::middleware;
use crate::http::middleware::rate_limit_middleware;

pub fn create_router(state: AppState) -> Router {
    let auth_routes = Router::new()
        .route("/register", post(register::register_user))
        .route("/login", post(login::login_user))
        .route("/logout", post(logout::logout))
        .route("/refresh", post(refresh::refresh_token))
        .layer(middleware::from_fn_with_state(
            state.clone(), 
            rate_limit_middleware::rate_limit_middleware
        ));

    let user_routes = Router::new()
        .route("/me", get(me::me))
        .route("/me", put(update_me::update_me))
        .route("/me/change-password", put(change_password::change_password))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware::auth_middleware,
        ))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            rate_limit_middleware::rate_limit_middleware
        ));

    Router::new()
        .route("/health", get(health::health_check))
        .nest("/auth", auth_routes)
        .nest("/users", user_routes)
        .with_state(state)
}
