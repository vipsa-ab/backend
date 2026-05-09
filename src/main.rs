//! Vipsa Backend - Simple API for booking and contact
//!
//! Only three endpoints:
//! - GET  /health   - Health check
//! - POST /booking  - Create booking and send email
//! - POST /contact - Send contact email

mod api;
mod domain;
mod infrastructure;

use axum::Router;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, registry, EnvFilter};

use api::routes::booking::{routes as booking_routes, AppState};
use api::routes::contact::routes as contact_routes;
use api::routes::health::health_handler;
use infrastructure::adapters::resend::ResendEmailAdapter;
use infrastructure::config::AppConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file if present
    dotenvy::dotenv().ok();

    // Initialize tracing
    registry()
        .with(EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,tower_http=debug".to_string()),
        ))
        .with(fmt::layer())
        .init();

    // Load configuration
    let config = AppConfig::load().unwrap_or_else(|_| {
        tracing::warn!("Could not load config file, using defaults");
        AppConfig {
            email: infrastructure::config::EmailConfig::default(),
            app: infrastructure::config::ServerConfig::default(),
        }
    });

    // Create email adapter
    let email_adapter: Arc<dyn domain::services::EmailPort> = Arc::new(ResendEmailAdapter::new(
        &config.email.resend_api_key,
        config.email.from_email.clone(),
    ));

    // Create shared state for routes
    let app_state = AppState {
        email_port: email_adapter,
    };

    // Build the router with only the needed routes
    let app: Router = Router::new()
        .route("/health", axum::routing::get(health_handler))
        .merge(booking_routes(app_state.clone()))
        .merge(contact_routes(app_state.clone()))
        .layer(TraceLayer::new_for_http());

    // Start server
    let addr = format!("{}:{}", config.app.host, config.app.port);
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("Server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
