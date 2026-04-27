//! Infrastructure layer - implements adapters and configuration
//! This layer depends on DOMAIN (to implement ports), but domain has NO knowledge of this

pub mod adapters;
pub mod config;
pub mod mappers;
