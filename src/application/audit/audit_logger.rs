use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::audit::{entity::AuditLog, repository::AuditLogRepository};

pub struct AuditLogger {
    pub(crate) repo: Arc<dyn AuditLogRepository>,
}

impl AuditLogger {
    pub fn new(repo: Arc<dyn AuditLogRepository>) -> Self {
        Self { repo }
    }

    pub async fn log(
        &self,
        actor_id: Option<Uuid>,
        action: &str,
        resource: &str,
        metadata: serde_json::Value,
    ) {
        let log = AuditLog {
            id: Uuid::now_v7(),
            actor_id,
            action: action.to_string(),
            resource: resource.to_string(),
            metadata,
            created_at: Utc::now(),
        };

        self.repo.store(log).await;
    }
}
