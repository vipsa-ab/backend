//! User query handlers - application layer for read operations

use crate::domain::entities::User;
use crate::domain::services::UserService;
use std::sync::Arc;

#[allow(dead_code)]
pub struct UserQueries {
    #[allow(dead_code)]
    service: Arc<UserService>,
}

#[allow(dead_code)]
impl UserQueries {
    #[allow(dead_code)]
    pub fn new(service: Arc<UserService>) -> Self {
        Self { service }
    }

    #[allow(dead_code)]
    pub async fn get_user(&self, id: &str) -> Result<User, String> {
        self.service.get_user(id).await.map_err(|e| e.to_string())
    }

    #[allow(dead_code)]
    pub async fn list_users(&self) -> Result<Vec<User>, String> {
        self.service.list_users().await.map_err(|e| e.to_string())
    }
}
