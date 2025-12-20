use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;
use crate::domain::user::{
    entity::User,
    repository::UserRepository,
    value_objects::{UserEmail, UserName},
};

use crate::domain::user::repository::UserRepositoryError;

#[derive(Debug, Error)]
pub enum RegisterUserError {
    #[error("email already registered")]
    EmailAlreadyExists,

    #[error("invalid user data")]
    InvalidUserData,

    #[error("repository error")]
    RepositoryError,
}

pub struct RegisterUserCommand {
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

pub struct RegisterUserUseCase {
    repo: Arc<dyn UserRepository>,
}

impl RegisterUserUseCase {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        cmd: RegisterUserCommand,
    ) -> Result<User, RegisterUserError> {
        let name = UserName::new(cmd.name)
            .map_err(|_| RegisterUserError::InvalidUserData)?;

        let email = UserEmail::new(cmd.email)
            .map_err(|_| RegisterUserError::InvalidUserData)?;

        match self.repo.find_by_email(email.value()).await {
            Ok(_) => return Err(RegisterUserError::EmailAlreadyExists),
            Err(UserRepositoryError::NotFound) => {}
            Err(_) => return Err(RegisterUserError::RepositoryError),
        }

        let user = User::register(
            Uuid::now_v7(),
            name,
            email,
            cmd.password_hash,
        );

        self.repo
            .save(&user)
            .await
            .map_err(|_| RegisterUserError::RepositoryError)?;

        Ok(user)
    }
}
