//! API DTOs - Data Transfer Objects for the REST API
//! These are external facing structures, completely separate from domain

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub timestamp: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserDto {
    pub id: String,
    pub email: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserDto {
    pub email: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUserDto {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NotificationDto {
    pub id: String,
    pub user_id: String,
    pub subject: String,
    pub body: String,
    pub sent_at: Option<String>,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateNotificationDto {
    pub user_id: String,
    pub subject: String,
    pub body: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

#[allow(dead_code)]
impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}
