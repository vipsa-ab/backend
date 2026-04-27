use crate::domain::entities::{Notification, NotificationStatus, User};
use crate::domain::repositories::{DomainError, DomainResult, Entity, Repository};
use async_trait::async_trait;
use sqlx::{PgPool, Row};

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository<User> for PostgresUserRepository {
    async fn save(&self, entity: User) -> DomainResult<User> {
        let result = sqlx::query(
            r#"
            INSERT INTO users (id, email, name, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET
                email = EXCLUDED.email,
                name = EXCLUDED.name,
                updated_at = EXCLUDED.updated_at
            RETURNING id, email, name, created_at, updated_at
            "#,
        )
        .bind(entity.id())
        .bind(entity.email())
        .bind(entity.name())
        .bind(entity.created_at())
        .bind(entity.updated_at())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::BusinessRule(format!("Database error: {}", e)))?;

        Ok(User::from_db(
            result.get("id"),
            result.get("email"),
            result.get("name"),
            result.get("created_at"),
            result.get("updated_at"),
        ))
    }

    async fn find_by_id(&self, id: &str) -> DomainResult<Option<User>> {
        let result =
            sqlx::query("SELECT id, email, name, created_at, updated_at FROM users WHERE id = $1")
                .bind(id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| DomainError::BusinessRule(format!("Database error: {}", e)))?;

        Ok(result.map(|row| {
            User::from_db(
                row.get("id"),
                row.get("email"),
                row.get("name"),
                row.get("created_at"),
                row.get("updated_at"),
            )
        }))
    }

    async fn find_all(&self) -> DomainResult<Vec<User>> {
        let results = sqlx::query("SELECT id, email, name, created_at, updated_at FROM users")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DomainError::BusinessRule(format!("Database error: {}", e)))?;

        Ok(results
            .iter()
            .map(|row| {
                User::from_db(
                    row.get("id"),
                    row.get("email"),
                    row.get("name"),
                    row.get("created_at"),
                    row.get("updated_at"),
                )
            })
            .collect())
    }

    async fn delete(&self, id: &str) -> DomainResult<()> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::BusinessRule(format!("Database error: {}", e)))?;

        Ok(())
    }
}

pub struct PostgresNotificationRepository {
    pool: PgPool,
}

impl PostgresNotificationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository<Notification> for PostgresNotificationRepository {
    async fn save(&self, entity: Notification) -> DomainResult<Notification> {
        let status = match entity.status() {
            NotificationStatus::Pending => "pending",
            NotificationStatus::Sent => "sent",
            NotificationStatus::Failed => "failed",
        };

        let result = sqlx::query(
            r#"
            INSERT INTO notifications (id, user_id, subject, body, sent_at, status)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (id) DO UPDATE SET
                user_id = EXCLUDED.user_id,
                subject = EXCLUDED.subject,
                body = EXCLUDED.body,
                sent_at = EXCLUDED.sent_at,
                status = EXCLUDED.status
            RETURNING id, user_id, subject, body, sent_at, status
            "#,
        )
        .bind(entity.id())
        .bind(entity.user_id())
        .bind(entity.subject())
        .bind(entity.body())
        .bind(entity.sent_at())
        .bind(status)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::BusinessRule(format!("Database error: {}", e)))?;

        let status_str: String = result.get("status");
        let notification_status = match status_str.as_str() {
            "sent" => NotificationStatus::Sent,
            "failed" => NotificationStatus::Failed,
            _ => NotificationStatus::Pending,
        };

        Ok(Notification::from_db(
            result.get("id"),
            result.get("user_id"),
            result.get("subject"),
            result.get("body"),
            result.get("sent_at"),
            notification_status,
        ))
    }

    async fn find_by_id(&self, id: &str) -> DomainResult<Option<Notification>> {
        let result = sqlx::query(
            "SELECT id, user_id, subject, body, sent_at, status FROM notifications WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::BusinessRule(format!("Database error: {}", e)))?;

        Ok(result.map(|row| {
            let status_str: String = row.get("status");
            let notification_status = match status_str.as_str() {
                "sent" => NotificationStatus::Sent,
                "failed" => NotificationStatus::Failed,
                _ => NotificationStatus::Pending,
            };

            Notification::from_db(
                row.get("id"),
                row.get("user_id"),
                row.get("subject"),
                row.get("body"),
                row.get("sent_at"),
                notification_status,
            )
        }))
    }

    async fn find_all(&self) -> DomainResult<Vec<Notification>> {
        let results =
            sqlx::query("SELECT id, user_id, subject, body, sent_at, status FROM notifications")
                .fetch_all(&self.pool)
                .await
                .map_err(|e| DomainError::BusinessRule(format!("Database error: {}", e)))?;

        Ok(results
            .iter()
            .map(|row| {
                let status_str: String = row.get("status");
                let notification_status = match status_str.as_str() {
                    "sent" => NotificationStatus::Sent,
                    "failed" => NotificationStatus::Failed,
                    _ => NotificationStatus::Pending,
                };

                Notification::from_db(
                    row.get("id"),
                    row.get("user_id"),
                    row.get("subject"),
                    row.get("body"),
                    row.get("sent_at"),
                    notification_status,
                )
            })
            .collect())
    }

    async fn delete(&self, id: &str) -> DomainResult<()> {
        sqlx::query("DELETE FROM notifications WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::BusinessRule(format!("Database error: {}", e)))?;

        Ok(())
    }
}
