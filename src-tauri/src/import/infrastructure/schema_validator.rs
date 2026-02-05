use crate::import::domain::ValidationError;
use serde_json::Value;

/// Error type for schema validation operations.
#[derive(Debug, Clone)]
pub enum SchemaValidationError {
    /// Schema validation failed with details
    Invalid(String),
    /// Schema loading/parsing error
    LoadError(String),
}

impl std::fmt::Display for SchemaValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(msg) => write!(f, "Schema validation failed: {}", msg),
            Self::LoadError(msg) => write!(f, "Schema load error: {}", msg),
        }
    }
}

impl std::error::Error for SchemaValidationError {}

/// Validates manifest JSON against the embedded schema.
#[derive(Debug, Clone)]
pub struct SchemaValidator {
    schema: Value,
}

impl SchemaValidator {
    /// Create a new schema validator with the embedded manifest schema.
    ///
    /// # Errors
    /// Returns `SchemaValidationError::LoadError` if the embedded schema is invalid.
    pub fn new() -> Result<Self, SchemaValidationError> {
        let schema = Self::load_embedded_schema()?;
        Ok(Self { schema })
    }

    /// Validate a manifest document against the schema.
    ///
    /// # Errors
    /// Returns `SchemaValidationError::Invalid` if the document does not conform to the schema.
    pub fn validate(&self, manifest: &Value) -> Result<(), SchemaValidationError> {
        // Use is_valid for simple true/false validation
        if jsonschema::is_valid(&self.schema, manifest) {
            Ok(())
        } else {
            Err(SchemaValidationError::Invalid(
                "Manifest does not conform to schema".to_string(),
            ))
        }
    }

    /// Validate and extract structured validation errors.
    ///
    /// Returns a vector of `ValidationError` structs with path information.
    pub fn validate_detailed(&self, manifest: &Value) -> Result<(), Vec<ValidationError>> {
        // First check if valid
        if jsonschema::is_valid(&self.schema, manifest) {
            return Ok(());
        }

        // If not valid, try to get error details
        if let Err(error) = jsonschema::validate(&self.schema, manifest) {
            let validation_error = ValidationError::new(
                error.instance_path.to_string(),
                "schema_validation",
                format!("{:?}", error.kind),
            );
            Err(vec![validation_error])
        } else {
            Err(vec![ValidationError::new(
                "",
                "schema_validation",
                "Validation failed".to_string(),
            )])
        }
    }

    /// Load the embedded manifest schema.
    fn load_embedded_schema() -> Result<Value, SchemaValidationError> {
        let schema_str = include_str!("../domain/manifest_schema.json");
        serde_json::from_str(schema_str)
            .map_err(|e| SchemaValidationError::LoadError(format!("Failed to parse schema: {}", e)))
    }
}

impl Default for SchemaValidator {
    fn default() -> Self {
        // This should not panic in normal operation as the schema is embedded
        Self::new().expect("Embedded schema should always be valid")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_valid_minimal_manifest() {
        let validator = SchemaValidator::new().expect("Schema should load");
        let manifest = json!({
            "version": "1.0",
            "data": {}
        });
        assert!(validator.validate(&manifest).is_ok());
    }

    #[test]
    fn test_invalid_missing_version() {
        let validator = SchemaValidator::new().expect("Schema should load");
        let manifest = json!({
            "data": {}
        });
        assert!(validator.validate(&manifest).is_err());
    }

    #[test]
    fn test_invalid_version_value() {
        let validator = SchemaValidator::new().expect("Schema should load");
        let manifest = json!({
            "version": "2.0",
            "data": {}
        });
        assert!(validator.validate(&manifest).is_err());
    }

    #[test]
    fn test_valid_manifest_with_manufacturers() {
        let validator = SchemaValidator::new().expect("Schema should load");
        let manifest = json!({
            "version": "1.0",
            "data": {
                "manufacturers": [
                    {
                        "id": "mfr1",
                        "name": "Märklin"
                    }
                ]
            }
        });
        assert!(validator.validate(&manifest).is_ok());
    }
}
