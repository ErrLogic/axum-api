use crate::domain::user::errors::UserDomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserEmail(String);

impl UserEmail {
    pub fn new(value: String) -> Result<Self, UserDomainError> {
        if !value.contains('@') {
            return Err(UserDomainError::InvalidEmail);
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserName(String);

impl UserName {
    pub fn new(value: String) -> Result<Self, UserDomainError> {
        if value.len() < 3 {
            return Err(UserDomainError::InvalidName);
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
