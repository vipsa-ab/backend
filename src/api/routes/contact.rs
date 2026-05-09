use axum::{extract::State, http::StatusCode, Json, Router};

use super::booking::AppState;
use crate::api::dtos::{ApiResponse, ContactRequest};

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/contact", axum::routing::post(create_contact))
        .with_state(state)
}

async fn create_contact(
    State(state): State<AppState>,
    Json(payload): Json<ContactRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let subject = format!("Kontakt: {}", payload.name);

    let body = format!(
        r#"<html>
<body style="font-family: Arial, sans-serif; line-height: 1.6; color: #333; max-width: 600px; margin: 0 auto; padding: 20px;">
    <h2 style="color: #2c3e50; border-bottom: 2px solid #3498db; padding-bottom: 10px;">Ny Kontaktförfrågan</h2>
    
    <h3 style="color: #3498db;">Kontaktinformation</h3>
    <table style="width: 100%; border-collapse: collapse;">
        <tr>
            <td style="padding: 8px; font-weight: bold;">Namn:</td>
            <td style="padding: 8px;">{}</td>
        </tr>
        <tr style="background-color: #f9f9f9;">
            <td style="padding: 8px; font-weight: bold;">E-post:</td>
            <td style="padding: 8px;"><a href="mailto:{}">{}</a></td>
        </tr>
        <tr>
            <td style="padding: 8px; font-weight: bold;">Telefon:</td>
            <td style="padding: 8px;">{}</td>
        </tr>
        <tr style="background-color: #f9f9f9;">
            <td style="padding: 8px; font-weight: bold;">Tjänst:</td>
            <td style="padding: 8px;">{}</td>
        </tr>
    </table>

    <h3 style="color: #3498db;">Meddelande</h3>
    <div style="background-color: #f9f9f9; padding: 15px; border-left: 4px solid #3498db; margin-top: 10px;">
        {}
    </div>
</body>
</html>"#,
        payload.name, payload.email, payload.email, payload.phone, payload.service, payload.message,
    );

    state
        .email_port
        .send_email("info@vipsa.se", &subject, &body)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse::success(serde_json::json!({
        "message": "Contact request sent successfully"
    }))))
}
