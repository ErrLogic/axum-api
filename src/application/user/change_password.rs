use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

use crate::domain::auth::repository::RefreshTokenRepository;
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

    #[error("user not found")]
    UserNotFound,

    #[error("unexpected error")]
    Unexpected,
}

pub struct ChangePasswordCommand {
    pub user_id: Uuid,
    pub current_password: String,
    pub new_password: String,
}

pub struct ChangePasswordUseCase {
    repo: Arc<dyn UserRepository>,
    refresh_repo: Arc<dyn RefreshTokenRepository>,
    hasher: Arc<dyn PasswordHasher>,
}

impl ChangePasswordUseCase {
    pub fn new(
        repo: Arc<dyn UserRepository>,
        refresh_repo: Arc<dyn RefreshTokenRepository>,
        hasher: Arc<dyn PasswordHasher>,
    ) -> Self {
        Self { repo, refresh_repo, hasher }
    }

    pub async fn execute(&self, cmd: ChangePasswordCommand) -> Result<(), ChangePasswordError> {
        let mut user = self
            .repo
            .find_by_id(cmd.user_id)
            .await
            .map_err(|_| ChangePasswordError::Unexpected)?;

        // 1️⃣ verify current password
        let valid = self
            .hasher
            .verify(&cmd.current_password, user.password_hash())
            .map_err(|_| ChangePasswordError::Unexpected)?;

        if !valid {
            return Err(ChangePasswordError::InvalidCurrentPassword);
        }

        // 2️⃣ password policy (RESULT-based)
        PasswordPolicy::validate(&cmd.new_password)
            .map_err(|_| ChangePasswordError::WeakPassword)?;

        // 3️⃣ hash new password
        let new_hash = self
            .hasher
            .hash(&cmd.new_password)
            .map_err(|_| ChangePasswordError::Unexpected)?;

        // 4️⃣ mutate domain
        user.change_password(new_hash);

        // 5️⃣ persist
        self.repo
            .update(&user)
            .await
            .map_err(|_| ChangePasswordError::Unexpected)?;

        self.refresh_repo
            .revoke_by_user(cmd.user_id)
            .await
            .map_err(|_| ChangePasswordError::Unexpected)?;

        Ok(())
    }
}
