use std::sync::Arc;

use crate::PulseResult;
use crate::config::db_config::DatabaseConfig;
use crate::model::dto::user_dto::UserDto;
use crate::model::entity::sea_orm_active_enums::Status;
use crate::model::entity::{contacts, messages, users};
use crate::model::vo::{login_request::LoginRequest, register_request::RegisterRequest};
use crate::utils::token::PulseClaims;
use futures_util::future::join_all;
use sea_orm::{
    ActiveValue, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    SelectColumns,
};

pub struct UserService;

impl UserService {
    pub async fn login(login_data: LoginRequest, secret: &str) -> PulseResult<(u8, String)> {
        let connect = DatabaseConfig::get_connection()?;

        let hash_password = format!("{:X}", md5::compute(login_data.password()));
        let user = users::Entity::find()
            .filter(users::Column::Username.eq(login_data.username()))
            .filter(users::Column::PasswordHash.eq(hash_password))
            .one(&connect)
            .await?;
        if let Some(model) = user {
            let token = PulseClaims::new(model.id).generate_token(secret)?;

            // 更新登录状态
            let mut user = users::ActiveModel::from(model);
            user.status = ActiveValue::Set(Some(Status::Online));
            users::Entity::update(user).exec(&connect).await?;
            return Ok((1, token));
        }
        Ok((0, "login failed".to_string()))
    }

    pub async fn loginout(user_id: u64) -> PulseResult<(u8, String)> {
        let connect = DatabaseConfig::get_connection()?;

        let user = users::Entity::find()
            .filter(
                Condition::all()
                    .add(users::Column::Id.eq(user_id))
                    .add(users::Column::Status.eq(Status::Online)),
            )
            .one(&connect)
            .await?;
        if let Some(model) = user {
            let mut active_user = users::ActiveModel::from(model);
            active_user.status = ActiveValue::Set(Some(Status::Offline));
            users::Entity::update(active_user).exec(&connect).await?;
            return Ok((1, "loginout success".to_string()));
        }
        Ok((0, "loginout failed".to_string()))
    }

    pub async fn register(register_data: RegisterRequest) -> PulseResult<(u8, String)> {
        let connect = DatabaseConfig::get_connection()?;

        let user = users::Entity::find()
            .filter(
                Condition::any()
                    .add(users::Column::Username.eq(register_data.username()))
                    .add(users::Column::Email.eq(register_data.email())),
            )
            .one(&connect)
            .await?;
        if let Some(_) = user {
            return Ok((0, "username or email exists".to_string()));
        }
        users::Entity::insert(users::ActiveModel {
            username: ActiveValue::Set(register_data.username().to_owned()),
            password_hash: ActiveValue::Set(format!(
                "{:X}",
                md5::compute(register_data.password())
            )),
            email: ActiveValue::Set(register_data.email().to_owned()),
            ..Default::default()
        })
        .exec(&connect)
        .await?;
        Ok((1, "register success".to_string()))
    }

    pub async fn search_contact(contact_name: &String, user_id: u64) -> PulseResult<Vec<UserDto>> {
        let connect = Arc::new(DatabaseConfig::get_connection()?);

        let user_list = users::Entity::find()
            .select_column(users::Column::Id)
            .filter(
                Condition::any()
                    .add(users::Column::Username.like(format!("%{}%", contact_name)))
                    .add(users::Column::Nickname.like(format!("%{}%", contact_name)))
                    .add(users::Column::Phone.like(format!("%{}%", contact_name))),
            )
            .all(&*connect)
            .await?;

        let mut search_result: Vec<UserDto> = Vec::new();
        if !user_list.is_empty() {
            let contact_list = user_list.iter().map(|user| user.id).collect::<Vec<u64>>();

            let contact_list = contacts::Entity::find()
                .select_column(contacts::Column::ContactId)
                .filter(
                    Condition::all()
                        .add(contacts::Column::UserId.eq(user_id))
                        .add(contacts::Column::ContactId.is_in(contact_list)),
                )
                .all(&*connect)
                .await?;

            if contact_list.is_empty() {
                return Ok(search_result);
            }
            let contact_id_list = contact_list
                .iter()
                .map(|item| item.contact_id)
                .collect::<Vec<u64>>();
            let connect_clone = Arc::clone(&connect);
            let temp = users::Entity::find()
                .filter(users::Column::Id.is_in(contact_id_list))
                .all(&*connect)
                .await?
                .into_iter()
                .map(|user| async {
                    let messages = messages::Entity::find()
                        .filter(
                            Condition::all()
                                .add(messages::Column::ReceiverId.eq(user.id))
                                .add(messages::Column::SenderId.eq(user_id)),
                        )
                        .order_by_desc(messages::Column::CreatedAt)
                        .all(&*connect_clone)
                        .await
                        .unwrap_or_default();

                    let last_message = if messages.is_empty() {
                        Some("尚未聊过天".to_string())
                    } else {
                        Some(messages.first().unwrap().content.to_string())
                    };
                    let count = messages::Entity::find()
                        .filter(
                            Condition::all()
                                .add(messages::Column::ReceiverId.eq(user_id))
                                .add(messages::Column::SenderId.eq(user.id)),
                        )
                        .count(&*connect_clone)
                        .await
                        .unwrap_or_default();

                    UserDto::from(user, last_message, Some(count))
                })
                .collect::<Vec<_>>();
            search_result = join_all(temp).await;
        }
        Ok(search_result)
    }

