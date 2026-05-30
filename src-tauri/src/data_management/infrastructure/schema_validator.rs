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
        let owned_rolling_stock_ids = Self::extract_ids(data, "ownedRollingStocks");
        let seller_ids = Self::extract_ids(data, "sellers");

        Self::validate_railway_model_references(
            data,
            &manufacturer_ids,
            &railway_company_ids,
            &mut errors,
        );
        Self::validate_collection_item_references(
            data,
            &railway_model_ids,
            &seller_ids,
            &mut errors,
        );
        Self::validate_maintenance_card_references(
            data,
            &collection_item_ids,
            &owned_rolling_stock_ids,
            &mut errors,
        );
        Self::validate_digital_rolling_stock_references(
            data,
            &owned_rolling_stock_ids,
            &mut errors,
        );
        Self::validate_formation_element_references(data, &owned_rolling_stock_ids, &mut errors);

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn validate_railway_model_references(
        data: &Value,
        manufacturer_ids: &HashSet<String>,
        railway_company_ids: &HashSet<String>,
        errors: &mut Vec<ValidationError>,
    ) {
        let Some(models) = data.get("railwayModels").and_then(|v| v.as_array()) else {
            return;
        };

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

            let Some(rolling_stocks) = model.get("rollingStocks").and_then(|v| v.as_array()) else {
                continue;
            };

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

    fn validate_collection_item_references(
        data: &Value,
        railway_model_ids: &HashSet<String>,
        seller_ids: &HashSet<String>,
        errors: &mut Vec<ValidationError>,
    ) {
        let Some(items) = data.get("collectionItems").and_then(|v| v.as_array()) else {
            return;
        };

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

    fn validate_maintenance_card_references(
        data: &Value,
        collection_item_ids: &HashSet<String>,
        owned_rolling_stock_ids: &HashSet<String>,
        errors: &mut Vec<ValidationError>,
    ) {
        let Some(cards) = data.get("maintenanceCards").and_then(|v| v.as_array()) else {
            return;
        };

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

            if let Some(owned_id) = card.get("ownedRollingStockId").and_then(|v| v.as_str())
                && !owned_rolling_stock_ids.contains(owned_id)
            {
                errors.push(ValidationError::new(
                    format!("data.maintenanceCards[{}].ownedRollingStockId", idx),
                    "referential_integrity",
                    format!(
                        "Owned rolling stock '{}' not found in ownedRollingStocks list",
                        owned_id
                    ),
                ));
            }
        }
    }

    fn validate_digital_rolling_stock_references(
        data: &Value,
        owned_rolling_stock_ids: &HashSet<String>,
        errors: &mut Vec<ValidationError>,
    ) {
        let Some(roster) = data.get("digitalRollingStocks").and_then(|v| v.as_array()) else {
            return;
        };

        for (idx, item) in roster.iter().enumerate() {
            if let Some(owned_id) = item.get("ownedRollingStockId").and_then(|v| v.as_str())
                && !owned_rolling_stock_ids.contains(owned_id)
            {
                errors.push(ValidationError::new(
                    format!("data.digitalRollingStocks[{}].ownedRollingStockId", idx),
                    "referential_integrity",
                    format!(
                        "Owned rolling stock '{}' not found in ownedRollingStocks list",
                        owned_id
                    ),
                ));
            }
        }
    }

    fn validate_formation_element_references(
        data: &Value,
        owned_rolling_stock_ids: &HashSet<String>,
        errors: &mut Vec<ValidationError>,
    ) {
        let Some(formations) = data.get("trainFormations").and_then(|v| v.as_array()) else {
            return;
        };

        for (formation_idx, formation) in formations.iter().enumerate() {
            let Some(elements) = formation.get("elements").and_then(|v| v.as_array()) else {
                continue;
            };

            for (element_idx, element) in elements.iter().enumerate() {
                if let Some(owned_id) = element.get("ownedRollingStockId").and_then(|v| v.as_str())
                    && !owned_rolling_stock_ids.contains(owned_id)
                {
                    errors.push(ValidationError::new(
                        format!(
                            "data.trainFormations[{}].elements[{}].ownedRollingStockId",
                            formation_idx, element_idx
                        ),
                        "referential_integrity",
                        format!(
                            "Owned rolling stock '{}' not found in ownedRollingStocks list",
                            owned_id
                        ),
                    ));
                }
            }
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

    #[test]
    fn test_validate_referential_integrity_passes_when_links_exist() {
        let manifest = json!({
            "data": {
                "manufacturers": [{ "id": "trn:manufacturer:acme" }],
                "railwayCompanies": [{ "id": "trn:railway-company:fs" }],
                "railwayModels": [{
                    "id": "trn:railway-model:acme:100",
                    "manufacturerId": "trn:manufacturer:acme",
                    "rollingStocks": [{ "railwayCompanyId": "trn:railway-company:fs" }]
                }],
                "sellers": [{ "id": "trn:seller:shop" }],
                "collectionItems": [{
                    "id": "trn:collection-item:1",
                    "railwayModelId": "trn:railway-model:acme:100",
                    "purchase": { "sellerId": "trn:seller:shop" }
                }],
                "ownedRollingStocks": [{ "id": "trn:owned-rolling-stock:1" }],
                "maintenanceCards": [{
                    "collectionItemId": "trn:collection-item:1",
                    "ownedRollingStockId": "trn:owned-rolling-stock:1"
                }],
                "digitalRollingStocks": [{
                    "id": "trn:digital-rolling-stock:1",
                    "ownedRollingStockId": "trn:owned-rolling-stock:1",
                    "dccAddress": 3
                }],
                "trainFormations": [{
                    "id": "trn:train-formation:1",
                    "name": "Formation",
                    "elements": [{
                        "id": "trn:formation-element:1",
                        "prototypeId": "trn:prototype:1",
                        "ownedRollingStockId": "trn:owned-rolling-stock:1",
                        "positionOrder": 0,
                        "tractionOverride": 0
                    }]
                }]
            }
        });

        let result = SchemaValidator::validate_referential_integrity(&manifest);
        assert!(result.is_ok(), "expected no referential integrity errors");
    }

    #[test]
    fn test_validate_referential_integrity_collects_all_missing_references() {
        let manifest = json!({
            "data": {
                "manufacturers": [],
                "railwayCompanies": [],
                "railwayModels": [{
                    "id": "trn:railway-model:missing:1",
                    "manufacturerId": "trn:manufacturer:missing",
                    "rollingStocks": [{ "railwayCompanyId": "trn:railway-company:missing" }]
                }],
                "collectionItems": [{
                    "id": "trn:collection-item:missing",
                    "railwayModelId": "trn:railway-model:unknown",
                    "purchase": { "sellerId": "trn:seller:missing" }
                }],
                "maintenanceCards": [{
                    "collectionItemId": "trn:collection-item:unknown",
                    "ownedRollingStockId": "trn:owned-rolling-stock:unknown"
                }],
                "digitalRollingStocks": [{
                    "id": "trn:digital-rolling-stock:missing",
                    "ownedRollingStockId": "trn:owned-rolling-stock:unknown",
                    "dccAddress": 3
                }],
                "trainFormations": [{
                    "id": "trn:train-formation:missing",
                    "name": "Broken Formation",
                    "elements": [{
                        "id": "trn:formation-element:missing",
                        "prototypeId": "trn:prototype:missing",
                        "ownedRollingStockId": "trn:owned-rolling-stock:unknown",
                        "positionOrder": 0,
                        "tractionOverride": 0
                    }]
                }]
            }
        });

        let errors = SchemaValidator::validate_referential_integrity(&manifest)
            .expect_err("expected referential integrity errors");

        assert_eq!(errors.len(), 8);
        assert!(
            errors
                .iter()
                .any(|e| e.path == "data.railwayModels[0].manufacturerId")
        );
        assert!(
            errors
                .iter()
                .any(|e| e.path == "data.railwayModels[0].rollingStocks[0].railwayCompanyId")
        );
        assert!(
            errors
                .iter()
                .any(|e| e.path == "data.collectionItems[0].railwayModelId")
        );
        assert!(
            errors
                .iter()
                .any(|e| e.path == "data.collectionItems[0].purchase.sellerId")
        );
        assert!(
            errors
                .iter()
                .any(|e| e.path == "data.maintenanceCards[0].collectionItemId")
        );
        assert!(
            errors
                .iter()
                .any(|e| e.path == "data.maintenanceCards[0].ownedRollingStockId")
        );
        assert!(
            errors
                .iter()
                .any(|e| e.path == "data.digitalRollingStocks[0].ownedRollingStockId")
        );
        assert!(
            errors
                .iter()
                .any(|e| e.path == "data.trainFormations[0].elements[0].ownedRollingStockId")
        );
    }

    #[test]
    fn test_validate_referential_integrity_skips_when_no_data_section() {
        let manifest = json!({ "version": "1.0" });
        let result = SchemaValidator::validate_referential_integrity(&manifest);
        assert!(result.is_ok());
    }
}
