use std::sync::Arc;
use thiserror::Error;

use crate::domain::auth::repository::{
    RefreshTokenRepository, RefreshTokenRepositoryError,
};

#[derive(Debug, Error)]
pub enum LogoutError {
    #[error("invalid refresh token")]
    InvalidToken,

    #[error("unexpected error")]
    Unexpected,
}

pub struct LogoutUseCase {
    refresh_repo: Arc<dyn RefreshTokenRepository>,
}

impl LogoutUseCase {
    pub fn new(refresh_repo: Arc<dyn RefreshTokenRepository>) -> Self {
        Self { refresh_repo }
    }

    pub async fn execute(&self, refresh_token: String) -> Result<(), LogoutError> {
        let token = self
            .refresh_repo
            .find_by_token(&refresh_token)
            .await
            .map_err(|e| match e {
                RefreshTokenRepositoryError::NotFound => LogoutError::InvalidToken,
                _ => LogoutError::Unexpected,
            })?;

        // revoke token
        self.refresh_repo
            .revoke(token.id)
            .await
            .map_err(|_| LogoutError::Unexpected)?;

        Ok(())
    }
}
