use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SendMessage {
    pub receive_id: u64,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: u64,
    pub content: String,
    pub create_at: DateTimeUtc,
    pub is_send: bool,
}
