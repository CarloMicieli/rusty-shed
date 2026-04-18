//! Interface Layer
//!
//! Contains Tauri command handlers and DTOs for external communication.

pub mod command_handlers;
pub mod image_dto;

pub use command_handlers::{
    UploadModelImageArgs, UploadModelImageBytesArgs, get_image_path, get_railway_model_image,
    upload_model_image, upload_model_image_bytes,
};
pub use image_dto::RailwayModelImageResponse;
