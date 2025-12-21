mod application;
mod domain;
mod http;
mod infrastructure;
mod shared;

use infrastructure::persistence::postgres_refresh_token_repository::PostgresRefreshTokenRepository;
use shared::{config::AppConfig, state::AppState};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use std::panic;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::application::audit::audit_logger::AuditLogger;
use crate::domain::rate_limit::store::RateLimitStore;
use crate::infrastructure::persistence::postgres_audit_log_repository::PostgresAuditLogRepository;
use crate::infrastructure::rate_limit::in_memory_store::InMemoryRateLimitStore;
use crate::infrastructure::rate_limit::redis_store::RedisRateLimitStore;
use infrastructure::{
    persistence::postgres_user_repository::PostgresUserRepository,
    security::{argon2_hasher::Argon2PasswordHasher, jwt_service::JwtServiceImpl},
};
use std::sync::Arc;
use tracing::error;
use tracing_appender::rolling::{RollingFileAppender, Rotation};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    init_logging();
    init_panic_hook();

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

    let rate_limit_store: Arc<dyn RateLimitStore> = if config.use_redis_rate_limit {
        let redis_client =
            redis::Client::open(config.redis_url.as_str()).expect("Invalid REDIS_URL");

        let redis_store = RedisRateLimitStore::new(redis_client)
            .await
            .expect("failed to connect to redis");

        Arc::new(redis_store)
    } else {
        Arc::new(InMemoryRateLimitStore::new())
    };

    let state = AppState {
        config,
        user_repo,
        refresh_token_repo,
        audit_logger,
        password_hasher,
        jwt_service,
        rate_limit_store,
    };

    let app = http::routes::create_router(state.clone());

    let addr = state.config.http_addr;
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind address");

    tracing::info!("🚀 Server up, listening on {}", addr);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

fn init_logging() {
    use once_cell::sync::OnceCell;
    use tracing_appender::non_blocking::WorkerGuard;

    static LOG_GUARD: OnceCell<WorkerGuard> = OnceCell::new();

    let env = std::env::var("APP_ENV").unwrap_or_else(|_| "local".into());

    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        "logs",
        format!("app-{}.log", env),
    );

    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);
    let _ = LOG_GUARD.set(guard);

    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_level(true);

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(file_writer)
        .with_ansi(false)
        .with_target(true)
        .with_level(true);

    let subscriber = tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(stdout_layer)
        .with(file_layer);

    let _ = subscriber.try_init();
}

fn init_panic_hook() {
    panic::set_hook(Box::new(|info| {
        let payload = info.payload();

        let message = if let Some(s) = payload.downcast_ref::<&str>() {
            *s
        } else if let Some(s) = payload.downcast_ref::<String>() {
            s.as_str()
        } else {
            "unknown panic payload"
        };

        let location = info
            .location()
            .map(|l| format!("{}:{}", l.file(), l.line()))
            .unwrap_or_else(|| "unknown location".into());

        error!(
            panic.message = %message,
            panic.location = %location,
            "application panicked"
        );
    }));
}
