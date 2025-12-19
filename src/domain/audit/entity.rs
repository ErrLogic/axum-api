use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct AuditLog {
    pub id: Uuid,
    pub actor_id: Option<Uuid>,
    pub action: String,
    pub resource: String,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}
