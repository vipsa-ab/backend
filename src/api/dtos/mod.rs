//! API DTOs - Data Transfer Objects for the REST API

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub timestamp: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
        }
    }
}

// ============================================
// Booking DTOs
// ============================================

#[derive(Debug, Deserialize, Serialize)]
pub struct AddonsDto {
    pub ironing: AddonOptionDto,
    pub windows: AddonOptionDto,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddonOptionDto {
    pub enabled: bool,
    pub hours: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BookingRequest {
    pub service: String,
    pub size: i32,
    pub rooms: i32,
    pub frequency: String,
    pub hours: i32,
    pub auto_hours: bool,
    pub date: String,
    pub time_slot: String,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub customer_type: String,
    pub personal_number: String,
    pub organisation_number: String,
    pub address: String,
    pub addons: AddonsDto,
}

// ============================================
// Contact DTOs
// ============================================

#[derive(Debug, Deserialize, Serialize)]
pub struct ContactRequest {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub service: String,
    pub message: String,
}
