use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Getters)]
pub struct ForgetPasswordRequest {
    email: String,
    check_password: String,
    new_password: String,
    code: String,
}
