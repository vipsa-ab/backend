use crate::domain::entities::{Notification, NotificationStatus};
use crate::domain::repositories::{DomainError, DomainResult, Repository};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Port for notification persistence
pub type NotificationRepository = Arc<dyn Repository<Notification>>;

/// Port for email sending - domain defines what it needs
/// This allows us to swap email providers without changing domain logic
pub trait EmailPort: Send + Sync {
    fn send_email(
        &self,
        to: &str,
        subject: &str,
        body: &str,
    ) -> Pin<Box<dyn Future<Output = Result<(), EmailError>> + Send + '_>>;
}

#[derive(Debug, Clone)]
pub enum EmailError {
    ConnectionFailed(String),
    InvalidRecipient(String),
    RateLimitExceeded,
    Unknown(String),
}

impl std::fmt::Display for EmailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmailError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            EmailError::InvalidRecipient(email) => write!(f, "Invalid recipient: {}", email),
            EmailError::RateLimitExceeded => write!(f, "Rate limit exceeded"),
            EmailError::Unknown(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl std::error::Error for EmailError {}

pub struct NotificationService {
    notification_repo: NotificationRepository,
    email_port: Arc<dyn EmailPort>,
}

impl NotificationService {
    pub fn new(notification_repo: NotificationRepository, email_port: Arc<dyn EmailPort>) -> Self {
        Self {
            notification_repo,
            email_port,
        }
    }

    /// Creates a notification and sends it via email
    /// This demonstrates the "both things" pattern you mentioned
    pub async fn create_and_send_notification(
        &self,
        user_id: String,
        subject: String,
        body: String,
    ) -> DomainResult<Notification> {
        // Domain validation
        if subject.is_empty() {
            return Err(DomainError::Validation(
                "Subject cannot be empty".to_string(),
            ));
        }
        if body.is_empty() {
            return Err(DomainError::Validation("Body cannot be empty".to_string()));
        }

        // Create notification entity
        let notification = Notification::new(
            uuid::Uuid::new_v4().to_string(),
            user_id.clone(),
            subject.clone(),
            body.clone(),
        );

        // Save to repository
        let mut saved_notification = self.notification_repo.save(notification).await?;

        // Send email via port (infrastructure concern)
        let email_result = self.email_port.send_email(&user_id, &subject, &body).await;

        // Update notification status based on email result
        match email_result {
            Ok(()) => {
                saved_notification.mark_sent();
                self.notification_repo
                    .save(saved_notification.clone())
                    .await?;
                Ok(saved_notification)
            }
            Err(_) => {
                saved_notification.mark_failed();
                let _failed_notification = self.notification_repo.save(saved_notification).await?;
                Err(DomainError::BusinessRule(
                    "Failed to send email".to_string(),
                ))
            }
        }
    }

    pub async fn get_notification(&self, id: &str) -> DomainResult<Notification> {
        self.notification_repo
            .find_by_id(id)
            .await?
            .ok_or(DomainError::NotFound)
    }

    pub async fn list_notifications(&self) -> DomainResult<Vec<Notification>> {
        self.notification_repo.find_all().await
    }

    /// Just save notification without sending (for batch operations)
    pub async fn create_notification(
        &self,
        user_id: String,
        subject: String,
        body: String,
    ) -> DomainResult<Notification> {
        if subject.is_empty() {
            return Err(DomainError::Validation(
                "Subject cannot be empty".to_string(),
            ));
        }
        if body.is_empty() {
            return Err(DomainError::Validation("Body cannot be empty".to_string()));
        }

        let notification =
            Notification::new(uuid::Uuid::new_v4().to_string(), user_id, subject, body);

        self.notification_repo.save(notification).await
    }

    /// Retry sending a failed notification
    pub async fn retry_notification(&self, id: &str, email: &str) -> DomainResult<Notification> {
        let mut notification = self
            .notification_repo
            .find_by_id(id)
            .await?
            .ok_or(DomainError::NotFound)?;

        if notification.status() == NotificationStatus::Sent {
            return Err(DomainError::BusinessRule(
                "Notification already sent".to_string(),
            ));
        }

        let email_result = self
            .email_port
            .send_email(email, notification.subject(), notification.body())
            .await;

        match email_result {
            Ok(()) => {
                notification.mark_sent();
            }
            Err(_) => {
                notification.mark_failed();
            }
        }

        self.notification_repo.save(notification).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::Entity;
    use std::sync::Mutex;

    struct MockEmailPort {
        should_fail: bool,
        calls: Mutex<Vec<String>>,
    }

    impl MockEmailPort {
        fn new(should_fail: bool) -> Self {
            Self {
                should_fail,
                calls: Mutex::new(Vec::new()),
            }
        }
    }

    impl EmailPort for MockEmailPort {
        fn send_email(
            &self,
            to: &str,
            _subject: &str,
            _body: &str,
        ) -> Pin<Box<dyn Future<Output = Result<(), EmailError>> + Send + '_>> {
            self.calls.lock().unwrap().push(to.to_string());
            let should_fail = self.should_fail;
            Box::pin(async move {
                if should_fail {
                    Err(EmailError::ConnectionFailed("Mock failure".to_string()))
                } else {
                    Ok(())
                }
            })
        }
    }

    struct MockNotificationRepository {
        notifications: Mutex<Vec<Notification>>,
    }

    impl MockNotificationRepository {
        fn new() -> Self {
            Self {
                notifications: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait::async_trait]
    impl Repository<Notification> for MockNotificationRepository {
        async fn save(&self, entity: Notification) -> DomainResult<Notification> {
            let mut notifications = self.notifications.lock().unwrap();
            if let Some(pos) = notifications.iter().position(|n| n.id() == entity.id()) {
                notifications[pos] = entity.clone();
            } else {
                notifications.push(entity.clone());
            }
            Ok(entity)
        }

        async fn find_by_id(&self, id: &str) -> DomainResult<Option<Notification>> {
            let notifications = self.notifications.lock().unwrap();
            Ok(notifications.iter().find(|n| n.id() == id).cloned())
        }

        async fn find_all(&self) -> DomainResult<Vec<Notification>> {
            let notifications = self.notifications.lock().unwrap();
            Ok(notifications.clone())
        }

        async fn delete(&self, _id: &str) -> DomainResult<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn send_notification_success() {
        let notification_repo = Arc::new(MockNotificationRepository::new());
        let email_port = Arc::new(MockEmailPort::new(false));
        let service = NotificationService::new(notification_repo.clone(), email_port);

        let result = service
            .create_and_send_notification(
                "user-123".to_string(),
                "Test Subject".to_string(),
                "Test Body".to_string(),
            )
            .await;

        assert!(result.is_ok());
        let notification = result.unwrap();
        assert_eq!(notification.status(), NotificationStatus::Sent);
    }

    #[tokio::test]
    async fn send_notification_failure() {
        let notification_repo = Arc::new(MockNotificationRepository::new());
        let email_port = Arc::new(MockEmailPort::new(true));
        let service = NotificationService::new(notification_repo, email_port);

        let result = service
            .create_and_send_notification(
                "user-123".to_string(),
                "Test Subject".to_string(),
                "Test Body".to_string(),
            )
            .await;

        // Should fail due to email error
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn retry_notification() {
        let notification_repo = Arc::new(MockNotificationRepository::new());
        let email_port = Arc::new(MockEmailPort::new(false));
        let service = NotificationService::new(notification_repo, email_port);

        // Create a notification first
        let notification = service
            .create_notification(
                "user-123".to_string(),
                "Test Subject".to_string(),
                "Test Body".to_string(),
            )
            .await
            .unwrap();

        // Retry sending it
        let result = service
            .retry_notification(notification.id(), "test@example.com")
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().status(), NotificationStatus::Sent);
    }
}
