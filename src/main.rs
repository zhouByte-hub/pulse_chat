use crate::{
    config::{db_config::DatabaseConfig, error_config::PulseError, filter::TokenFilterFactory},
    controller::message_controller::message_request_config,
    controller::user_controller::user_request_config,
    utils::result::PulseResponseBody,
};
use ::config::{Config, File};
use actix_web::{App, HttpServer, web};
use serde::{Deserialize, Serialize};

pub(crate) mod config;
pub(crate) mod controller;
pub(crate) mod model;
pub(crate) mod service;
pub(crate) mod utils;

// 逻辑层面的返回类型
pub type PulseResult<T> = Result<T, PulseError>;
// 响应层面的返回类型
pub type PulseResponse<T> = Result<PulseResponseBody<T>, PulseError>;

#[actix_web::main]
async fn main() -> PulseResult<()> {
    let config = read_config().await?;
    // 初始化数据库连接
    DatabaseConfig::init_from(&config.database).await?;

    HttpServer::new(move || {
        let token_security = config.token.secret.clone();
        let token_filter =
            TokenFilterFactory::new(config.public_list.clone(), token_security.clone());
        App::new()
            .app_data(web::Data::new(token_security))
            .app_data(web::Data::new(config.email.clone()))
            .configure(user_request_config)
            .configure(message_request_config)
            .wrap(token_filter)
    })
    .bind("0.0.0.0:9527")?
    .run()
    .await?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct PulseConfig {
    database: DatabaseConfig,
    token: TokenConfig,
    public_list: Vec<String>,
    email: EmailConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenConfig {
    secret: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct EmailConfig {
    host: String,
    port: u16,
    username: String,
    password: String,
    timeout: u64,
}

async fn read_config() -> PulseResult<PulseConfig> {
    let config: PulseConfig = Config::builder()
        .add_source(File::with_name("config/application.yaml"))
        .build()?
        .try_deserialize()?;
    // 初始化数据库连接
    Ok(config)
}
