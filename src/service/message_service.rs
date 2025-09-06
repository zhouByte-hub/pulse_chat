use crate::config::db_config::DatabaseConfig;
use crate::model::entity::messages;
use crate::model::vo::message::SendMessage;
use crate::{PulseResult, model::vo::message::Message};
use sea_orm::{ActiveValue::Set, ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder};

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

    pub async fn send_message(user_id: u64, message: &SendMessage) -> PulseResult<u64> {
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

    pub async fn history_message(user_id: u64, receive_id: u64) -> PulseResult<Vec<Message>> {
        let connect = DatabaseConfig::get_connection()?;
        let result = messages::Entity::find()
            .filter(
                Condition::any()
                    .add(
                        Condition::all()
                            .add(messages::Column::SenderId.eq(user_id))
                            .add(messages::Column::ReceiverId.eq(receive_id)),
                    )
                    .add(
                        Condition::all()
                            .add(messages::Column::SenderId.eq(receive_id))
                            .add(messages::Column::ReceiverId.eq(user_id)),
                    ),
            )
            .order_by_asc(messages::Column::CreatedAt)
            .all(&connect)
            .await?;
        let messages = result
            .into_iter()
            .map(|item| Message {
                id: item.id,
                content: item.content,
                create_at: item.created_at,
                is_send: item.sender_id == user_id,
            })
            .collect();

        // 更新接收方消息为已读
        Self::update_message_status(receive_id, user_id).await?;

        PulseResult::Ok(messages)
    }
}
