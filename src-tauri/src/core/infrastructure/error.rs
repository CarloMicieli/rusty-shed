//! Error types used by the core infrastructure layer.
//!
//! This module defines `CommandError`, an application-level error enum used by
//! command handlers and infrastructure components to represent database and
//! other execution errors in a serializable, human-friendly way.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::db::SqliteDbError;

/// Application-level error returned by command handlers in the core infrastructure.
///
/// This enum provides structured error variants that allow the frontend to react
/// intelligently to different error conditions. Each variant is serializable and
/// includes enough context for appropriate UI rendering.
#[derive(thiserror::Error, Debug, Serialize, Deserialize, specta::Type)]
pub enum CommandError {
    /// Represents an error coming from the database layer.
    ///
    /// The inner `String` should contain a concise, non-sensitive description
    /// of the underlying database failure.
    #[error("database error: {0}")]
    DatabaseError(String),

    /// Indicates a requested resource was not found.
    ///
    /// Use this when a query returns no results for a specific ID or criteria.
    /// The inner `String` should describe what resource was not found (e.g., "Wishlist not found").
    #[error("not found: {0}")]
    NotFound(String),

    /// Validation error with field-specific messages.
    ///
    /// The map contains field names as keys and error messages as values.
    /// This allows the frontend to display validation errors next to the appropriate form fields.
    /// Example: `{"email": "Invalid email format", "age": "Must be at least 18"}`
    #[error("validation error: {0:?}")]
    ValidationError(HashMap<String, String>),

    /// Permission denied for the requested operation.
    ///
    /// Use this when the user lacks sufficient privileges to perform an action.
    #[error("permission denied: {0}")]
    PermissionDenied(String),

    /// A catch-all for unexpected errors that don't map to a specific variant.
    ///
    /// The inner `String` can include a short debug message suitable for
    /// logging; avoid placing secrets here.
    #[error("unknown error: {0}")]
    Unknown(String),
}

/// Automatic conversion from database errors.
///
/// This allows using `?` operator directly on database operations without manual `.map_err()`.
impl From<SqliteDbError> for CommandError {
    fn from(err: SqliteDbError) -> Self {
        CommandError::DatabaseError(err.to_string())
    }
}

/// Automatic conversion from sqlx errors.
///
/// This allows using `?` operator directly on sqlx operations without manual `.map_err()`.
impl From<sqlx::Error> for CommandError {
    fn from(err: sqlx::Error) -> Self {
        CommandError::DatabaseError(err.to_string())
    }
}

/// Automatic conversion from anyhow errors.
///
/// This allows using `?` operator directly on anyhow operations without manual `.map_err()`.
impl From<anyhow::Error> for CommandError {
    fn from(err: anyhow::Error) -> Self {
        CommandError::Unknown(err.to_string())
    }
}

impl CommandError {
    /// Helper to create a validation error for a single field.
    ///
    /// # Example
    /// ```
    /// use rusty_shed_lib::core::infrastructure::error::CommandError;
    /// let err = CommandError::validation_field("email", "Invalid email format");
    /// ```
    pub fn validation_field(field: impl Into<String>, message: impl Into<String>) -> Self {
        let mut fields = HashMap::new();
        fields.insert(field.into(), message.into());
        CommandError::ValidationError(fields)
    }

    /// Helper to create a validation error from multiple field errors.
    ///
    /// # Example
    /// ```
    /// use rusty_shed_lib::core::infrastructure::error::CommandError;
    /// let err = CommandError::validation_fields([
    ///     ("email", "Invalid email format"),
    ///     ("age", "Must be at least 18"),
    /// ]);
    /// ```
    pub fn validation_fields<I, K, V>(fields: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        let map = fields
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        CommandError::ValidationError(map)
    }
}
