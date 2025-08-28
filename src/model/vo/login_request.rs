use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Getters)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
