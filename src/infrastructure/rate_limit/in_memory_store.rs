use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;

use crate::domain::rate_limit::{
    bucket::RateLimitBucket,
    store::{RateLimitError, RateLimitStore},
};

pub struct InMemoryRateLimitStore {
    buckets: Mutex<HashMap<String, RateLimitBucket>>,
}

impl InMemoryRateLimitStore {
    pub fn new() -> Self {
        Self {
            buckets: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl RateLimitStore for InMemoryRateLimitStore {
    async fn check(
        &self,
        key: String,
        limit: u32,
        window: Duration,
    ) -> Result<bool, RateLimitError> {
        let mut map = self.buckets.lock().unwrap();

        let bucket = map
            .entry(key)
            .or_insert_with(|| RateLimitBucket::new(window));

        Ok(bucket.allow(limit, window))
    }
}
