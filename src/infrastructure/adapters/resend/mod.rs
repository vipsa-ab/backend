use crate::domain::services::notification::{EmailError, EmailPort};
use resend_rs::types::CreateEmailBaseOptions;
use resend_rs::Resend;
use std::future::Future;
use std::pin::Pin;

pub struct ResendEmailAdapter {
    client: Resend,
    from_email: String,
}

impl ResendEmailAdapter {
    pub fn new(api_key: &str, from_email: String) -> Self {
        Self {
            client: Resend::new(api_key),
            from_email,
        }
    }
}

impl EmailPort for ResendEmailAdapter {
    fn send_email(
        &self,
        to: &str,
        subject: &str,
        body: &str,
    ) -> Pin<Box<dyn Future<Output = Result<(), EmailError>> + Send + '_>> {
        let client = self.client.clone();
        let from = self.from_email.clone();
        let to_string = to.to_string();
        let subject_string = subject.to_string();
        let body_string = body.to_string();

        Box::pin(async move {
            let email = CreateEmailBaseOptions::new(from, vec![to_string], subject_string)
                .with_text(&body_string);

            client
                .emails
                .send(email)
                .await
                .map_err(|e| EmailError::Unknown(format!("Resend error: {}", e)))?;
            Ok(())
        })
    }
}
