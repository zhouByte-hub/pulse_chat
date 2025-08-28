use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::PulseResult;
use crate::model::entity::users;
use crate::model::vo::login_request::LoginRequest;

pub struct UserService;

impl UserService {
    pub async fn login(login_data: LoginRequest) -> PulseResult<String> {
        // let user = users::Entity::find()
        //     .filter(users::Column::Username.eq(login_data.username()))
        //     .filter(users::Column::PasswordHash.eq(login_data.password()))
        //     .one(db)
        //     .await?;
        Ok("abc".to_string())
    }
}
