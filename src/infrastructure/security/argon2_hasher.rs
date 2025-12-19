use argon2::{
    password_hash::{PasswordHash, PasswordHasher as _, PasswordVerifier, SaltString},
    Argon2,
};
use argon2::password_hash::rand_core::OsRng;

use crate::application::security::password_hasher::{PasswordHashError, PasswordHasher};

pub struct Argon2PasswordHasher {
    argon2: Argon2<'static>,
}

impl Argon2PasswordHasher {
    pub fn new() -> Self {
        Self {
            argon2: Argon2::default(), // argon2id
        }
    }
}

impl PasswordHasher for Argon2PasswordHasher {
    fn hash(&self, plain: &str) -> Result<String, PasswordHashError> {
        let salt = SaltString::generate(&mut OsRng);

        let hash = self
            .argon2
            .hash_password(plain.as_bytes(), &salt)
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
