use axum::{extract::State, http::StatusCode, Json, Router};
use std::sync::Arc;

use crate::api::dtos::{ApiResponse, BookingRequest};
use crate::domain::services::EmailPort;

#[derive(Clone)]
pub struct AppState {
    pub email_port: Arc<dyn EmailPort>,
}

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/booking", axum::routing::post(create_booking))
        .with_state(state)
}

async fn create_booking(
    State(state): State<AppState>,
    Json(payload): Json<BookingRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let subject = format!("Boka Städ: {}", payload.name);

    let body = format!(
        r#"<html>
<body style="font-family: Arial, sans-serif; line-height: 1.6; color: #333; max-width: 600px; margin: 0 auto; padding: 20px;">
    <h2 style="color: #2c3e50; border-bottom: 2px solid #3498db; padding-bottom: 10px;">Ny Bokning</h2>
    
    <h3 style="color: #3498db;">Kundinformation</h3>
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
            <td style="padding: 8px; font-weight: bold;">Kundtyp:</td>
            <td style="padding: 8px;">{}</td>
        </tr>
        <tr>
            <td style="padding: 8px; font-weight: bold;">Personnummer:</td>
            <td style="padding: 8px;">{}</td>
        </tr>
        <tr style="background-color: #f9f9f9;">
            <td style="padding: 8px; font-weight: bold;">Organisationsnummer:</td>
            <td style="padding: 8px;">{}</td>
        </tr>
    </table>

    <h3 style="color: #3498db;">Bokningsdetaljer</h3>
    <table style="width: 100%; border-collapse: collapse;">
        <tr>
            <td style="padding: 8px; font-weight: bold;">Tjänst:</td>
            <td style="padding: 8px;">{}</td>
        </tr>
        <tr style="background-color: #f9f9f9;">
            <td style="padding: 8px; font-weight: bold;">Storlek (m²):</td>
            <td style="padding: 8px;">{}</td>
        </tr>
        <tr>
            <td style="padding: 8px; font-weight: bold;">Antal rum:</td>
            <td style="padding: 8px;">{}</td>
        </tr>
        <tr style="background-color: #f9f9f9;">
            <td style="padding: 8px; font-weight: bold;">Frekvens:</td>
            <td style="padding: 8px;">{}</td>
        </tr>
        <tr>
            <td style="padding: 8px; font-weight: bold;">Timmar:</td>
            <td style="padding: 8px;">{}</td>
        </tr>
        <tr style="background-color: #f9f9f9;">
            <td style="padding: 8px; font-weight: bold;">Automatisk tid:</td>
            <td style="padding: 8px;">{}</td>
        </tr>
        <tr>
            <td style="padding: 8px; font-weight: bold;">Datum:</td>
            <td style="padding: 8px;">{}</td>
        </tr>
        <tr style="background-color: #f9f9f9;">
            <td style="padding: 8px; font-weight: bold;">Tid:</td>
            <td style="padding: 8px;">{}</td>
        </tr>
        <tr>
            <td style="padding: 8px; font-weight: bold;">Adress:</td>
            <td style="padding: 8px;">{}</td>
        </tr>
    </table>

    <h3 style="color: #3498db;">Tillägg</h3>
    <table style="width: 100%; border-collapse: collapse;">
        <tr>
            <td style="padding: 8px; font-weight: bold;">Strykjärn:</td>
            <td style="padding: 8px;">{} ({} tim)</td>
        </tr>
        <tr style="background-color: #f9f9f9;">
            <td style="padding: 8px; font-weight: bold;">Fönster:</td>
            <td style="padding: 8px;">{} ({} tim)</td>
        </tr>
    </table>
</body>
</html>"#,
        payload.name,
        payload.email,
        payload.email,
        payload.phone,
        payload.customer_type,
        payload.personal_number,
        payload.organisation_number,
        payload.service,
        payload.size,
        payload.rooms,
        payload.frequency,
        payload.hours,
        if payload.auto_hours { "Ja" } else { "Nej" },
        payload.date,
        payload.time_slot,
        payload.address,
        if payload.addons.ironing.enabled {
            "Ja"
        } else {
            "Nej"
        },
        payload.addons.ironing.hours,
        if payload.addons.windows.enabled {
            "Ja"
        } else {
            "Nej"
        },
        payload.addons.windows.hours,
    );

    state
        .email_port
        .send_email("info@vipsa.se", &subject, &body)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse::success(serde_json::json!({
        "message": "Booking created and email sent successfully"
    }))))
}
