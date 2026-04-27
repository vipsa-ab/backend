use crate::domain::entities::User;
use crate::domain::repositories::{DomainError, DomainResult, Repository};
use std::sync::Arc;

/// Port for user persistence - this is what the domain defines as its need
pub type UserRepository = Arc<dyn Repository<User>>;

pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }

    pub async fn create_user(&self, email: String, name: String) -> DomainResult<User> {
        // Domain validation
        if email.is_empty() {
            return Err(DomainError::Validation("Email cannot be empty".to_string()));
        }
        if !email.contains('@') {
            return Err(DomainError::Validation("Invalid email format".to_string()));
        }
        if name.is_empty() {
            return Err(DomainError::Validation("Name cannot be empty".to_string()));
        }

        let user = User::new(uuid::Uuid::new_v4().to_string(), email, name);

        self.repository.save(user).await
    }

    pub async fn get_user(&self, id: &str) -> DomainResult<User> {
        self.repository
            .find_by_id(id)
            .await?
            .ok_or(DomainError::NotFound)
    }

    pub async fn list_users(&self) -> DomainResult<Vec<User>> {
        self.repository.find_all().await
    }

    pub async fn update_user_name(&self, id: &str, name: String) -> DomainResult<User> {
        if name.is_empty() {
            return Err(DomainError::Validation("Name cannot be empty".to_string()));
        }

        let mut user = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or(DomainError::NotFound)?;
        user.update_name(name);

        self.repository.save(user).await
    }

    pub async fn delete_user(&self, id: &str) -> DomainResult<()> {
        // Verify user exists
        let _ = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or(DomainError::NotFound)?;
        self.repository.delete(id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::Entity;
    use async_trait::async_trait;
    use std::sync::Mutex;

    struct MockUserRepository {
        users: Mutex<Vec<User>>,
    }

    impl MockUserRepository {
        fn new() -> Self {
            Self {
                users: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl Repository<User> for MockUserRepository {
        async fn save(&self, entity: User) -> DomainResult<User> {
            let mut users = self.users.lock().unwrap();
            if let Some(pos) = users.iter().position(|u| u.id() == entity.id()) {
                users[pos] = entity.clone();
            } else {
                users.push(entity.clone());
            }
            Ok(entity)
        }

        async fn find_by_id(&self, id: &str) -> DomainResult<Option<User>> {
            let users = self.users.lock().unwrap();
            Ok(users.iter().find(|u| u.id() == id).cloned())
        }

        async fn find_all(&self) -> DomainResult<Vec<User>> {
            let users = self.users.lock().unwrap();
            Ok(users.clone())
        }

        async fn delete(&self, id: &str) -> DomainResult<()> {
            let mut users = self.users.lock().unwrap();
            users.retain(|u| u.id() != id);
            Ok(())
        }
    }

    #[tokio::test]
    async fn create_user_valid() {
        let repo = Arc::new(MockUserRepository::new());
        let service = UserService::new(repo);

        let user = service
            .create_user("test@example.com".to_string(), "Test User".to_string())
            .await
            .unwrap();
        assert_eq!(user.email(), "test@example.com");
        assert_eq!(user.name(), "Test User");
    }

    #[tokio::test]
    async fn create_user_invalid_email() {
        let repo = Arc::new(MockUserRepository::new());
        let service = UserService::new(repo);

        let result = service.create_user("invalid-email".to_string(), "Test User".to_string());
        assert!(result.await.is_err());
    }

    #[tokio::test]
    async fn create_user_empty_name() {
        let repo = Arc::new(MockUserRepository::new());
        let service = UserService::new(repo);

        let result = service.create_user("test@example.com".to_string(), "".to_string());
        assert!(result.await.is_err());
    }

    #[tokio::test]
    async fn get_user_not_found() {
        let repo = Arc::new(MockUserRepository::new());
        let service = UserService::new(repo);

        let result = service.get_user("non-existent-id").await;
        assert!(matches!(result, Err(DomainError::NotFound)));
    }

    #[tokio::test]
    async fn update_user_name() {
        let repo = Arc::new(MockUserRepository::new());
        let service = UserService::new(repo);

        let created = service
            .create_user("test@example.com".to_string(), "Original Name".to_string())
            .await
            .unwrap();
        let updated = service
            .update_user_name(created.id(), "New Name".to_string())
            .await
            .unwrap();

        assert_eq!(updated.name(), "New Name");
        assert_eq!(updated.id(), created.id());
    }
}
