use crate::PulseResult;
use crate::config::db_config::DatabaseConfig;
use crate::model::entity::messages;
use crate::model::vo::message::Message;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::{Condition, EntityTrait, QueryFilter};

pub struct MessageService;

impl MessageService {
    pub async fn update_message_status(send_id: u64, receive_id: u64) -> PulseResult<u64> {
        let connect = DatabaseConfig::get_connection()?;
        let result = messages::Entity::update_many()
            .set(messages::ActiveModel {
                is_read: Set(1),
                ..Default::default()
            })
            .filter(
                Condition::all()
                    .add(messages::Column::SenderId.eq(send_id))
                    .add(messages::Column::ReceiverId.eq(receive_id))
                    .add(messages::Column::IsRead.eq(0)),
            )
            .exec(&connect)
            .await?;

        // 返回实际更新的行数
        PulseResult::Ok(result.rows_affected)
    }

    pub async fn send_message(user_id: u64, message: &Message) -> PulseResult<u64> {
        let connect = DatabaseConfig::get_connection()?;
        let result = messages::Entity::insert(messages::ActiveModel {
            sender_id: Set(user_id),
            receiver_id: Set(message.receive_id),
            content: Set(message.content.clone()),
            ..Default::default()
        })
        .exec(&connect)
        .await?;
        PulseResult::Ok(result.last_insert_id)
    }
}
