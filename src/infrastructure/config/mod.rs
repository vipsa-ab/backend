use serde::Deserialize;

/// Application configuration loaded from environment variables.
///
/// In production: vars come from the system (Docker, Kubernetes, etc.)
/// In development: vars come from .env via dotenvy, or are set manually.
///
/// Environment variables use VIPSA__ prefix with __ as separator:
/// - VIPSA__EMAIL__RESEND_API_KEY
/// - VIPSA__EMAIL__FROM_EMAIL
/// - VIPSA__APP__HOST
/// - VIPSA__APP__PORT

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub email: EmailConfig,
    pub app: ServerConfig,
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
    pub fn load() -> Self {
        Self {
            email: EmailConfig {
                resend_api_key: std::env::var("VIPSA__EMAIL__RESEND_API_KEY").unwrap_or_default(),
                from_email: std::env::var("VIPSA__EMAIL__FROM_EMAIL")
                    .unwrap_or_else(|_| "no-reply@vipsa.com".to_string()),
            },
            app: ServerConfig {
                host: std::env::var("VIPSA__APP__HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: std::env::var("VIPSA__APP__PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()
                    .unwrap_or(8080),
            },
        }
    }

    /// Returns true if email configuration is present (API key set)
    pub fn has_email_config(&self) -> bool {
        !self.email.resend_api_key.is_empty()
    }
}
