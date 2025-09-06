use crate::model::entity::sea_orm_active_enums::Status;
use crate::model::entity::users;
use crate::utils::datetime_format::{deserialize_datetime_utc, serialize_datetime_utc};
use derive_getters::Getters;
use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Getters)]
pub struct UserDto {
    id: u64,
    username: String,
    status: String,
    address: String,
    description: String,
    last_message: String,
    unread_count: u64,
    avatar: String,

    #[serde(serialize_with = "serialize_datetime_utc")]
    #[serde(deserialize_with = "deserialize_datetime_utc")]
    pub time: DateTimeUtc,
}

impl UserDto {
    pub fn from(
        user: users::Model,
        last_message: Option<String>,
        unread_count: Option<u64>,
        time: Option<DateTimeUtc>,
    ) -> Self {
        let status = match user.status {
            Some(status) => match status {
                Status::Online => "online".to_string(),
                Status::Offline => "offline".to_string(),
                Status::Away => "away".to_string(),
                Status::Busy => "busy".to_string(),
            },
            None => "offline".to_string(),
        };
        Self {
            id: user.id,
            username: user.username,
            status: status,
            address: user.address.unwrap_or_default(),
            description: user.description.unwrap_or_default(),
            last_message: last_message.unwrap_or_default(),
            unread_count: unread_count.unwrap_or_default(),
            avatar: user.avatar.unwrap_or_default(),
            time: time.unwrap_or_default(),
        }
    }
}
