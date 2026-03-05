use crate::core::domain::identifiers::IdParseError;
use crate::core::domain::validation::ValidationError;
use std::collections::HashMap;

/// This enum categorizes errors to help the Interface layer decide
/// how to present failures to the user.
#[derive(thiserror::Error, Debug)]
pub enum DomainError {
    /// Indicates that the input data was structurally or logically invalid.
    ///
    /// **Source:** Usually triggered during the mapping from a Command DTO
    /// to a Domain Params struct (e.g., an invalid email format or empty item list).
    #[error("Validation failed: {0}")]
    Validation(String),

    /// Indicates a failure within the persistence or infrastructure layer.
    ///
    /// **Source:** Triggered by the [`InvoiceRepository`] during database
    /// operations like unique constraint violations or connection timeouts.
    ///
    /// *Note:* In production, the raw error should be logged, but a generic
    /// message may be shown to the user for security.
    #[error("Internal persistence error: {0}")]
    Infrastructure(#[from] sqlx::Error),

    /// Indicates that a requested resource was not found.
    ///
    /// **Source:** Triggered by Use Cases or Repositories when a specific
    /// ID does not exist in the system.
    #[error("Resource not found: {resource}")]
    NotFound {
        /// The type of resource (e.g., "Invoice")
        resource: String,
        /// The identifier that was searched for
        identifier: String,
    },

    /// Indicates a violation of a specific business invariant.
    ///
    /// **Source:** Triggered by Domain Entities or Use Cases (e.g.,
    /// "Cannot cancel an invoice that has already been paid").
    #[error("Business rule violation: {0}")]
    BusinessRule(String),

    /// Validation error with field-specific messages.
    ///
    /// The map contains field names as keys and error messages as values.
    /// This allows the frontend to display validation errors next to the appropriate form fields.
    /// Example: `{"email": "Invalid email format", "age": "Must be at least 18"}`
    #[error("validation error: {0:?}")]
    ValidationError(HashMap<String, Vec<ValidationError>>),

    /// Indicates an invalid identifier format or parsing error.
    ///
    /// **Source:** Triggered when attempting to parse a string into an identifier
    /// and the format is invalid or the prefix doesn't match.
    #[error("Invalid identifier: {0}")]
    InvalidIdentifier(#[from] IdParseError),

    /// Indicates a conflict with existing data (e.g., a unique constraint violation).
    ///
    /// **Source:** Triggered when an operation would produce a duplicate entry.
    #[error("Conflict: {0}")]
    Conflict(String),
}
