//! Error types used by the core infrastructure layer.
//!
//! This module defines `CommandError`, an application-level error enum used by
//! command handlers and infrastructure components to represent database and
//! other execution errors in a serializable, human-friendly way.

use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::SystemTime;

use super::db::SqliteDbError;
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::validation::ValidationError;
use crate::data_management::domain::DataManagementError;
use garde::Report;
use serde::Serialize;

/// A session-scoped unique identifier for errors.
///
/// Format: `ERR-NNNN-X` where NNNN is a 4-digit number (1000–9999)
/// and X is an uppercase letter (A–Z).
pub struct ErrorId(String);

impl ErrorId {
    /// Generate a new unique Error ID from the current system time.
    ///
    /// Combines epoch milliseconds with a monotonic counter to ensure
    /// uniqueness even when called multiple times within the same millisecond.
    pub fn generate() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let millis = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        let seq = COUNTER.fetch_add(1, Ordering::Relaxed);
        let combined = millis.wrapping_mul(10_000).wrapping_add(seq);
        let n = (combined % 9000 + 1000) as u16;
        let c = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ"[(combined / 9000 % 26) as usize] as char;
        ErrorId(format!("ERR-{n:04}-{c}"))
    }
}

impl std::fmt::Display for ErrorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

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
    /// Contains a human-readable message and a unique Error ID for log correlation.
    /// Always construct via [`CommandError::unknown()`] to ensure logging.
    #[error("unknown error: {message}")]
    Unknown { message: String, error_id: String },

    /// Indicates a violation of a specific business invariant.
    ///
    /// **Source:** Triggered by Domain Entities or Use Cases (e.g.,
    /// "Cannot cancel an invoice that has already been paid").
    #[error("Business rule violation: {0}")]
    BusinessRule(String),

    /// Indicates a conflict with existing data (e.g., a unique constraint violation).
    #[error("Conflict: {0}")]
    Conflict(String),
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
            DomainError::Validation(msg) => {
                let mut map = HashMap::new();
                map.insert(
                    "_general".to_string(),
                    vec![ValidationError {
                        code: Cow::Borrowed("invalid"),
                        message: Some(Cow::Owned(msg)),
                        params: HashMap::new(),
                    }],
                );
                CommandError::ValidationError(map)
            }
            DomainError::Infrastructure(inner) => CommandError::DatabaseError(inner),
            DomainError::BusinessRule(msg) => CommandError::BusinessRule(msg),
            DomainError::ValidationError(errors) => CommandError::ValidationError(errors),
            DomainError::InvalidIdentifier(e) => {
                CommandError::validation_field("id", e.to_string())
            }
            DomainError::Conflict(msg) => CommandError::Conflict(msg),
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
        CommandError::unknown(err.to_string())
    }
}

/// Automatic conversion from data management domain errors.
impl From<DataManagementError> for CommandError {
    fn from(err: DataManagementError) -> Self {
        match err {
            DataManagementError::DatabaseError(msg) => CommandError::DatabaseError(msg),
            DataManagementError::NotFound(msg) => CommandError::NotFound(msg),
            DataManagementError::SchemaViolation(msg) => CommandError::BusinessRule(msg),
            DataManagementError::InvalidInput(msg) => CommandError::BusinessRule(msg),
            DataManagementError::ArchiveError(msg)
            | DataManagementError::IoError(msg)
            | DataManagementError::Unknown(msg) => CommandError::unknown(msg),
        }
    }
}

