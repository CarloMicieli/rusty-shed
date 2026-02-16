//! Infrastructure layer for settings
//!
//! Contains repository implementations using tauri-plugin-store.

pub mod os_language;
pub mod store_repository;

pub use store_repository::{SettingsRepository, StoreSettingsRepository};
