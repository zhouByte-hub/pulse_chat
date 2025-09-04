use serde::{Deserialize, Serialize};
use derive_getters::Getters;


#[derive(Debug, Serialize, Deserialize, Getters)]
pub struct UserSearch {
    content: String
}