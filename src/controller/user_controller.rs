use crate::model::dto::user_dto::UserDto;
use crate::model::vo::{login_request::LoginRequest, register_request::RegisterRequest};
use crate::{PulseResponse, service::user_service::UserService, utils::result::PulseResponseBody};
use actix_web::{HttpMessage, HttpRequest, get, post, web};

#[post("/login")]
pub async fn user_login(login_data: web::Json<LoginRequest>) -> PulseResponse<String> {
    let login_token = UserService::login(login_data.into_inner()).await?;
    if login_token.0 == 0 {
        return Ok(PulseResponseBody::error(login_token.1));
    }
    Ok(PulseResponseBody::success(login_token.1))
}

#[get("/logout")]
pub async fn user_loginout(req: HttpRequest) -> PulseResponse<String> {
    let extensions = req.extensions_mut();
    let user_id = extensions
        .get::<u64>()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("无法获取用户ID"))?;
    let result = UserService::loginout(*user_id).await?;
    if result.0 == 0 {
        return Ok(PulseResponseBody::error(result.1));
    }
    Ok(PulseResponseBody::success(result.1))
}

#[post("/register")]
pub async fn user_register(register_data: web::Json<RegisterRequest>) -> PulseResponse<String> {
    let register_token = UserService::register(register_data.into_inner()).await?;
    if register_token.0 == 0 {
        return Ok(PulseResponseBody::error(register_token.1));
    }
    Ok(PulseResponseBody::success(register_token.1))
}

#[post("/search_contact")]
pub async fn search_contact(
    contact_name: web::Json<String>,
    req: HttpRequest,
) -> PulseResponse<Vec<UserDto>> {
    let extensions = req.extensions_mut();
    let user_id = extensions
        .get::<u64>()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("无法获取用户ID"))?;

    let contact_list = UserService::search_contact(contact_name.into_inner(), *user_id).await?;
    Ok(PulseResponseBody::success(contact_list))
}

#[post("/search_user")]
pub async fn search_user(content: web::Query<String>) -> PulseResponse<Vec<UserDto>> {
    let user_list = UserService::search_user(content.into_inner()).await?;
    Ok(PulseResponseBody::success(user_list))
}

pub fn user_request_config(service_config: &mut web::ServiceConfig) {
    let scope = web::scope("/api/user")
        .service(user_login)
        .service(user_loginout)
        .service(user_register)
        .service(search_contact);
    service_config.service(scope);
}