    pub async fn search_user(content: String) -> PulseResult<Vec<UserDto>> {
        let connect = Arc::new(DatabaseConfig::get_connection()?);
        let user_list = users::Entity::find()
            .filter(
                Condition::any()
                    .add(users::Column::Username.like(format!("%{}%", content)))
                    .add(users::Column::Nickname.like(format!("%{}%", content)))
                    .add(users::Column::Phone.like(format!("%{}%", content))),
            )
            .all(&*connect)
            .await?;
        let connect_clone = Arc::clone(&connect);
        let list = user_list
            .into_iter()
            .map(|user| async {
                let messages = messages::Entity::find()
                    .filter(
                        Condition::all()
                            .add(messages::Column::ReceiverId.eq(user.id))
                            .add(messages::Column::IsRead.eq(0)),
                    )
                    .order_by_desc(messages::Column::CreatedAt)
                    .all(&*connect_clone)
                    .await
                    .unwrap_or_default();

                let last_message = if messages.is_empty() {
                    Some("尚未聊过天".to_string())
                } else {
                    Some(messages.first().unwrap().content.to_string())
                };
                UserDto::from(user, last_message, None)
            })
            .collect::<Vec<_>>();
        Ok(join_all(list).await)
    }

    pub async fn contact_list(user_id: u64) -> PulseResult<Vec<UserDto>> {
        let connect = Arc::new(DatabaseConfig::get_connection()?);
        let contact_list = contacts::Entity::find()
            .filter(
                Condition::any()
                    .add(contacts::Column::UserId.eq(user_id))
                    .add(contacts::Column::ContactId.eq(user_id)),
            )
            .all(&*connect)
            .await?;
        let contact_id_list = contact_list
            .into_iter()
            .map(|contact| {
                if contact.user_id == user_id {
                    contact.contact_id
                } else {
                    contact.user_id
                }
            })
            .collect::<Vec<u64>>();

        let user_list = users::Entity::find()
            .filter(users::Column::Id.is_in(contact_id_list))
            .all(&*connect)
            .await?;
        let connect_clone = Arc::clone(&connect);
        let list = user_list
            .into_iter()
            .map(|user| async {
                let messages = messages::Entity::find()
                        .filter(
                            Condition::all()
                                .add(messages::Column::ReceiverId.eq(user.id))
                                .add(messages::Column::SenderId.eq(user_id)),
                        )
                        .order_by_desc(messages::Column::CreatedAt)
                        .all(&*connect_clone)
                        .await
                        .unwrap_or_default();

                let last_message = if messages.is_empty() {
                    Some("尚未聊过天".to_string())
                } else {
                    Some(messages.first().unwrap().content.to_string())
                };
                let count = messages::Entity::find()
                    .filter(
                        Condition::all()
                            .add(messages::Column::ReceiverId.eq(user_id))
                            .add(messages::Column::SenderId.eq(user.id)),
                    )
                    .count(&*connect_clone)
                    .await
                    .unwrap_or_default();

                UserDto::from(user, last_message, Some(count))
            })
            .collect::<Vec<_>>();
        Ok(join_all(list).await)
    }

    pub async fn get_user_info(user_id: u64) -> PulseResult<UserDto> {
        let connect = DatabaseConfig::get_connection()?;
        let user = users::Entity::find()
            .filter(users::Column::Id.eq(user_id))
            .one(&connect)
            .await?;
        Ok(user.map(|user| UserDto::from(user, None, None)).unwrap())
    }
}
