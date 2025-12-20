mod application;
mod domain;
mod http;
mod infrastructure;
mod shared;

use std::net::SocketAddr;
use infrastructure::persistence::postgres_refresh_token_repository::PostgresRefreshTokenRepository;
use shared::{config::AppConfig, state::AppState};
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::sync::Arc;

use crate::application::audit::audit_logger::AuditLogger;
use crate::infrastructure::persistence::postgres_audit_log_repository::PostgresAuditLogRepository;
use infrastructure::{
    persistence::postgres_user_repository::PostgresUserRepository,
    security::{argon2_hasher::Argon2PasswordHasher, jwt_service::JwtServiceImpl},
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = AppConfig::from_env();
    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .expect("failed to connect to database");

    let user_repo = Arc::new(PostgresUserRepository::new(db.clone()));
    let password_hasher = Arc::new(Argon2PasswordHasher::new());
    let jwt_service = Arc::new(JwtServiceImpl::new(
        &config.jwt_secret,
        config.jwt_ttl_seconds,
    ));
    let refresh_token_repo = Arc::new(PostgresRefreshTokenRepository::new(db.clone()));
    let audit_repo = Arc::new(PostgresAuditLogRepository::new(db.clone()));
    let audit_logger = Arc::new(AuditLogger::new(audit_repo));

    let state = AppState {
        config,
        user_repo,
        refresh_token_repo,
        audit_logger,
        password_hasher,
        jwt_service,
    };

    let app = http::routes::create_router(state.clone());

    let addr = state.config.http_addr;
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind");

    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>(),).await.unwrap();
}
