use crate::PulseResponse;
use crate::PulseResponseBody;
use crate::model::vo::message::Message;
use crate::service::message_service::MessageService;
use actix_web::HttpMessage;
use actix_web::HttpRequest;
use actix_web::{get, post, web};

#[get("/update_status/{user_id}")]
pub async fn update_message_status(
    req: HttpRequest,
    user_id: web::Path<u64>,
) -> PulseResponse<u64> {
    let extensions = req.extensions_mut();
    let receive_id = extensions
        .get::<u64>()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("无法获取用户ID"))?;
    let result = MessageService::update_message_status(user_id.into_inner(), *receive_id).await?;
    Ok(PulseResponseBody::success(result))
}

#[post("/send")]
pub async fn send_message(req: HttpRequest, message: web::Json<Message>) -> PulseResponse<u64> {
    let extensions = req.extensions_mut();
    let user_id = extensions
        .get::<u64>()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("无法获取用户ID"))?;
    let result = MessageService::send_message(*user_id, &message.into_inner()).await?;
    Ok(PulseResponseBody::success(result))
}

pub fn message_request_config(service_config: &mut web::ServiceConfig) {
    let scope = web::scope("/api/message")
        .service(update_message_status)
        .service(send_message);
    service_config.service(scope);
}
