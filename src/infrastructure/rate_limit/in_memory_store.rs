use crate::domain::rate_limit::bucket::RateLimitBucket;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;

pub struct InMemoryRateLimitStore {
    buckets: Mutex<HashMap<String, RateLimitBucket>>,
}

impl InMemoryRateLimitStore {
    pub fn new() -> Self {
        Self {
            buckets: Mutex::new(HashMap::new()),
        }
    }

    pub fn check(&self, key: String, limit: u32, window: Duration) -> bool {
        let mut map = self.buckets.lock().unwrap();

        let bucket = map
            .entry(key)
            .or_insert_with(|| RateLimitBucket::new(window));

        bucket.allow(limit, window)
    }
}
