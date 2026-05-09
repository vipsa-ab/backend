//! Email port - domain defines what it needs for email sending
//! Implementation is in infrastructure (Resend adapter)

use std::future::Future;
use std::pin::Pin;

pub trait EmailPort: Send + Sync {
    fn send_email(
        &self,
        to: &str,
        subject: &str,
        body: &str,
    ) -> Pin<Box<dyn Future<Output = Result<(), EmailError>> + Send + '_>>;
}

#[derive(Debug, Clone)]
pub struct EmailError(pub String);

impl std::fmt::Display for EmailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Email error: {}", self.0)
    }
}

impl std::error::Error for EmailError {}
