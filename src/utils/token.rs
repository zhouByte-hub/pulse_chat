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


#[cfg(test)]
mod test{
    use crate::utils::token::PulseClaims;


    #[test]
    fn encode_test(){
        let claims = PulseClaims::new(1);
        match claims.generate_token("dev_123456") {
            Ok(token) => {
                println!("token: {}", token);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    #[test]
    fn decode_test(){
        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJwdWxzZV9jaGF0Iiwic3ViIjoicHVsc2VfY2hhdCIsImV4cCI6MTc1Njk5Nzk4NywiaWF0IjoxNzU2OTk0Mzg3LCJ1c2VyX2lkIjoxfQ.hjb_8GAWc7aiNCfPSVHKZ7A6yzvNIa0XdbXoZn5khz8";
        match PulseClaims::verify_token(token, "dev_123456") {
            Ok(user_id) => {
                println!("user_id: {}", user_id);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}