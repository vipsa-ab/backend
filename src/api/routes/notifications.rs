use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json, Router,
};
use std::sync::Arc;

use crate::api::dtos::{ApiResponse, CreateNotificationDto, NotificationDto};
use crate::domain::repositories::DomainError;
use crate::domain::services::NotificationService;
use crate::infrastructure::mappers::{dto_to_domain, entity_to_dto::EntityToDto};

#[derive(Clone)]
pub struct NotificationState {
    pub notification_service: Arc<NotificationService>,
}

pub fn routes(state: NotificationState) -> Router {
    Router::new()
        .route(
            "/notifications",
            axum::routing::get(get_notifications).post(create_notification),
        )
        .route("/notifications/{id}", axum::routing::get(get_notification))
        .route(
            "/notifications/{id}/send",
            axum::routing::post(send_notification),
        )
        .with_state(state)
}

async fn get_notifications(
    State(state): State<NotificationState>,
) -> Result<Json<ApiResponse<Vec<NotificationDto>>>, StatusCode> {
    match state.notification_service.list_notifications().await {
        Ok(notifications) => Ok(Json(ApiResponse::success(
            notifications
                .iter()
                .map(EntityToDto::to_notification_dto)
                .collect(),
        ))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_notification(
    State(state): State<NotificationState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<NotificationDto>>, StatusCode> {
    match state.notification_service.get_notification(&id).await {
        Ok(notification) => Ok(Json(ApiResponse::success(
            EntityToDto::to_notification_dto(&notification),
        ))),
        Err(DomainError::NotFound) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Create notification WITHOUT sending email
/// Use this for batch operations or queued notifications
async fn create_notification(
    State(state): State<NotificationState>,
    Json(payload): Json<CreateNotificationDto>,
) -> Result<Json<ApiResponse<NotificationDto>>, StatusCode> {
    let (user_id, subject, body) = dto_to_domain::create_notification_from_dto(&payload);

    match state
        .notification_service
        .create_notification(user_id, subject, body)
        .await
    {
        Ok(notification) => Ok(Json(ApiResponse::success(
            EntityToDto::to_notification_dto(&notification),
        ))),
        Err(DomainError::Validation(_)) => Err(StatusCode::BAD_REQUEST),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Create notification AND send email immediately
/// This demonstrates the "both things" endpoint pattern
async fn send_notification(
    State(state): State<NotificationState>,
    Path(id): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<NotificationDto>>, StatusCode> {
    // Get email from payload - in real app this would come from user lookup
    let email = payload
        .get("email")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    match state
        .notification_service
        .retry_notification(&id, email)
        .await
    {
        Ok(notification) => Ok(Json(ApiResponse::success(
            EntityToDto::to_notification_dto(&notification),
        ))),
        Err(DomainError::NotFound) => Err(StatusCode::NOT_FOUND),
        Err(DomainError::BusinessRule(_)) => Err(StatusCode::CONFLICT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
