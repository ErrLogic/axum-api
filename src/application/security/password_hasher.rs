pub trait PasswordHasher: Send + Sync {
    fn hash(&self, plain: &str) -> Result<String, PasswordHashError>;
    fn verify(&self, plain: &str, hash: &str) -> Result<bool, PasswordHashError>;
}

#[derive(Debug)]
pub enum PasswordHashError {
    HashFailed,
    VerifyFailed,
}
