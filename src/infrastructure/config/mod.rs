use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub email: EmailConfig,
    pub app: ServerConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EmailConfig {
    pub resend_api_key: String,
    pub from_email: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("config").required(false))
            .add_source(File::with_name("config.local").required(false))
            .add_source(
                config::Environment::default()
                    .prefix("VIPSA_")
                    .separator("__"),
            )
            .build()?;

        config.try_deserialize()
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "postgres://localhost:5432/vipsa".to_string(),
            max_connections: 5,
        }
    }
}

impl Default for EmailConfig {
    fn default() -> Self {
        Self {
            resend_api_key: String::new(),
            from_email: "noreply@vipsa.com".to_string(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
        }
    }
}
