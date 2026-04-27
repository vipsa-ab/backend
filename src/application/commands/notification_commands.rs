//! Notification command handlers

use crate::domain::entities::Notification;
use crate::domain::services::NotificationService;
use std::sync::Arc;

#[allow(dead_code)]
pub struct NotificationCommands {
    #[allow(dead_code)]
    service: Arc<NotificationService>,
}

#[allow(dead_code)]
impl NotificationCommands {
    pub fn new(service: Arc<NotificationService>) -> Self {
        Self { service }
    }

    #[allow(dead_code)]
    pub async fn create_and_send(
        &self,
        user_id: String,
        subject: String,
        body: String,
    ) -> Result<Notification, String> {
        self.service
            .create_and_send_notification(user_id, subject, body)
            .await
            .map_err(|e| e.to_string())
    }
}
