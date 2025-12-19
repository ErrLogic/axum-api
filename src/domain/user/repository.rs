use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::user::entity::User;

#[derive(Debug)]
pub enum UserRepositoryError {
    NotFound,
    Conflict,
    Domain,
    Unknown,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<User, UserRepositoryError>;

    async fn find_by_email(&self, email: &str) -> Result<User, UserRepositoryError>;

    async fn save(&self, user: &User) -> Result<(), UserRepositoryError>;
}