impl From<Report> for CommandError {
    fn from(report: Report) -> Self {
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

        CommandError::ValidationError(fields)
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

impl CommandError {
    /// Create an Unknown error with a generated Error ID and structured logging.
    ///
    /// This is the **only** correct way to construct `CommandError::Unknown`.
    /// It generates a unique Error ID, emits a structured log entry, and
    /// returns the error with the ID embedded for UI correlation.
    pub fn unknown(msg: impl Into<String>) -> Self {
        let id = ErrorId::generate();
        let message = msg.into();
        tracing::error!("Signal Fault: error_id={id}, message={message}");
        CommandError::Unknown {
            message,
            error_id: id.to_string(),
        }
    }

    /// Helper to create a validation error for a single field.
    pub fn validation_field(field: impl Into<String>, error: impl Into<String>) -> Self {
        let mut fields = HashMap::new();
        fields.insert(
            field.into(),
            vec![ValidationError {
                code: Cow::Borrowed("invalid"),
                message: Some(Cow::Owned(error.into())),
                params: HashMap::new(),
            }],
        );
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
            .map(|(k, v)| {
                (
                    k.into(),
                    vec![ValidationError {
                        code: Cow::Borrowed("invalid"),
                        message: Some(Cow::Owned(v.into())),
                        params: HashMap::new(),
                    }],
                )
            })
            .collect();
        CommandError::ValidationError(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use garde::Validate;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use std::collections::HashMap as StdHashMap;
    use std::collections::HashSet;

    fn validate_coded_error(value: &str, _: &()) -> garde::Result {
        if value == "ok" {
            Ok(())
        } else {
            Err(garde::Error::new("error_invalid_magic"))
        }
    }

    #[derive(garde::Validate)]
    struct GardeMappingFixture {
        #[garde(custom(validate_coded_error))]
        coded: String,
        #[garde(length(min = 3))]
        generic: String,
    }

    // --- ErrorId Tests ---

    #[test]
    fn test_error_id_format() {
        let id = ErrorId::generate().to_string();
        let parts: Vec<&str> = id.split('-').collect();
        assert_eq!(parts.len(), 3, "Expected 3 parts in ID: {id}");
        assert_eq!(parts[0], "ERR", "Expected 'ERR' prefix: {id}");
        assert_eq!(parts[1].len(), 4, "Expected 4-digit numeric segment: {id}");
        assert!(
            parts[1].chars().all(|c| c.is_ascii_digit()),
            "Expected digits in numeric segment: {id}"
        );
        assert_eq!(parts[2].len(), 1, "Expected single letter suffix: {id}");
        assert!(
            parts[2].chars().all(|c| c.is_ascii_uppercase()),
            "Expected uppercase letter: {id}"
        );
    }

    #[test]
    fn test_error_id_numeric_range() {
        for _ in 0..100 {
            let id = ErrorId::generate().to_string();
            // Format is ERR-NNNN-X; numeric segment occupies chars 4..8
            let numeric: u16 = id[4..8].parse().expect("Expected numeric segment");
            assert!(
                (1000..=9999).contains(&numeric),
                "Numeric segment out of range: {numeric}"
            );
        }
    }

    #[test]
    fn test_error_id_uniqueness() {
        let ids: HashSet<String> = (0..500).map(|_| ErrorId::generate().to_string()).collect();
        assert_eq!(ids.len(), 500, "Expected 500 unique IDs, got {}", ids.len());
    }

    #[test]
    fn test_unknown_factory_sets_error_id() {
        let err = CommandError::unknown("test error");
        match err {
            CommandError::Unknown { error_id, .. } => {
                assert!(!error_id.is_empty(), "error_id should not be empty");
            }
            _ => panic!("Expected Unknown variant"),
        }
    }

    // --- DomainError / CommandError Tests ---

    #[test]
    fn it_should_test_domain_error_to_command_error_not_found() {
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
    fn it_should_test_command_error_validation_field() {
        let command_error = CommandError::validation_field("email", "Invalid format");
        match command_error {
            CommandError::ValidationError(map) => {
                assert!(map.contains_key("email"));
                let errs = &map["email"];
                assert_eq!(errs.len(), 1);
                assert_eq!(errs[0].message.as_deref(), Some("Invalid format"));
            }
            _ => panic!("Expected ValidationError variant"),
        }
    }

    #[test]
    fn it_should_test_command_error_validation_fields() {
        let fields = vec![("email", "Invalid format"), ("age", "Must be at least 18")];
        let command_error = CommandError::validation_fields(fields);
        match command_error {
            CommandError::ValidationError(map) => {
                assert!(map.contains_key("email"));
                assert!(map.contains_key("age"));
                assert_eq!(map["email"][0].message.as_deref(), Some("Invalid format"));
                assert_eq!(
                    map["age"][0].message.as_deref(),
                    Some("Must be at least 18")
                );
            }
            _ => panic!("Expected ValidationError variant"),
        }
    }

    #[test]
    fn domain_validation_error_maps_to_general_key() {
        let domain_error = DomainError::Validation("some validation message".to_string());
        let cmd: CommandError = domain_error.into();
        match cmd {
            CommandError::ValidationError(map) => {
                assert!(map.contains_key("_general"), "_general key must be present");
                let errs = &map["_general"];
                assert_eq!(errs.len(), 1);
                assert_eq!(errs[0].message.as_deref(), Some("some validation message"));
            }
            _ => panic!("Expected ValidationError variant"),
        }
    }

    #[test]
    fn it_should_test_domain_error_to_command_error_business_rule() {
        let domain_error = DomainError::BusinessRule("Cannot delete paid invoice".to_string());
        let command_error: CommandError = domain_error.into();
        match command_error {
            CommandError::BusinessRule(msg) => {
                assert_eq!(msg, "Cannot delete paid invoice");
            }
            _ => panic!("Expected BusinessRule variant"),
        }
    }

    #[rstest]
    #[case::not_found("not_found")]
    #[case::validation("validation")]
    #[case::infrastructure("infrastructure")]
    #[case::business_rule("business_rule")]
    #[case::validation_error("validation_error")]
    fn parameterized_domain_to_command_conversion(#[case] case_name: &str) {
        let domain_error = match case_name {
            "not_found" => DomainError::NotFound {
                resource: "Item".to_string(),
                identifier: "id-42".to_string(),
            },
            "validation" => DomainError::Validation("bad input".to_string()),
            "infrastructure" => DomainError::Infrastructure("row not found".to_string()),
            "business_rule" => DomainError::BusinessRule("some rule".to_string()),
            "validation_error" => {
                let mut map: StdHashMap<String, Vec<ValidationError>> = StdHashMap::new();
                map.insert(
                    "field".to_string(),
                    vec![ValidationError {
                        code: std::borrow::Cow::Borrowed("required"),
                        message: Some(std::borrow::Cow::Borrowed("is required")),
                        params: std::collections::HashMap::new(),
                    }],
                );
                DomainError::ValidationError(map)
            }
            _ => panic!("unknown case"),
        };

        let cmd: CommandError = domain_error.into();

        match case_name {
            "not_found" => match cmd {
                CommandError::NotFound(msg) => {
                    assert!(msg.contains("Item with identifier 'id-42' not found"))
                }
                _ => panic!("expected NotFound"),
            },
            "validation" => match cmd {
                CommandError::ValidationError(map) => {
                    assert!(map.contains_key("_general"), "_general key must be present");
                    assert!(!map["_general"].is_empty());
                }
                _ => panic!("expected ValidationError"),
            },
            "infrastructure" => match cmd {
                CommandError::DatabaseError(s) => assert!(!s.is_empty()),
                _ => panic!("expected DatabaseError"),
            },
            "business_rule" => match cmd {
                CommandError::BusinessRule(msg) => assert_eq!(msg, "some rule"),
                _ => panic!("expected BusinessRule"),
            },
            "validation_error" => match cmd {
                CommandError::ValidationError(map) => assert!(map.contains_key("field")),
                _ => panic!("expected ValidationError"),
            },
            _ => unreachable!(),
        }
    }

    #[test]
    fn garde_report_keeps_machine_code_and_uses_invalid_fallback() {
        let fixture = GardeMappingFixture {
            coded: "bad".to_string(),
            generic: "x".to_string(),
        };

        let report = fixture.validate().expect_err("fixture should be invalid");
        let cmd = CommandError::from(report);

        match cmd {
            CommandError::ValidationError(map) => {
                let coded = &map["coded"][0];
                assert_eq!(coded.code, "error_invalid_magic");
                assert_eq!(coded.message, None);

                let generic = &map["generic"][0];
                assert_eq!(generic.code, "invalid");
                assert!(generic.message.is_some());
            }
            _ => panic!("Expected ValidationError variant"),
        }
    }
}
