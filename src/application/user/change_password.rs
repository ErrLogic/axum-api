use serde_json::json;
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

use crate::application::audit::audit_logger::AuditLogger;
use crate::domain::audit::action::AuditAction;
use crate::domain::auth::repository::RefreshTokenRepository;
use crate::http::extractors::client_context::ClientContext;
use crate::{
    application::security::{password_hasher::PasswordHasher, password_policy::PasswordPolicy},
    domain::user::repository::UserRepository,
};

#[derive(Debug, Error)]
pub enum ChangePasswordError {
    #[error("invalid current password")]
    InvalidCurrentPassword,

    #[error("weak password")]
    WeakPassword,

    #[error("unexpected error")]
    Unexpected,
}

pub struct ChangePasswordCommand {
    pub user_id: Uuid,
    pub current_password: String,
    pub new_password: String,
    pub context: ClientContext,
}

pub struct ChangePasswordUseCase {
    repo: Arc<dyn UserRepository>,
    refresh_repo: Arc<dyn RefreshTokenRepository>,
    hasher: Arc<dyn PasswordHasher>,
    audit: Arc<AuditLogger>,
}

impl ChangePasswordUseCase {
    pub fn new(
        repo: Arc<dyn UserRepository>,
        refresh_repo: Arc<dyn RefreshTokenRepository>,
        hasher: Arc<dyn PasswordHasher>,
        audit: Arc<AuditLogger>,
    ) -> Self {
        Self {
            repo,
            refresh_repo,
            hasher,
            audit,
        }
    }

    pub async fn execute(&self, cmd: ChangePasswordCommand) -> Result<(), ChangePasswordError> {
        let mut user = self
            .repo
            .find_by_id(cmd.user_id)
            .await
            .map_err(|_| ChangePasswordError::Unexpected)?;

        let valid = self
            .hasher
            .verify(&cmd.current_password, user.password_hash())
            .map_err(|_| ChangePasswordError::Unexpected)?;

        if !valid {
            self.audit
                .log(
                    Some(user.id()),
                    AuditAction::ChangePasswordFailed.as_str(),
                    "user",
                    json!({
                        "ip": cmd.context.ip,
                        "user_agent": cmd.context.user_agent,
                        "reason": "invalid_current_password",
                    }),
                )
                .await;

            return Err(ChangePasswordError::InvalidCurrentPassword);
        }

        PasswordPolicy::validate(&cmd.new_password)
            .map_err(|_| ChangePasswordError::WeakPassword)?;

        let new_hash = self
            .hasher
            .hash(&cmd.new_password)
            .map_err(|_| ChangePasswordError::Unexpected)?;

        user.change_password(new_hash);

        self.repo
            .update(&user)
            .await
            .map_err(|_| ChangePasswordError::Unexpected)?;

        self.refresh_repo
            .revoke_by_user(cmd.user_id)
            .await
            .map_err(|_| ChangePasswordError::Unexpected)?;

        self.audit
            .log(
                Some(user.id()),
                AuditAction::ChangePasswordSuccess.as_str(),
                "user",
                json!({
                    "ip": cmd.context.ip,
                    "user_agent": cmd.context.user_agent,
                }),
            )
            .await;

        Ok(())
    }
}
