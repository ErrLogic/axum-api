use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswordPolicyError {
    #[error("password too weak")]
    WeakPassword,
}

pub struct PasswordPolicy;

impl PasswordPolicy {
    pub fn validate(password: &str) -> Result<(), PasswordPolicyError> {
        if password.len() < 8 {
            return Err(PasswordPolicyError::WeakPassword);
        }

        // nanti gampang ditambah:
        // - uppercase
        // - number
        // - symbol

        Ok(())
    }
}
