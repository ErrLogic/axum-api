use async_trait::async_trait;
use std::time::Duration;

#[derive(Debug)]
pub enum RateLimitError {
    StorageError,
}

#[async_trait]
pub trait RateLimitStore: Send + Sync {
    async fn check(
        &self,
        key: String,
        limit: u32,
        window_secs: Duration,
    ) -> Result<bool, RateLimitError>;
}
