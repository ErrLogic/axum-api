use uuid::Uuid;
use super::refresh_token::RefreshToken;

#[derive(Debug)]
pub enum RefreshTokenRepositoryError {
    NotFound,
    Unexpected,
}

#[async_trait::async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    async fn store(&self, token: RefreshToken) -> Result<(), RefreshTokenRepositoryError>;
    async fn find_by_token(&self, token: &str) -> Result<RefreshToken, RefreshTokenRepositoryError>;
    async fn revoke(&self, id: Uuid) -> Result<(), RefreshTokenRepositoryError>;
}
