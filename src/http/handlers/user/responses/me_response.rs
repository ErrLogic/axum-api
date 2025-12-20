use serde::Serialize;
use uuid::Uuid;

use crate::domain::user::entity::User;

#[derive(Debug, Serialize)]
pub struct MeResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

impl From<User> for MeResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id(),
            name: user.name().value().to_string(),
            email: user.email().value().to_string(),
        }
    }
}
