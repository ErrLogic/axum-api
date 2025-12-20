use async_trait::async_trait;
use redis::aio::MultiplexedConnection;
use redis::Script;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

use crate::domain::rate_limit::store::{RateLimitError, RateLimitStore};

pub struct RedisRateLimitStore {
    conn: Arc<Mutex<MultiplexedConnection>>,
}

impl RedisRateLimitStore {
    pub async fn new(client: redis::Client) -> Result<Self, redis::RedisError> {
        let conn = client.get_multiplexed_async_connection().await?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }
}

#[async_trait]
impl RateLimitStore for RedisRateLimitStore {
    async fn check(
        &self,
        key: String,
        limit: u32,
        window: Duration,
    ) -> Result<bool, RateLimitError> {
        let script = Script::new(include_str!("lua/fixed_window.lua"));

        let mut conn = self.conn.lock().await;

        let allowed: i32 = script
            .key(key)
            .arg(limit)
            .arg(window.as_secs() as i64)
            .invoke_async(&mut *conn)
            .await
            .map_err(|_| RateLimitError::StorageError)?;

        Ok(allowed == 1)
    }
}
