//! Vipsa Backend - Hexagonal Architecture REST API
//!
//! Architecture layers:
//! - **domain**: Core business logic, entities, ports (interfaces)
//! - **application**: Use cases (commands/queries), orchestration
//! - **infrastructure**: Adapters (Postgres, Resend), config, mappers
//! - **api**: Routes, DTOs, HTTP handlers
//!
//! Key principles:
//! - Domain has ZERO dependencies on external frameworks
//! - Ports (traits) defined in domain, implemented in infrastructure
//! - Mappers convert between layers (never expose domain types to external)
//! - Services are swappable - inject any implementation via Arc<dyn Trait>

mod api;
mod application;
mod domain;
mod infrastructure;

use axum::Router;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, registry, EnvFilter};

use domain::services::{NotificationService, UserService};
use infrastructure::adapters::postgres::{PostgresNotificationRepository, PostgresUserRepository};
use infrastructure::adapters::resend::ResendEmailAdapter;
use infrastructure::config::AppConfig;

use api::routes::health::health_handler;
use api::routes::notifications::{routes as notification_routes, NotificationState};
use api::routes::users::{routes as user_routes, AppState as UserAppState};

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<UserService>,
    pub notification_service: Arc<NotificationService>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
            database: infrastructure::config::DatabaseConfig::default(),
            email: infrastructure::config::EmailConfig::default(),
            app: infrastructure::config::ServerConfig::default(),
        }
    });

    // Initialize database connection pool
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(&config.database.url)
        .await?;

    tracing::info!("Connected to database");

    // Create repositories (adapters)
    let user_repository = Arc::new(PostgresUserRepository::new(pool.clone()));
    let notification_repository = Arc::new(PostgresNotificationRepository::new(pool.clone()));

    // Create email adapter (swappable - could be SendGrid, SMTP, etc.)
    let email_adapter = Arc::new(ResendEmailAdapter::new(
        &config.email.resend_api_key,
        config.email.from_email.clone(),
    ));

    // Create domain services with injected dependencies
    let user_service = Arc::new(UserService::new(user_repository));
    let notification_service = Arc::new(NotificationService::new(
        notification_repository,
        email_adapter,
    ));

    // Create user routes with its state
    let user_routes = user_routes(UserAppState {
        user_service: user_service.clone(),
    });

    // Create notification routes with its state
    let notification_routes = notification_routes(NotificationState {
        notification_service: notification_service.clone(),
    });

    // Build the router - use empty state type () for merging
    let app: Router = Router::new()
        .route("/health", axum::routing::get(health_handler))
        .merge(user_routes)
        .merge(notification_routes)
        .layer(TraceLayer::new_for_http());

    // Start server
    let addr = format!("{}:{}", config.app.host, config.app.port);
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("Server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
