use crate::model::dto::user_dto::UserDto;
use crate::model::vo::user_search::UserSearch;
use crate::model::vo::{login_request::LoginRequest, register_request::RegisterRequest};
use crate::{PulseResponse, service::user_service::UserService, utils::result::PulseResponseBody};
use actix_web::{HttpMessage, HttpRequest, get, post, web};

#[post("/login")]
pub async fn user_login(login_data: web::Json<LoginRequest>, token_security: web::Data<String>) -> PulseResponse<String> {
    let login_token = UserService::login(login_data.into_inner(), &token_security).await?;
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

#[get("/search_contact")]
pub async fn search_contact(
    content: web::Query<UserSearch>,
    req: HttpRequest,
) -> PulseResponse<Vec<UserDto>> {
    let extensions = req.extensions_mut();
    let user_id = extensions
        .get::<u64>()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("无法获取用户ID"))?;

    let list = UserService::search_contact(content.content(), *user_id).await?;
    Ok(PulseResponseBody::success(list))
}

#[post("/search_user")]
pub async fn search_user(content: web::Query<String>) -> PulseResponse<Vec<UserDto>> {
    let user_list = UserService::search_user(content.into_inner()).await?;
    Ok(PulseResponseBody::success(user_list))
}

#[get("/contact_list")]
pub async fn contact_list(req: HttpRequest) -> PulseResponse<Vec<UserDto>> {
    let extensions = req.extensions_mut();
    let user_id = extensions
        .get::<u64>()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("无法获取用户ID"))?;
    let contact_list = UserService::contact_list(*user_id).await?;
    Ok(PulseResponseBody::success(contact_list))
}

#[get("/info")]
pub async fn get_user_info(req: HttpRequest) -> PulseResponse<UserDto> {
    let extensions = req.extensions_mut();
    let user_id = extensions
        .get::<u64>()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("无法获取用户ID"))?;
    let user_info = UserService::get_user_info(*user_id).await?;
    Ok(PulseResponseBody::success(user_info))
}

pub fn user_request_config(service_config: &mut web::ServiceConfig) {
    let scope = web::scope("/api/user")
        .service(user_login)
        .service(user_loginout)
        .service(get_user_info)
        .service(user_register)
        .service(contact_list)
        .service(search_contact)
        .service(search_user);
    service_config.service(scope);
}
