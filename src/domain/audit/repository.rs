use async_trait::async_trait;

use super::entity::AuditLog;

#[async_trait]
pub trait AuditLogRepository: Send + Sync {
    async fn store(&self, log: AuditLog);
}
