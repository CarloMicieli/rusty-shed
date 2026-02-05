use serde::{Deserialize, Serialize};
use specta::Type;

/// A validation error that blocks import.
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ValidationError {
    /// JSON path to the error (e.g., "data.railwayModels[3].productCode")
    pub path: String,
    /// Error code for i18n lookup
    pub code: String,
    /// Human-readable message
    pub message: String,
}

impl ValidationError {
    /// Create a new validation error.
    pub fn new(
        path: impl Into<String>,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            path: path.into(),
            code: code.into(),
            message: message.into(),
        }
    }

    /// Create a missing required field error.
    pub fn missing_field(path: &str, field_name: &str) -> Self {
        Self::new(
            format!("{}.{}", path, field_name),
            "missing_required_field",
            format!("Field '{}' is required", field_name),
        )
    }

    /// Create an invalid enum value error.
    pub fn invalid_enum(path: &str, value: &str, allowed: &[&str]) -> Self {
        Self::new(
            path,
            "invalid_enum_value",
            format!("Invalid value '{}'. Allowed: {}", value, allowed.join(", ")),
        )
    }

    /// Create an orphaned reference error.
    pub fn orphaned_reference(path: &str, referenced_id: &str) -> Self {
        Self::new(
            path,
            "orphaned_reference",
            format!("Referenced ID '{}' not found in data", referenced_id),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_validation_error() {
        let err = ValidationError::new("data.models[0]", "test_code", "Test message");
        assert_eq!(err.path, "data.models[0]");
        assert_eq!(err.code, "test_code");
        assert_eq!(err.message, "Test message");
    }

    #[test]
    fn test_missing_field_error() {
        let err = ValidationError::missing_field("data.models[0]", "productCode");
        assert_eq!(err.path, "data.models[0].productCode");
        assert_eq!(err.code, "missing_required_field");
    }

    #[test]
    fn test_invalid_enum_error() {
        let err = ValidationError::invalid_enum("data.models[0].scale", "XL", &["H0", "N", "TT"]);
        assert!(err.message.contains("XL"));
        assert!(err.message.contains("H0"));
    }

    #[test]
    fn test_orphaned_reference_error() {
        let err = ValidationError::orphaned_reference("data.models[0].manufacturerId", "mfr-999");
        assert_eq!(err.code, "orphaned_reference");
        assert!(err.message.contains("mfr-999"));
    }
}
