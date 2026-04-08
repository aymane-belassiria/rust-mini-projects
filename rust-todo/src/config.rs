use config::{Config, Envirenment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig{
    pub app_name: String,
    pub env: String,
    pub port: u16,
    pub db_url: Option<String>
}

impl AppConfig{
    pub fn load() -> Result<Self, config::ConfigError>{
        Config::builder()
            .add_source(file::with_name("config/base").required(false))
            .add_source(file::with_name("config/local").required(false))
            .add_source(Envirenment::with_prefix("APP").seprtor("_"))
            .build()?
            .try_deserialize()
    }
}