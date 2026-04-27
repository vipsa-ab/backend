//! User command handlers - application layer orchestration
//! These are simple pass-throughs in this architecture, but allow for future complex workflows

use crate::domain::entities::User;
use crate::domain::services::UserService;
use std::sync::Arc;

#[allow(dead_code)]
pub struct UserCommands {
    #[allow(dead_code)]
    service: Arc<UserService>,
}

#[allow(dead_code)]
impl UserCommands {
    pub fn new(service: Arc<UserService>) -> Self {
        Self { service }
    }

    #[allow(dead_code)]
    pub async fn create_user(&self, email: String, name: String) -> Result<User, String> {
        self.service
            .create_user(email, name)
            .await
            .map_err(|e| e.to_string())
    }
}
