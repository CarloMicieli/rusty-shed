//! Domain Layer
//!
//! Contains core business entities, value objects, and domain logic for the media module.

pub mod image_error;
pub mod image_placeholder;
pub mod image_validation;
pub mod railway_model_image;

pub use image_error::ImageError;
pub use image_placeholder::ImagePlaceholder;
pub use image_validation::{
    FileSize, ImageFormat, ImageValidator, MAX_FILE_SIZE_BYTES, ModelImagePath, StorageError,
    ValidationError,
};
pub use railway_model_image::RailwayModelImage;
