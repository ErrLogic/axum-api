use argon2::{
    password_hash::{phc::PasswordHash, PasswordHasher as _, PasswordVerifier},
    Argon2,
};

use crate::application::security::password_hasher::{PasswordHashError, PasswordHasher};

pub struct Argon2PasswordHasher {
    argon2: Argon2<'static>,
}

impl Argon2PasswordHasher {
    pub fn new() -> Self {
        Self {
            argon2: Argon2::default(),
        }
    }
}

impl PasswordHasher for Argon2PasswordHasher {
    fn hash(&self, plain: &str) -> Result<String, PasswordHashError> {

        let hash = self
            .argon2
            .hash_password(plain.as_bytes())
            .map_err(|_| PasswordHashError::HashFailed)?
            .to_string();

        Ok(hash)
    }

    fn verify(&self, plain: &str, hash: &str) -> Result<bool, PasswordHashError> {
        let parsed_hash =
            PasswordHash::new(hash).map_err(|_| PasswordHashError::VerifyFailed)?;

        Ok(self
            .argon2
            .verify_password(plain.as_bytes(), &parsed_hash)
            .is_ok())
    }
}
