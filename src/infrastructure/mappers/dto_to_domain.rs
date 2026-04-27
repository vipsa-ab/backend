//! DTO -> Domain mappers
//! These convert external DTOs (from API) to domain types for commands/queries

use crate::api::dtos::{CreateNotificationDto, CreateUserDto, UpdateUserDto};

/// Creates a CreateUser command from the DTO
/// This ensures the domain never directly depends on API DTOs
pub fn create_user_from_dto(dto: &CreateUserDto) -> (String, String) {
    (dto.email.clone(), dto.name.clone())
}

pub fn update_user_from_dto(dto: &UpdateUserDto) -> String {
    dto.name.clone()
}

pub fn create_notification_from_dto(dto: &CreateNotificationDto) -> (String, String, String) {
    (dto.user_id.clone(), dto.subject.clone(), dto.body.clone())
}
