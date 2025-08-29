use crate::model::vo::login_request::LoginRequest;
use crate::{PulseResponse, service::user_service::UserService, utils::result::PulseResponseBody};
use actix_web::{post, web};

#[post("/login")]
pub async fn user_login(login_data: web::Json<LoginRequest>) -> PulseResponse<String> {
    let login_token = UserService::login(login_data.into_inner()).await?;
    if login_token.is_empty() {
        return Ok(PulseResponseBody::error("login failed".to_string()));
    }
    Ok(PulseResponseBody::success(login_token))
}

pub fn user_request_config(service_config: &mut web::ServiceConfig) {
    let scope = web::scope("/user").service(user_login);
    service_config.service(scope);
}
