//! Error types used by the core infrastructure layer.
//!
//! This module defines `CommandError`, an application-level error enum used by
//! command handlers and infrastructure components to represent database and
//! other execution errors in a serializable, human-friendly way.

use std::collections::HashMap;

use super::db::SqliteDbError;
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::validation::ValidationError;
use serde::Serialize;

/// Application-level error returned by command handlers in the core infrastructure.
///
/// This enum provides structured error variants that allow the frontend to react
/// intelligently to different error conditions. Each variant is serializable and
/// includes enough context for appropriate UI rendering.
#[derive(thiserror::Error, Debug, Serialize, specta::Type)]
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
    ValidationError(HashMap<String, Vec<ValidationError>>),

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

    /// Indicates a violation of a specific business invariant.
    ///
    /// **Source:** Triggered by Domain Entities or Use Cases (e.g.,
    /// "Cannot cancel an invoice that has already been paid").
    #[error("Business rule violation: {0}")]
    BusinessRule(String),
}

impl From<DomainError> for CommandError {
    fn from(error: DomainError) -> Self {
        match error {
            DomainError::NotFound {
                resource,
                identifier,
            } => CommandError::NotFound(format!(
                "{} with identifier '{}' not found",
                resource, identifier
            )),
            DomainError::Validation(_) => CommandError::ValidationError(HashMap::new()),
            DomainError::Infrastructure(inner) => CommandError::DatabaseError(inner.to_string()),
            DomainError::BusinessRule(msg) => CommandError::BusinessRule(msg),
            DomainError::ValidationError(errors) => CommandError::ValidationError(errors),
        }
    }
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
    pub fn validation_field(field: impl Into<String>, _error: impl Into<String>) -> Self {
        let mut fields = HashMap::new();
        fields.insert(field.into(), Vec::new());
        CommandError::ValidationError(fields)
    }

    /// Helper to create a validation error from multiple field errors.
    pub fn validation_fields<I, K, V>(fields: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        let map = fields
            .into_iter()
            .map(|(k, _v)| (k.into(), Vec::new()))
            .collect();
        CommandError::ValidationError(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_domain_error_to_command_error_not_found() {
        let domain_error = DomainError::NotFound {
            resource: "Wishlist".to_string(),
            identifier: "123".to_string(),
        };
        let command_error: CommandError = domain_error.into();
        match command_error {
            CommandError::NotFound(msg) => {
                assert_eq!(msg, "Wishlist with identifier '123' not found");
            }
            _ => panic!("Expected NotFound variant"),
        }
    }

    #[test]
    fn test_command_error_validation_field() {
        let command_error = CommandError::validation_field("email", "Invalid format");
        match command_error {
            CommandError::ValidationError(map) => {
                assert!(map.contains_key("email"));
            }
            _ => panic!("Expected ValidationError variant"),
        }
    }

    #[test]
    fn test_command_error_validation_fields() {
        let fields = vec![("email", "Invalid format"), ("age", "Must be at least 18")];
        let command_error = CommandError::validation_fields(fields);
        match command_error {
            CommandError::ValidationError(map) => {
                assert!(map.contains_key("email"));
                assert!(map.contains_key("age"));
            }
            _ => panic!("Expected ValidationError variant"),
        }
    }

    #[test]
    fn test_domain_error_to_command_error_business_rule() {
        let domain_error = DomainError::BusinessRule("Cannot delete paid invoice".to_string());
        let command_error: CommandError = domain_error.into();
        match command_error {
            CommandError::BusinessRule(msg) => {
                assert_eq!(msg, "Cannot delete paid invoice");
            }
            _ => panic!("Expected BusinessRule variant"),
        }
    }
}
