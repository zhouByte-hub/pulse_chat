use crate::PulseResponse;
use crate::PulseResponseBody;
use crate::model::dto::message::Message;
use crate::model::dto::message::SendMessage;
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
pub async fn send_message(req: HttpRequest, message: web::Json<SendMessage>) -> PulseResponse<u64> {
    let extensions = req.extensions_mut();
    let user_id = extensions
        .get::<u64>()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("无法获取用户ID"))?;
    let result = MessageService::send_message(*user_id, &message.into_inner()).await?;
    Ok(PulseResponseBody::success(result))
}

#[get("/history/{user_id}")]
pub async fn history_message(
    req: HttpRequest,
    receive_id: web::Path<u64>,
) -> PulseResponse<Vec<Message>> {
    let extensions = req.extensions_mut();
    let user_id = extensions
        .get::<u64>()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("无法获取用户ID"))?;
    let result = MessageService::history_message(*user_id, receive_id.into_inner()).await?;
    Ok(PulseResponseBody::success(result))
}

pub fn message_request_config(service_config: &mut web::ServiceConfig) {
    let scope = web::scope("/api/message")
        .service(update_message_status)
        .service(send_message)
        .service(history_message);
    service_config.service(scope);
}
