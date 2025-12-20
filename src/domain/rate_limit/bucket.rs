use std::time::{Duration, Instant};

pub struct RateLimitBucket {
    pub count: u32,
    pub reset_at: Instant,
}

impl RateLimitBucket {
    pub fn new(window: Duration) -> Self {
        Self {
            count: 0,
            reset_at: Instant::now() + window,
        }
    }

    pub fn allow(&mut self, limit: u32, window: Duration) -> bool {
        let now = Instant::now();

        if now >= self.reset_at {
            self.count = 0;
            self.reset_at = now + window;
        }

        self.count += 1;
        self.count <= limit
    }
}
