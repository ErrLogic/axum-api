use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

use crate::domain::user::{
    repository::{UserRepository, UserRepositoryError},
    value_objects::UserName,
};

#[derive(Debug, Error)]
pub enum UpdateProfileError {
    #[error("user not found")]
    UserNotFound,

    #[error("invalid data")]
    InvalidData,

    #[error("unexpected error")]
    Unexpected,
}

pub struct UpdateProfileCommand {
    pub user_id: Uuid,
    pub name: String,
}

pub struct UpdateProfileUseCase {
    repo: Arc<dyn UserRepository>,
}

impl UpdateProfileUseCase {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        cmd: UpdateProfileCommand,
    ) -> Result<(), UpdateProfileError> {

        let mut user = self
            .repo
            .find_by_id(cmd.user_id)
            .await
            .map_err(|e| match e {
                UserRepositoryError::NotFound => UpdateProfileError::UserNotFound,
                _ => UpdateProfileError::Unexpected,
            })?;

        let name = UserName::new(cmd.name)
            .map_err(|_| UpdateProfileError::InvalidData)?;

        user.rename(name);

        self.repo
            .update(&user)
            .await
            .map_err(|_| UpdateProfileError::Unexpected)?;

        Ok(())
    }
}
