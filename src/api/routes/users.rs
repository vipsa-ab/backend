use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json, Router,
};
use std::sync::Arc;

use crate::api::dtos::{ApiResponse, CreateUserDto, UpdateUserDto, UserDto};
use crate::domain::repositories::DomainError;
use crate::domain::services::UserService;
use crate::infrastructure::mappers::{dto_to_domain, entity_to_dto::EntityToDto};

/// Injects dependencies into route handlers
/// This is how we achieve the "easily swappable services" - we inject the service directly
#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<UserService>,
}

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/users", axum::routing::get(get_users).post(create_user))
        .route(
            "/users/{id}",
            axum::routing::get(get_user)
                .put(update_user)
                .delete(delete_user),
        )
        .with_state(state)
}

async fn get_users(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<UserDto>>>, StatusCode> {
    match state.user_service.list_users().await {
        Ok(users) => Ok(Json(ApiResponse::success(
            users.iter().map(EntityToDto::to_user_dto).collect(),
        ))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<UserDto>>, StatusCode> {
    match state.user_service.get_user(&id).await {
        Ok(user) => Ok(Json(ApiResponse::success(EntityToDto::to_user_dto(&user)))),
        Err(DomainError::NotFound) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserDto>,
) -> Result<Json<ApiResponse<UserDto>>, StatusCode> {
    let (email, name) = dto_to_domain::create_user_from_dto(&payload);

    match state.user_service.create_user(email, name).await {
        Ok(user) => Ok(Json(ApiResponse::success(EntityToDto::to_user_dto(&user)))),
        Err(DomainError::Validation(_msg)) => Err(StatusCode::BAD_REQUEST),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateUserDto>,
) -> Result<Json<ApiResponse<UserDto>>, StatusCode> {
    let name = dto_to_domain::update_user_from_dto(&payload);

    match state.user_service.update_user_name(&id, name).await {
        Ok(user) => Ok(Json(ApiResponse::success(EntityToDto::to_user_dto(&user)))),
        Err(DomainError::NotFound) => Err(StatusCode::NOT_FOUND),
        Err(DomainError::Validation(_)) => Err(StatusCode::BAD_REQUEST),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    match state.user_service.delete_user(&id).await {
        Ok(()) => Ok(StatusCode::NO_CONTENT),
        Err(DomainError::NotFound) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
