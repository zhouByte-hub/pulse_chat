use crate::model::vo::login_request::LoginRequest;
use crate::{PulseResponse, service::user_service::UserService};
use actix_web::{HttpResponse, post, web};

#[post("/login")]
pub async fn user_login(login_data: web::Json<LoginRequest>) -> PulseResponse {
    let login_token = UserService::login(login_data.into_inner()).await?;
    Ok(HttpResponse::Ok().body(login_token))
}
