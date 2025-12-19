use sqlx::PgPool;

use crate::domain::audit::{entity::AuditLog, repository::AuditLogRepository};

pub struct PostgresAuditLogRepository {
    pool: PgPool,
}

impl PostgresAuditLogRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl AuditLogRepository for PostgresAuditLogRepository {
    async fn store(&self, log: AuditLog) {
        let _ = sqlx::query(
            r#"
            INSERT INTO audit_logs (id, actor_id, action, resource, metadata, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
            .bind(log.id)
            .bind(log.actor_id)
            .bind(log.action)
            .bind(log.resource)
            .bind(log.metadata)
            .bind(log.created_at)
            .execute(&self.pool)
            .await;
        // deliberately ignored
    }
}
