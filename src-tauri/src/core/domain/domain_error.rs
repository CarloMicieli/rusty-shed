use crate::core::domain::identifiers::IdParseError;
use crate::core::domain::validation::ValidationError;
use std::borrow::Cow;
use std::collections::HashMap;

/// This enum categorizes errors to help the Interface layer decide
/// how to present failures to the user.
#[derive(thiserror::Error, Debug)]
pub enum DomainError {
    /// Indicates that the input data was structurally or logically invalid.
    ///
    /// **Source:** Usually triggered during the mapping from a Command DTO
    /// to a Domain Params struct (e.g., an invalid email format or empty item list).
    ///
    /// Prefer [`DomainError::validation_general`] for new code, which stores the
    /// message in the structured `ValidationError` map under `_general` and is
    /// consistent with the field-level `ValidationError` shape the frontend expects.
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
    Infrastructure(String),

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

impl DomainError {
    /// Creates a structured validation error with a single general message stored
    /// under the `_general` sentinel key.
    ///
    /// Prefer this over `DomainError::Validation(msg)` for new code: it produces
    /// a `ValidationError` map that the frontend can display consistently alongside
    /// field-specific validation errors.
    pub fn validation_general(msg: impl Into<String>) -> Self {
        let mut map = HashMap::new();
        map.insert(
            "_general".to_string(),
            vec![ValidationError {
                code: Cow::Borrowed("invalid"),
                message: Some(Cow::Owned(msg.into())),
                params: HashMap::new(),
            }],
        );
        DomainError::ValidationError(map)
    }
}

impl From<garde::Report> for DomainError {
    fn from(report: garde::Report) -> Self {
        let mut fields: HashMap<String, Vec<ValidationError>> = HashMap::new();

        for (path, error) in report.into_inner() {
            let raw = error.to_string();
            let code = extract_machine_validation_code(&raw)
                .map(Cow::Owned)
                .unwrap_or(Cow::Borrowed("invalid"));

            let message = if code.as_ref() == raw {
                None
            } else {
                Some(Cow::Owned(raw))
            };

            fields
                .entry(path.to_string())
                .or_default()
                .push(ValidationError {
                    code,
                    message,
                    params: HashMap::new(),
                });
        }

        DomainError::ValidationError(fields)
    }
}

fn extract_machine_validation_code(input: &str) -> Option<String> {
    let candidate = input.trim();
    if candidate.starts_with("error_")
        && candidate
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    {
        Some(candidate.to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use garde::Validate;

    fn validate_magic(value: &str, _: &()) -> garde::Result {
        if value == "ok" {
            Ok(())
        } else {
            Err(garde::Error::new("error_invalid_magic"))
        }
    }

    #[derive(Validate)]
    struct InvalidFixture {
        #[garde(custom(validate_magic))]
        coded: String,
        #[garde(length(min = 3))]
        generic: String,
    }

    #[test]
    fn garde_report_maps_to_structured_validation_error() {
        let fixture = InvalidFixture {
            coded: "bad".to_string(),
            generic: "x".to_string(),
        };

        let report = fixture.validate().expect_err("fixture should be invalid");
        let err = DomainError::from(report);

        match err {
            DomainError::ValidationError(map) => {
                assert_eq!(map["coded"][0].code, "error_invalid_magic");
                assert!(map["coded"][0].message.is_none());

                assert_eq!(map["generic"][0].code, "invalid");
                assert!(map["generic"][0].message.is_some());
            }
            other => panic!("expected ValidationError, got {other:?}"),
        }
    }
}
