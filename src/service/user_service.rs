use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::PulseResult;
use crate::config::db_config::DatabaseConfig;
use crate::model::entity::users;
use crate::model::vo::login_request::LoginRequest;
use crate::utils::token::PulseClaims;

pub struct UserService;

impl UserService {
    pub async fn login(login_data: LoginRequest) -> PulseResult<String> {
        let connect = DatabaseConfig::get_connection()?;

        let hash_password = format!("{:X}", md5::compute(login_data.password()));
        let user = users::Entity::find()
            .filter(users::Column::Username.eq(login_data.username()))
            .filter(users::Column::PasswordHash.eq(hash_password))
            .one(&connect)
            .await?;
        if let Some(model) = user {
            let token = PulseClaims::new(model.id).generate_token("secret")?;
            return Ok(token);
        }
        Ok("".to_string())
    }
}
