//! Settings module for Tauri 2 settings management
//!
//! This module implements settings persistence using tauri-plugin-store
//! and provides reactive settings management for the application.
//!
//! This replaces the old SQLite-based settings system. No migration is performed.

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod interface;

// Re-export main types and commands for convenience
pub use crate::core::domain::Language;
pub use domain::user_settings::{MeasureUnit, PowerSystem, UserSettings};
pub use interface::commands::{get_settings, initialize_settings, update_settings};

// Placeholder for backward compatibility (removed old ensure_default_settings)
// The new system uses initialize_settings instead
pub async fn ensure_default_settings(
    _pool: &sqlx::SqlitePool,
) -> Result<(), crate::core::infrastructure::error::CommandError> {
    // No-op: new settings system doesn't use SQLite
    Ok(())
}
