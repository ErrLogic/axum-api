use uuid::Uuid;

pub struct JwtClaims {
    pub sub: Uuid,
    /// exp already validated by jwt lib, kept for audit/logging
    pub exp: i64,
}

pub trait JwtService: Send + Sync {
    fn generate(&self, user_id: Uuid) -> Result<String, JwtError>;
    fn verify(&self, token: &str) -> Result<JwtClaims, JwtError>;
}

#[derive(Debug)]
pub enum JwtError {
    InvalidToken,
    Internal,
}
