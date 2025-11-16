use config::Config;

use crate::config::{db::DbConfig, jwt::JwtConfig, server::ServerConfig, trace::TraceConfig};

pub mod common;
pub mod db;
pub mod jwt;
pub mod server;
pub mod trace;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DbConfig,
    pub jwt: JwtConfig,
    pub tracing: Option<TraceConfig>,
}

impl AppConfig {
    pub fn load_config() -> Result<Self, Box<dyn std::error::Error>> {
        use std::env;

        let config_path = env::var("CONFIG_PATH").unwrap_or("./config.yml".to_owned());
        let app_name = env::var("APP_NAME").unwrap_or("APP".to_owned());

        let configuration = Config::builder()
            .add_source(config::File::with_name(&config_path).required(false))
            .add_source(
                config::Environment::with_prefix(&app_name)
                    .prefix_separator("__")
                    .separator("_")
                    .try_parsing(true),
            )
            .build()?
            .try_deserialize()?;

        Ok(configuration)
    }
}
