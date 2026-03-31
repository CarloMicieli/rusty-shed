use crate::data_management::domain::ValidationError;
use serde_json::Value;
use std::collections::HashSet;

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
    /// Returns `SchemaValidationError::Invalid` with the first failing constraint's path and
    /// kind, so callers can log actionable diagnostic information.
    pub fn validate(&self, manifest: &Value) -> Result<(), SchemaValidationError> {
        match jsonschema::validate(&self.schema, manifest) {
            Ok(()) => Ok(()),
            Err(error) => {
                let msg = format!(
                    "at '{}' (schema: '{}'): {:?}",
                    error.instance_path(),
                    error.schema_path(),
                    error.kind()
                );
                Err(SchemaValidationError::Invalid(msg))
            }
        }
    }

    /// Validate and extract structured validation errors.
    ///
    /// Returns a vector of `ValidationError` structs with path information.
    pub fn validate_detailed(&self, manifest: &Value) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        // First check schema conformance
        if !jsonschema::is_valid(&self.schema, manifest) {
            if let Err(error) = jsonschema::validate(&self.schema, manifest) {
                let validation_error = ValidationError::new(
                    error.instance_path().to_string(),
                    "schema_validation",
                    format!("{:?}", error.kind()),
                );
                errors.push(validation_error);
            } else {
                errors.push(ValidationError::new(
                    "",
                    "schema_validation",
                    "Validation failed".to_string(),
                ));
            }
        }

        // Then check referential integrity
        if let Err(mut ref_errors) = Self::validate_referential_integrity(manifest) {
            errors.append(&mut ref_errors);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Validate referential integrity between entities.
    ///
    /// Checks that all foreign keys reference existing entities.
    fn validate_referential_integrity(manifest: &Value) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        let data = match manifest.get("data") {
            Some(d) => d,
            None => return Ok(()), // Schema validation will catch this
        };

        // Build lookup sets for all entity IDs
        let manufacturer_ids = Self::extract_ids(data, "manufacturers");
        let railway_company_ids = Self::extract_ids(data, "railwayCompanies");
        let railway_model_ids = Self::extract_ids(data, "railwayModels");
        let collection_item_ids = Self::extract_ids(data, "collectionItems");
        let seller_ids = Self::extract_ids(data, "sellers");

        // Validate railway models reference valid manufacturers;
        // validate nested rolling stocks reference valid railway companies.
        if let Some(models) = data.get("railwayModels").and_then(|v| v.as_array()) {
            for (model_idx, model) in models.iter().enumerate() {
                if let Some(manufacturer_id) = model.get("manufacturerId").and_then(|v| v.as_str())
                    && !manufacturer_ids.contains(manufacturer_id)
                {
                    errors.push(ValidationError::new(
                        format!("data.railwayModels[{}].manufacturerId", model_idx),
                        "referential_integrity",
                        format!(
                            "Manufacturer '{}' not found in manufacturers list",
                            manufacturer_id
                        ),
                    ));
                }

                if let Some(rolling_stocks) = model.get("rollingStocks").and_then(|v| v.as_array())
                {
                    for (rs_idx, rs) in rolling_stocks.iter().enumerate() {
                        if let Some(rc_id) = rs.get("railwayCompanyId").and_then(|v| v.as_str())
                            && !railway_company_ids.contains(rc_id)
                        {
                            errors.push(ValidationError::new(
                                format!(
                                    "data.railwayModels[{}].rollingStocks[{}].railwayCompanyId",
                                    model_idx, rs_idx
                                ),
                                "referential_integrity",
                                format!(
                                    "Railway company '{}' not found in railwayCompanies list",
                                    rc_id
                                ),
                            ));
                        }
                    }
                }
            }
        }

        // Validate collection items reference valid railway models and sellers.
        if let Some(items) = data.get("collectionItems").and_then(|v| v.as_array()) {
            for (idx, item) in items.iter().enumerate() {
                if let Some(model_id) = item.get("railwayModelId").and_then(|v| v.as_str())
                    && !railway_model_ids.contains(model_id)
                {
                    errors.push(ValidationError::new(
                        format!("data.collectionItems[{}].railwayModelId", idx),
                        "referential_integrity",
                        format!(
                            "Railway model '{}' not found in railwayModels list",
                            model_id
                        ),
                    ));
                }

                if let Some(seller_id) = item
                    .get("purchase")
                    .and_then(|p| p.get("sellerId"))
                    .and_then(|v| v.as_str())
                    && !seller_ids.contains(seller_id)
                {
                    errors.push(ValidationError::new(
                        format!("data.collectionItems[{}].purchase.sellerId", idx),
                        "referential_integrity",
                        format!("Seller '{}' not found in sellers list", seller_id),
                    ));
                }
            }
        }

        // Validate maintenance cards reference valid collection items
        if let Some(cards) = data.get("maintenanceCards").and_then(|v| v.as_array()) {
            for (idx, card) in cards.iter().enumerate() {
                if let Some(item_id) = card.get("collectionItemId").and_then(|v| v.as_str())
                    && !collection_item_ids.contains(item_id)
                {
                    errors.push(ValidationError::new(
                        format!("data.maintenanceCards[{}].collectionItemId", idx),
                        "referential_integrity",
                        format!(
                            "Collection item '{}' not found in collectionItems list",
                            item_id
                        ),
                    ));
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Extract all IDs from an entity array.
    fn extract_ids(data: &Value, field: &str) -> HashSet<String> {
        data.get(field)
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| item.get("id").and_then(|id| id.as_str()))
                    .map(String::from)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Load the embedded manifest schema.
    fn load_embedded_schema() -> Result<Value, SchemaValidationError> {
        let schema_str = include_str!("../../../schemas/manifest_schema.json");
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
    fn test_schema_loads_successfully() {
        let result = SchemaValidator::new();
        assert!(result.is_ok(), "Embedded schema should load successfully");
    }

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
    fn test_invalid_missing_data() {
        let validator = SchemaValidator::new().expect("Schema should load");
        let manifest = json!({
            "version": "1.0"
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
                        "id": "trn:manufacturer:marklin",
                        "name": "Märklin"
                    }
                ]
            }
        });
        assert!(validator.validate(&manifest).is_ok());
    }

    #[test]
    fn test_valid_manifest_with_railway_companies() {
        let validator = SchemaValidator::new().expect("Schema should load");
        let manifest = json!({
            "version": "1.0",
            "data": {
                "railwayCompanies": [
                    {
                        "id": "trn:railway-company:deutsche-bahn",
                        "name": "Deutsche Bahn",
                        "countryCode": "DE"
                    }
                ]
            }
        });
        assert!(validator.validate(&manifest).is_ok());
    }

    #[test]
    fn test_validate_detailed_returns_errors() {
        let validator = SchemaValidator::new().expect("Schema should load");
        let manifest = json!({
            "data": {}
        });
        let result = validator.validate_detailed(&manifest);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_validate_detailed_passes_for_valid() {
        let validator = SchemaValidator::new().expect("Schema should load");
        let manifest = json!({
            "version": "1.0",
            "data": {}
        });
        let result = validator.validate_detailed(&manifest);
        assert!(result.is_ok());
    }

    #[test]
    fn test_default_validator() {
        let validator = SchemaValidator::default();
        let manifest = json!({
            "version": "1.0",
            "data": {}
        });
        assert!(validator.validate(&manifest).is_ok());
    }
}
