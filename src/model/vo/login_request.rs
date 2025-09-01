use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Getters)]
pub struct LoginRequest {
    username: String,
    password: String,

    #[serde(rename = "rememberMe")]
    remember_me: bool
}
