use crate::model::entity::sea_orm_active_enums::Status;
use crate::model::entity::users;
use serde::{Deserialize, Serialize};
use derive_getters::Getters;

#[derive(Debug, Serialize, Deserialize, Getters)]
pub struct UserDto {
    id: u64,
    username: String,
    status: String,
    address: String,
    description: String
}

impl From<users::Model> for UserDto {
    fn from(user: users::Model) -> Self {
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
        }
    }
}
