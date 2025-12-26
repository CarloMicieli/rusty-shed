//! Error types used by the core infrastructure layer.
//!
//! This module defines `CommandError`, an application-level error enum used by
//! command handlers and infrastructure components to represent database and
//! other execution errors in a serializable, human-friendly way.

use serde::{Deserialize, Serialize};

/// Application-level error returned by command handlers in the core infrastructure.
///
/// Variants are simple wrappers around strings to keep boundaries between
/// infrastructure and domain code straightforward. Prefer constructing
/// `CommandError::DatabaseError` when an underlying DB call fails, and
/// `CommandError::Unknown` for unexpected failures.
#[derive(thiserror::Error, Debug, Serialize, Deserialize, specta::Type)]
pub enum CommandError {
    /// Represents an error coming from the database layer.
    ///
    /// The inner `String` should contain a concise, non-sensitive description
    /// of the underlying database failure.
    #[error("database error: {0}")]
    DatabaseError(String),

    /// A catch-all for unexpected errors that don't map to a specific variant.
    ///
    /// The inner `String` can include a short debug message suitable for
    /// logging; avoid placing secrets here.
    #[error("unknown error: {0}")]
    Unknown(String),
}
