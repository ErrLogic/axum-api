use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::security::jwt::{JwtClaims, JwtError, JwtService};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub struct JwtServiceImpl {
    encoding: EncodingKey,
    decoding: DecodingKey,
    ttl_seconds: i64,
}

impl JwtServiceImpl {
    pub fn new(secret: &str, ttl_seconds: i64) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret.as_bytes()),
            decoding: DecodingKey::from_secret(secret.as_bytes()),
            ttl_seconds,
        }
    }
}

impl JwtService for JwtServiceImpl {
    fn generate(&self, user_id: Uuid) -> Result<String, JwtError> {
        let exp = (Utc::now() + Duration::seconds(self.ttl_seconds)).timestamp();

        let claims = Claims {
            sub: user_id.to_string(),
            exp: exp as usize,
        };

        encode(&Header::default(), &claims, &self.encoding)
            .map_err(|_| JwtError::Internal)
    }

    fn verify(&self, token: &str) -> Result<JwtClaims, JwtError> {
        let data = decode::<Claims>(
            token,
            &self.decoding,
            &Validation::default(),
        )
            .map_err(|_| JwtError::InvalidToken)?;

        let user_id = Uuid::parse_str(&data.claims.sub)
            .map_err(|_| JwtError::InvalidToken)?;

        Ok(JwtClaims {
            sub: user_id,
            exp: data.claims.exp as i64,
        })
    }
}
