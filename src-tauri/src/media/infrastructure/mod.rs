//! Infrastructure Layer
//!
//! Contains implementations for external services, file I/O, and data access.

pub mod image_repository;
pub mod placeholder_generator;

pub use image_repository::ImageRepository;
pub use placeholder_generator::PlaceholderGenerator;
