use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

use crate::domain::user::{
    entity::User,
    repository::{UserRepository, UserRepositoryError},
};

#[derive(Debug, Error)]
pub enum GetCurrentUserError {
    #[error("user not found")]
    NotFound,

    #[error("repository error")]
    RepositoryError,
}

pub struct GetCurrentUserUseCase {
    repo: Arc<dyn UserRepository>,
}

impl GetCurrentUserUseCase {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        user_id: Uuid,
    ) -> Result<User, GetCurrentUserError> {
        match self.repo.find_by_id(user_id).await {
            Ok(user) => Ok(user),
            Err(UserRepositoryError::NotFound) => Err(GetCurrentUserError::NotFound),
            Err(_) => Err(GetCurrentUserError::RepositoryError),
        }
    }
}
