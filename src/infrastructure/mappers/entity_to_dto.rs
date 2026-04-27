//! Entity -> DTO mappers
//! These convert domain entities to API DTOs (or any external format)

use crate::api::dtos::{NotificationDto, UserDto};
use crate::domain::entities::{Notification, NotificationStatus, User};
use crate::domain::repositories::Entity;

pub struct EntityToDto;

impl EntityToDto {
    pub fn to_user_dto(user: &User) -> UserDto {
        UserDto::from(user)
    }

    pub fn to_notification_dto(notification: &Notification) -> NotificationDto {
        NotificationDto::from(notification)
    }
}

impl From<&User> for UserDto {
    fn from(user: &User) -> Self {
        Self {
            id: user.id().to_string(),
            email: user.email().to_string(),
            name: user.name().to_string(),
            created_at: user.created_at().to_rfc3339(),
            updated_at: user.updated_at().to_rfc3339(),
        }
    }
}

impl From<&Notification> for NotificationDto {
    fn from(notification: &Notification) -> Self {
        Self {
            id: notification.id().to_string(),
            user_id: notification.user_id().to_string(),
            subject: notification.subject().to_string(),
            body: notification.body().to_string(),
            sent_at: notification.sent_at().map(|dt| dt.to_rfc3339()),
            status: match notification.status() {
                NotificationStatus::Pending => "pending".to_string(),
                NotificationStatus::Sent => "sent".to_string(),
                NotificationStatus::Failed => "failed".to_string(),
            },
        }
    }
}
