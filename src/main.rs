use actix_web::{App, HttpResponse, HttpServer};

use crate::config::error_config::PulseError;

pub(crate) mod config;
pub(crate) mod controller;
pub(crate) mod model;
pub(crate) mod service;

pub type PulseResult<T> = Result<T, PulseError>;
pub type PulseResponse = Result<HttpResponse, PulseError>;

#[actix_web::main]
async fn main() -> PulseResult<()> {
    HttpServer::new(|| App::new())
        .bind("0.0.0.0:9527")?
        .run()
        .await?;
    Ok(())
}
