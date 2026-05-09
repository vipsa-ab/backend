use std::str::FromStr;

/// Application configuration loaded from environment variables.
///
/// In production: vars come from the system (Docker, Kubernetes, etc.)
/// In development: vars come from .env via dotenvy, or are set manually.

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub email: EmailConfig,
    pub app: ServerConfig,
    pub cors: CorsConfig,
}

#[derive(Debug, Clone)]
pub struct EmailConfig {
    pub resend_api_key: String,
    pub from_email: String,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
}

impl FromStr for CorsConfig {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let allowed_origins = if s.is_empty() {
            vec![] // No origins allowed by default
        } else {
            s.split(',')
                .map(|origin| origin.trim().to_string())
                .filter(|origin| !origin.is_empty())
                .collect()
        };
        Ok(CorsConfig { allowed_origins })
    }
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
            cors: std::env::var("VIPSA__CORS__ALLOWED_ORIGINS")
                .unwrap_or_default()
                .parse()
                .unwrap_or_else(|_| CorsConfig {
                    allowed_origins: vec![],
                }),
        }
    }

    /// Returns true if email configuration is present (API key set)
    pub fn has_email_config(&self) -> bool {
        !self.email.resend_api_key.is_empty()
    }
}
