use crate::application::audit::audit_logger::AuditLogger;
use crate::application::security::password_hasher::PasswordHasher;
use crate::domain::audit::action::AuditAction;
use crate::domain::auth::refresh_token::RefreshToken;
use crate::domain::auth::repository::RefreshTokenRepository;
use crate::domain::user::repository::{UserRepository, UserRepositoryError};
use base64::{engine::general_purpose, Engine as _};
use chrono::{Duration, Utc};
use rand::{rng, RngCore};
use serde_json::json;
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum LoginUserError {
    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("unexpected error")]
    Unexpected,
}

pub struct LoginContext {
    pub ip: Option<String>,
    pub user_agent: Option<String>,
}

pub struct LoginUserCommand {
    pub email: String,
    pub password: String,
    pub context: LoginContext,
}

pub struct LoginResult {
    pub user_id: Uuid,
    pub refresh_token: String,
}

pub struct LoginUserUseCase {
    user_repo: Arc<dyn UserRepository>,
    refresh_token_repo: Arc<dyn RefreshTokenRepository>,
    hasher: Arc<dyn PasswordHasher>,
    audit: Arc<AuditLogger>,
    refresh_ttl: i64,
}

impl LoginUserUseCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        refresh_token_repo: Arc<dyn RefreshTokenRepository>,
        hasher: Arc<dyn PasswordHasher>,
        audit: Arc<AuditLogger>,
        refresh_ttl: i64,
    ) -> Self {
        Self {
            user_repo,
            refresh_token_repo,
            hasher,
            audit,
            refresh_ttl,
        }
    }

    pub async fn execute(&self, cmd: LoginUserCommand) -> Result<LoginResult, LoginUserError> {
        let user = match self.user_repo.find_by_email(&cmd.email).await {
            Ok(user) => user,
            Err(UserRepositoryError::NotFound) => return Err(LoginUserError::InvalidCredentials),
            Err(_) => return Err(LoginUserError::Unexpected),
        };

        let verified = self
            .hasher
            .verify(&cmd.password, user.password_hash())
            .map_err(|_| LoginUserError::Unexpected)?;

        if !verified {
            self.audit
                .log(
                    None,
                    AuditAction::LoginFailed.as_str(),
                    "auth",
                    json!({
                        "ip": cmd.context.ip,
                        "user_agent": cmd.context.user_agent,
                        "reason": "invalid_credentials"
                    }),
                )
                .await;

            return Err(LoginUserError::InvalidCredentials);
        }

        let mut buf = [0u8; 32];
        rng().fill_bytes(&mut buf);
        let refresh_token_value = general_purpose::STANDARD.encode(buf);

        let refresh_token = RefreshToken {
            id: Uuid::now_v7(),
            user_id: user.id(),
            token: refresh_token_value.clone(),
            expires_at: Utc::now() + Duration::seconds(self.refresh_ttl),
            revoked_at: None,
        };

        self.refresh_token_repo
            .store(refresh_token)
            .await
            .map_err(|_| LoginUserError::Unexpected)?;

        self.audit
            .log(
                Some(user.id()),
                AuditAction::LoginSuccess.as_str(),
                "auth",
                json!({
                    "ip": cmd.context.ip,
                    "user_agent": cmd.context.user_agent,
                }),
            )
            .await;

        Ok(LoginResult {
            user_id: user.id(),
            refresh_token: refresh_token_value,
        })
    }
}
