//! Interface Layer
//!
//! Contains Tauri command handlers and DTOs for external communication.

pub mod command_handlers;
pub mod image_dto;

pub use command_handlers::get_railway_model_image;
pub use image_dto::RailwayModelImageResponse;
