use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Entity not found")]
    NotFound,
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Domain logic error: {0}")]
    BusinessRule(String),
}

pub type DomainResult<T> = Result<T, DomainError>;

#[async_trait]
pub trait Repository<T: Entity>: Send + Sync {
    async fn save(&self, entity: T) -> DomainResult<T>;
    async fn find_by_id(&self, id: &str) -> DomainResult<Option<T>>;
    async fn find_all(&self) -> DomainResult<Vec<T>>;
    async fn delete(&self, id: &str) -> DomainResult<()>;
}

pub trait Entity: Send + Sync {
    fn id(&self) -> &str;
}
