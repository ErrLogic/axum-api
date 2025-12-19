use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserDomainError {
    #[error("email is invalid")]
    InvalidEmail,

    #[error("user name is too short")]
    InvalidName,
}
