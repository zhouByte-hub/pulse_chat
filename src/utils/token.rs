use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::PulseResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct PulseClaims {
    #[serde(rename = "iss")]
    pub issuer: String, // 签发人
    #[serde(rename = "sub")]
    pub subject: String, // 主题
    #[serde(rename = "exp")]
    pub expiration: usize, // 过期时间
    #[serde(rename = "iat")]
    pub issued_at: usize, // 签发时间

    pub user_id: u64,
}

impl PulseClaims {
    pub fn new(user_id: u64) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        Self {
            issuer: "pulse_chat".to_string(),
            subject: "pulse_chat".to_string(),
            expiration: now + 3600,
            issued_at: now,
            user_id,
        }
    }

    pub fn generate_token(&self, secret: &str) -> PulseResult<String> {
        let token = encode(
            &Header::new(Algorithm::HS256),
            &self,
            &EncodingKey::from_secret(secret.as_bytes()),
        )?;
        Ok(token)
    }

    pub fn verify_token(token: &str, secret: &str) -> PulseResult<u64> {
        let decoded = decode::<PulseClaims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )?;
        Ok(decoded.claims.user_id)
    }
}
