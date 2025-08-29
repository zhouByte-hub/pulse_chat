use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Getters)]
pub struct RegisterRequest {
    username: String,
    password: String,
    email: String,
    nickname: String,
    phone: String,
}
