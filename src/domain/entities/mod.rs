use crate::domain::repositories::Entity;

#[derive(Debug, Clone)]
pub struct User {
    id: String,
    email: String,
    name: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl Entity for User {
    fn id(&self) -> &str {
        &self.id
    }
}

impl User {
    pub fn new(id: String, email: String, name: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id,
            email,
            name,
            created_at: now,
            updated_at: now,
        }
    }

    /// Create User from database row - used by infrastructure adapters
    pub fn from_db(
        id: String,
        email: String,
        name: String,
        created_at: chrono::DateTime<chrono::Utc>,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            id,
            email,
            name,
            created_at,
            updated_at,
        }
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.updated_at
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name;
        self.updated_at = chrono::Utc::now();
    }
}

#[derive(Debug, Clone)]
pub struct Notification {
    id: String,
    user_id: String,
    subject: String,
    body: String,
    sent_at: Option<chrono::DateTime<chrono::Utc>>,
    status: NotificationStatus,
}

impl Entity for Notification {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NotificationStatus {
    Pending,
    Sent,
    Failed,
}

impl Notification {
    pub fn new(id: String, user_id: String, subject: String, body: String) -> Self {
        Self {
            id,
            user_id,
            subject,
            body,
            sent_at: None,
            status: NotificationStatus::Pending,
        }
    }

    /// Create Notification from database row - used by infrastructure adapters
    pub fn from_db(
        id: String,
        user_id: String,
        subject: String,
        body: String,
        sent_at: Option<chrono::DateTime<chrono::Utc>>,
        status: NotificationStatus,
    ) -> Self {
        Self {
            id,
            user_id,
            subject,
            body,
            sent_at,
            status,
        }
    }

    pub fn mark_sent(&mut self) {
        self.sent_at = Some(chrono::Utc::now());
        self.status = NotificationStatus::Sent;
    }

    pub fn mark_failed(&mut self) {
        self.status = NotificationStatus::Failed;
    }

    pub fn status(&self) -> NotificationStatus {
        self.status.clone()
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn sent_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.sent_at
    }
}
