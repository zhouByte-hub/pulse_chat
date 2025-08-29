use actix_web::{HttpResponse, Responder, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PulseResponseBody<T> {
    pub data: Option<T>,
    pub code: u16,
    pub msg: String,
}

impl<T> PulseResponseBody<T> {
    pub fn success(data: T) -> Self {
        Self {
            data: Some(data),
            code: StatusCode::OK.as_u16(),
            msg: "success".to_string(),
        }
    }

    pub fn error(msg: String) -> Self {
        Self {
            data: None,
            code: StatusCode::BAD_REQUEST.as_u16(),
            msg,
        }
    }
}

/// 为 PulseResponseBody 实现 Responder trait，使其可以作为 HTTP 响应返回
impl<T> Responder for PulseResponseBody<T>
where
    T: Serialize,
{
    type Body = <HttpResponse as Responder>::Body;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let status_code = match self.code {
            200 => StatusCode::OK,
            400 => StatusCode::BAD_REQUEST,
            401 => StatusCode::UNAUTHORIZED,
            403 => StatusCode::FORBIDDEN,
            404 => StatusCode::NOT_FOUND,
            500 => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::OK,
        };

        HttpResponse::build(status_code)
            .insert_header(("Content-Type", "application/json"))
            .json(self)
    }
}
