use axum::{routing::{get, post}, Router};

use crate::shared::state::AppState;
use crate::http::handlers::health;
use crate::http::handlers::user::{login, logout, refresh, register};

use axum::middleware;
use crate::http::middleware::auth_middleware;
use crate::http::handlers::user::me;

pub fn create_router(state: AppState) -> Router {
    let auth_routes = Router::new()
        .route("/register", post(register::register_user))
        .route("/login", post(login::login_user))
        .route("/logout", post(logout::logout))
        .route("/refresh", post(refresh::refresh_token));

    let user_routes = Router::new()
        .route("/me", get(me::me))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    Router::new()
        .route("/health", get(health::health_check))
        .nest("/auth", auth_routes)
        .nest("/users", user_routes)
        .with_state(state)
}
