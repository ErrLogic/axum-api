use std::time::Duration;

pub struct RateLimitRule {
    pub limit: u32,
    pub window: Duration,
}

pub fn policy_for_path(path: &str) -> Option<RateLimitRule> {
    match path {
        "/login" => Some(RateLimitRule {
            limit: 5,
            window: Duration::from_secs(60),
        }),
        "/logout" => Some(RateLimitRule {
            limit: 5,
            window: Duration::from_secs(60),
        }),
        "/refresh" => Some(RateLimitRule {
            limit: 10,
            window: Duration::from_secs(60),
        }),
        "/register" => Some(RateLimitRule {
            limit: 10,
            window: Duration::from_secs(60),
        }),
        "/change-password" => Some(RateLimitRule {
            limit: 10,
            window: Duration::from_secs(60),
        }),
        _ => None,
    }
}
