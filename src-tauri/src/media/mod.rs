//! Media Module
//!
//! This module handles image management for railway models, including:
//! - Image retrieval from the filesystem
//! - Placeholder generation when no image is available
//! - Path validation for security
//!
//! The module follows Domain-Driven Design (DDD) architecture:
//! - **Domain**: Core entities and value objects
//! - **Application**: Use cases and business workflows
//! - **Infrastructure**: File I/O, external services
//! - **Interface**: Tauri commands and DTOs

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod interface;
