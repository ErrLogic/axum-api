use std::env;
use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub app_name: String,
    pub env: String,
    pub http_addr: SocketAddr,
    pub jwt_secret: String,
    pub database_url: String,
    pub jwt_ttl_seconds: i64,
    pub refresh_token_ttl_seconds: i64,
    pub redis_url: String,
    pub use_redis_rate_limit: bool,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let app_name = env::var("APP_NAME").unwrap_or_else(|_| "api".into());
        let env_name = env::var("APP_ENV").unwrap_or_else(|_| "local".into());
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let host = env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".into());
        let port: u16 = env::var("APP_PORT")
            .unwrap_or_else(|_| "3000".into())
            .parse()
            .expect("APP_PORT must be a number");

        let http_addr: SocketAddr = format!("{}:{}", host, port)
            .parse()
            .expect("Invalid APP_HOST or APP_PORT");

        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "change-me".into());

        let jwt_ttl_seconds: i64 = env::var("JWT_TTL_SECONDS")
            .unwrap_or_else(|_| "3600".into())
            .parse()
            .expect("JWT_TTL_SECONDS must be number");

        let refresh_token_ttl_seconds: i64 = env::var("REFRESH_TOKEN_TTL_SECONDS")
            .unwrap_or_else(|_| "604800".into()) // 7 hari
            .parse()
            .expect("REFRESH_TOKEN_TTL_SECONDS must be number");

        let redis_url = env::var("REDIS_URL")
            .unwrap_or_else(|_| "redis://127.0.0.1:6379".into());

        let use_redis_rate_limit = env::var("USE_REDIS_RATE_LIMIT")
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);

        Self {
            app_name,
            env: env_name,
            http_addr,
            jwt_secret,
            database_url,
            jwt_ttl_seconds,
            refresh_token_ttl_seconds,
            redis_url,
            use_redis_rate_limit,
        }
    }
}
