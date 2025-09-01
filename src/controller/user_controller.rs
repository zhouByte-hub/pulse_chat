use crate::model::vo::{login_request::LoginRequest, register_request::RegisterRequest};
use crate::{PulseResponse, service::user_service::UserService, utils::result::PulseResponseBody};
use actix_web::{post, web};

#[post("/login")]
pub async fn user_login(login_data: web::Json<LoginRequest>) -> PulseResponse<String> {
    let login_token = UserService::login(login_data.into_inner()).await?;
    if login_token.0 == 0 {
        return Ok(PulseResponseBody::error(login_token.1));
    }
    Ok(PulseResponseBody::success(login_token.1))
}

#[post("/register")]
pub async fn user_register(register_data: web::Json<RegisterRequest>) -> PulseResponse<String> {
    let register_token = UserService::register(register_data.into_inner()).await?;
    if register_token.0 == 0 {
        return Ok(PulseResponseBody::error(register_token.1));
    }
    Ok(PulseResponseBody::success(register_token.1))
}

pub fn user_request_config(service_config: &mut web::ServiceConfig) {
    let scope = web::scope("/api/user")
        .service(user_login)
        .service(user_register);
    service_config.service(scope);
}
