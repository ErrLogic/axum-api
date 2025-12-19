use std::sync::Arc;
use chrono::{Duration, Utc};
use rand::{RngCore, rng};
use uuid::Uuid;
use thiserror::Error;
use base64::{engine::general_purpose, Engine as _};

use crate::domain::auth::refresh_token::RefreshToken;
use crate::domain::auth::repository::{
    RefreshTokenRepository, RefreshTokenRepositoryError,
};
use crate::application::security::jwt::JwtService;

#[derive(Debug, Error)]
pub enum RefreshAccessTokenError {
    #[error("invalid refresh token")]
    InvalidToken,
    #[error("unexpected error")]
    Unexpected,
}

pub struct RefreshResult {
    pub user_id: Uuid,
    pub refresh_token: String,
}

pub struct RefreshAccessTokenUseCase {
    refresh_repo: Arc<dyn RefreshTokenRepository>,
    jwt_service: Arc<dyn JwtService>,
    refresh_ttl: i64,
}

impl RefreshAccessTokenUseCase {
    pub fn new(
        refresh_repo: Arc<dyn RefreshTokenRepository>,
        jwt_service: Arc<dyn JwtService>,
        refresh_ttl: i64,
    ) -> Self {
        Self {
            refresh_repo,
            jwt_service,
            refresh_ttl,
        }
    }

    pub async fn execute(
        &self,
        token_value: String,
    ) -> Result<RefreshResult, RefreshAccessTokenError> {
        // 1) Find token
        let token = self
            .refresh_repo
            .find_by_token(&token_value)
            .await
            .map_err(|e| match e {
                RefreshTokenRepositoryError::NotFound => RefreshAccessTokenError::InvalidToken,
                _ => RefreshAccessTokenError::Unexpected,
            })?;

        // 2) Validate domain
        if !token.is_valid() {
            return Err(RefreshAccessTokenError::InvalidToken);
        }

        let user_id = token.user_id;

        // 3) Revoke old token
        self.refresh_repo
            .revoke(token.id)
            .await
            .map_err(|_| RefreshAccessTokenError::Unexpected)?;

        // 4) Generate new refresh token
        let mut buf = [0u8; 32];
        rng().fill_bytes(&mut buf);
        let new_value = general_purpose::STANDARD.encode(buf);

        let new_token = RefreshToken {
            id: Uuid::now_v7(),
            user_id,
            token: new_value.clone(),
            expires_at: Utc::now() + Duration::seconds(self.refresh_ttl),
            revoked_at: None,
        };

        self.refresh_repo
            .store(new_token)
            .await
            .map_err(|_| RefreshAccessTokenError::Unexpected)?;

        Ok(RefreshResult {
            user_id,
            refresh_token: new_value,
        })
    }
}
