use std::time::Duration;

use crate::{config::error_config::PulseError, PulseResult};
use once_cell::sync::OnceCell;
use sea_orm::{ConnectOptions, DatabaseConnection, Database};
use serde::{Deserialize, Serialize};

pub static DATABASE_CONNECT: OnceCell<DatabaseConnection> = OnceCell::new();

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub username: String,
    pub password: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u64,
    pub idle_timeout: u64,
    pub max_lifetime: u64,
    pub sqlx_logging: bool,
}

impl DatabaseConfig {
    pub async fn init_from(config: &DatabaseConfig) -> PulseResult<()> {
        let url = format!(
            "mysql://{}:{}@{}/pulse_chat",
            config.username, config.password, config.url
        );
        
        let mut connect_options = ConnectOptions::new(url);
        connect_options.max_connections(config.max_connections);
        connect_options.min_connections(config.min_connections);
        connect_options.connect_timeout(Duration::from_secs(config.connect_timeout));
        connect_options.idle_timeout(Duration::from_secs(config.idle_timeout));
        connect_options.max_lifetime(Duration::from_secs(config.max_lifetime));
        connect_options.sqlx_logging(config.sqlx_logging); 

        let connect = Database::connect(connect_options).await?;
        DATABASE_CONNECT.get_or_init(move || connect);
        Ok(())
    }
    
    // 添加获取数据库连接的方法
    pub fn get_connection() -> PulseResult<DatabaseConnection> {
        let connect = DATABASE_CONNECT.get().cloned().ok_or(PulseError::DatabaseNotInitialized)?;
        Ok(connect)
    }
}
