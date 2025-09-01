use crate::PulseResult;
use crate::config::db_config::DatabaseConfig;
use crate::model::entity::users;
use crate::model::vo::{login_request::LoginRequest, register_request::RegisterRequest};
use crate::utils::token::PulseClaims;
use sea_orm::{ActiveValue, ColumnTrait, Condition, EntityTrait, QueryFilter};

pub struct UserService;

impl UserService {
    pub async fn login(login_data: LoginRequest) -> PulseResult<(u8, String)> {
        let connect = DatabaseConfig::get_connection()?;

        let hash_password = format!("{:X}", md5::compute(login_data.password()));
        let user = users::Entity::find()
            .filter(users::Column::Username.eq(login_data.username()))
            .filter(users::Column::PasswordHash.eq(hash_password))
            .one(&connect)
            .await?;
        if let Some(model) = user {
            let token = PulseClaims::new(model.id).generate_token("secret")?;
            return Ok((1, token));
        }
        Ok((0, "login failed".to_string()))
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
}
