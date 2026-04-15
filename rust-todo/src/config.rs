use config::{Config, Environment, File};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub app_name: String,
    pub env: String,
    pub port: u16,
    pub db_url: Option<String>,
}

impl AppConfig {
    pub fn load() -> Result<Self, config::ConfigError> {
        Config::builder()
            .add_source(File::with_name("config/base").required(false))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::with_prefix("APP").separator("_"))
            .build()?
            .try_deserialize()
    }
}

pub fn validate(cfg: &AppConfig) -> Result<(), String> {
    if cfg.app_name.is_empty() {
        return Err("app name cant be empty".to_string());
    }
    if cfg.port == 0 {
        return Err("port must be greater then 0".to_string());
    }
    Ok(())
}
