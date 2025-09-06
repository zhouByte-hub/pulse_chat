use crate::utils::datetime_format::{deserialize_datetime_utc, serialize_datetime_utc};
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
    #[serde(serialize_with = "serialize_datetime_utc")]
    #[serde(deserialize_with = "deserialize_datetime_utc")]
    pub create_at: DateTimeUtc,
    pub is_send: bool,
}
