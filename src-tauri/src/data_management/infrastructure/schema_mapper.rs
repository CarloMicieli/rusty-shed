use crate::data_management::domain::DataManagementError;
use tracing::warn;

/// Convert schema category value (camelCase) to DB value (SCREAMING_SNAKE_CASE).
pub(crate) fn schema_category_to_db(
    schema_value: &str,
) -> Result<&'static str, DataManagementError> {
    match schema_value {
        "LOCOMOTIVES" => Ok("LOCOMOTIVES"),
        "TRAIN_SETS" => Ok("TRAIN_SETS"),
        "STARTER_SETS" => Ok("STARTER_SETS"),
        "FREIGHT_CARS" => Ok("FREIGHT_CARS"),
        "PASSENGER_CARS" => Ok("PASSENGER_CARS"),
        "ELECTRIC_MULTIPLE_UNITS" => Ok("ELECTRIC_MULTIPLE_UNITS"),
        "RAILCARS" => Ok("RAILCARS"),
        "locomotive" => Ok("LOCOMOTIVES"),
        "trainSet" => Ok("TRAIN_SETS"),
        "freightCar" => Ok("FREIGHT_CARS"),
        "passengerCar" => Ok("PASSENGER_CARS"),
        "electricMultipleUnit" => Ok("ELECTRIC_MULTIPLE_UNITS"),
        "railcar" => Ok("RAILCARS"),
        other => Err(DataManagementError::SchemaViolation(format!(
            "Unknown category: '{other}'"
        ))),
    }
}

/// Convert schema power method value (lowercase) to DB value (SCREAMING_SNAKE_CASE).
pub(crate) fn schema_power_method_to_db(
    schema_value: &str,
) -> Result<&'static str, DataManagementError> {
    match schema_value {
        "AC" => Ok("AC"),
        "DC" => Ok("DC"),
        "TRIX_EXPRESS" => Ok("TRIX_EXPRESS"),
        "ac" => Ok("AC"),
        "dc" => Ok("DC"),
        "trixExpress" => Ok("TRIX_EXPRESS"),
        other => Err(DataManagementError::SchemaViolation(format!(
            "Unknown power method: '{other}'"
        ))),
    }
}

/// Convert schema seller type (camelCase/lowercase) to DB value (SCREAMING_SNAKE_CASE).
pub(crate) fn schema_seller_type_to_db(
    schema_value: &str,
) -> Result<&'static str, DataManagementError> {
    match schema_value {
        "SHOP" => Ok("SHOP"),
        "PRIVATE" => Ok("PRIVATE"),
        "MARKETPLACE" => Ok("MARKETPLACE"),
        "DISTRIBUTOR" => Ok("DISTRIBUTOR"),
        "shop" => Ok("SHOP"),
        "private" => Ok("PRIVATE"),
        "marketplace" => Ok("MARKETPLACE"),
        "distributor" => Ok("DISTRIBUTOR"),
        other => Err(DataManagementError::SchemaViolation(format!(
            "Unknown seller type: '{other}'"
        ))),
    }
}

pub(crate) fn schema_manufacturer_status_to_db(
    schema_value: Option<&str>,
) -> Result<&'static str, DataManagementError> {
    match schema_value.unwrap_or("ACTIVE") {
        "ACTIVE" | "active" => Ok("ACTIVE"),
        "MERGED" | "merged" => Ok("MERGED"),
        "OUT_OF_BUSINESS" | "outOfBusiness" => Ok("OUT_OF_BUSINESS"),
        other => Err(DataManagementError::SchemaViolation(format!(
            "Unknown manufacturer status: '{other}'"
        ))),
    }
}

pub(crate) fn schema_railway_company_status_to_db(
    schema_value: Option<&str>,
) -> Result<&'static str, DataManagementError> {
    match schema_value.unwrap_or("ACTIVE") {
        "ACTIVE" | "active" => Ok("ACTIVE"),
        "INACTIVE" | "inactive" => Ok("INACTIVE"),
        "MERGED" | "merged" => Ok("MERGED"),
        other => Err(DataManagementError::SchemaViolation(format!(
            "Unknown railway company status: '{other}'"
        ))),
    }
}

pub(crate) fn schema_purchase_condition_to_db(
    schema_value: Option<&str>,
) -> Result<Option<&'static str>, DataManagementError> {
    match schema_value {
        None => Ok(None),
        Some("NEW") | Some("new") => Ok(Some("NEW")),
        Some("PRE_OWNED") | Some("preowned") | Some("used") => Ok(Some("PRE_OWNED")),
        Some(other) => Err(DataManagementError::SchemaViolation(format!(
            "Unknown purchase condition: '{other}'"
        ))),
    }
}

pub(crate) fn schema_model_condition_to_db(
    schema_value: Option<&str>,
) -> Result<Option<&'static str>, DataManagementError> {
    match schema_value {
        None => Ok(None),
        Some("MINT") | Some("mint") => Ok(Some("MINT")),
        Some("NEAR_MINT") => Ok(Some("NEAR_MINT")),
        Some("EXCELLENT") | Some("excellent") => Ok(Some("EXCELLENT")),
        Some("VERY_GOOD") => Ok(Some("VERY_GOOD")),
        Some("GOOD") | Some("good") => Ok(Some("GOOD")),
        Some("FAIR") | Some("fair") => Ok(Some("FAIR")),
        Some("POOR") | Some("poor") => Ok(Some("POOR")),
        Some("FOR_PARTS") => Ok(Some("FOR_PARTS")),
        Some(other) => Err(DataManagementError::SchemaViolation(format!(
            "Unknown model condition: '{other}'"
        ))),
    }
}

pub(crate) fn schema_box_condition_to_db(
    schema_value: Option<&str>,
) -> Result<Option<&'static str>, DataManagementError> {
    match schema_value {
        None => Ok(None),
        Some("ORIGINAL_MINT") | Some("mint") => Ok(Some("ORIGINAL_MINT")),
        Some("ORIGINAL_GOOD") | Some("good") => Ok(Some("ORIGINAL_GOOD")),
        Some("ORIGINAL_WORN") | Some("damaged") => Ok(Some("ORIGINAL_WORN")),
        Some("REPLACEMENT_BOX") => Ok(Some("REPLACEMENT_BOX")),
        Some("NO_BOX") | Some("missing") => Ok(Some("NO_BOX")),
        Some(other) => Err(DataManagementError::SchemaViolation(format!(
            "Unknown box condition: '{other}'"
        ))),
    }
}

pub(crate) fn schema_purchase_type_to_db(
    schema_value: &str,
) -> Result<&'static str, DataManagementError> {
    match schema_value {
        "purchased" | "PURCHASED" => Ok("PURCHASED"),
        "sold" | "SOLD" => Ok("SOLD"),
        "preOrdered" | "preordered" | "PRE_ORDERED" => Ok("PRE_ORDERED"),
        other => Err(DataManagementError::SchemaViolation(format!(
            "Unknown purchase type: '{other}'"
        ))),
    }
}

/// Convert schema maintenance type to DB value (SCREAMING_SNAKE_CASE).
///
/// Uses a lenient fallback: unknown values map to "OTHER" with a warning,
/// rather than failing the import.
pub(crate) fn schema_maintenance_type_to_db(
    schema_value: &str,
) -> Result<&'static str, DataManagementError> {
    match schema_value {
        "cleaning" => Ok("WHEEL_CLEANING"),
        "lubrication" => Ok("LUBRICATION"),
        "repair" => Ok("OTHER"),
        "modification" => Ok("WEATHERING"),
        "inspection" => Ok("GENERAL_INSPECTION"),
        other => {
            warn!(
                "Unknown maintenance type '{}', defaulting to 'OTHER'",
                other
            );
            Ok("OTHER")
        }
    }
}

/// Map railway model DB category (plural) to rolling stock DB category (singular).
pub(crate) fn model_category_to_rolling_stock_category(db_category: &str) -> &'static str {
    match db_category {
        "FREIGHT_CARS" => "FREIGHT_CAR",
        "PASSENGER_CARS" => "PASSENGER_CAR",
        "ELECTRIC_MULTIPLE_UNITS" => "ELECTRIC_MULTIPLE_UNIT",
        "RAILCARS" => "RAILCAR",
        _ => "LOCOMOTIVE", // LOCOMOTIVES, TRAIN_SETS, STARTER_SETS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_category_to_db() {
        assert_eq!(schema_category_to_db("LOCOMOTIVES").unwrap(), "LOCOMOTIVES");
        assert_eq!(schema_category_to_db("locomotive").unwrap(), "LOCOMOTIVES");
        assert_eq!(schema_category_to_db("freightCar").unwrap(), "FREIGHT_CARS");
        assert_eq!(
            schema_category_to_db("passengerCar").unwrap(),
            "PASSENGER_CARS"
        );
        assert_eq!(
            schema_category_to_db("electricMultipleUnit").unwrap(),
            "ELECTRIC_MULTIPLE_UNITS"
        );
        assert_eq!(schema_category_to_db("railcar").unwrap(), "RAILCARS");
        assert_eq!(schema_category_to_db("trainSet").unwrap(), "TRAIN_SETS");
        assert!(schema_category_to_db("unknown").is_err());
    }

    #[test]
    fn test_schema_power_method_to_db() {
        assert_eq!(schema_power_method_to_db("AC").unwrap(), "AC");
        assert_eq!(schema_power_method_to_db("ac").unwrap(), "AC");
        assert_eq!(schema_power_method_to_db("dc").unwrap(), "DC");
        assert_eq!(
            schema_power_method_to_db("trixExpress").unwrap(),
            "TRIX_EXPRESS"
        );
        assert!(schema_power_method_to_db("unknown").is_err());
    }

    #[test]
    fn test_schema_seller_type_to_db() {
        assert_eq!(schema_seller_type_to_db("SHOP").unwrap(), "SHOP");
        assert_eq!(schema_seller_type_to_db("shop").unwrap(), "SHOP");
        assert_eq!(schema_seller_type_to_db("private").unwrap(), "PRIVATE");
        assert_eq!(
            schema_seller_type_to_db("marketplace").unwrap(),
            "MARKETPLACE"
        );
        assert_eq!(
            schema_seller_type_to_db("distributor").unwrap(),
            "DISTRIBUTOR"
        );
        assert!(schema_seller_type_to_db("unknown").is_err());
    }

    #[test]
    fn test_model_category_to_rolling_stock_category() {
        assert_eq!(
            model_category_to_rolling_stock_category("LOCOMOTIVES"),
            "LOCOMOTIVE"
        );
        assert_eq!(
            model_category_to_rolling_stock_category("FREIGHT_CARS"),
            "FREIGHT_CAR"
        );
        assert_eq!(
            model_category_to_rolling_stock_category("PASSENGER_CARS"),
            "PASSENGER_CAR"
        );
        assert_eq!(
            model_category_to_rolling_stock_category("ELECTRIC_MULTIPLE_UNITS"),
            "ELECTRIC_MULTIPLE_UNIT"
        );
        assert_eq!(
            model_category_to_rolling_stock_category("RAILCARS"),
            "RAILCAR"
        );
        assert_eq!(
            model_category_to_rolling_stock_category("TRAIN_SETS"),
            "LOCOMOTIVE"
        );
    }

    #[test]
    fn test_schema_status_mappings() {
        assert_eq!(schema_manufacturer_status_to_db(None).unwrap(), "ACTIVE");
        assert_eq!(
            schema_manufacturer_status_to_db(Some("outOfBusiness")).unwrap(),
            "OUT_OF_BUSINESS"
        );
        assert_eq!(
            schema_railway_company_status_to_db(Some("INACTIVE")).unwrap(),
            "INACTIVE"
        );
        assert_eq!(
            schema_railway_company_status_to_db(Some("MERGED")).unwrap(),
            "MERGED"
        );
    }

    #[test]
    fn test_schema_condition_mappings() {
        assert_eq!(
            schema_purchase_condition_to_db(Some("used")).unwrap(),
            Some("PRE_OWNED")
        );
        assert_eq!(
            schema_model_condition_to_db(Some("VERY_GOOD")).unwrap(),
            Some("VERY_GOOD")
        );
        assert_eq!(
            schema_box_condition_to_db(Some("missing")).unwrap(),
            Some("NO_BOX")
        );
    }

    #[test]
    fn test_schema_purchase_type_mapping() {
        assert_eq!(
            schema_purchase_type_to_db("purchased").unwrap(),
            "PURCHASED"
        );
        assert_eq!(
            schema_purchase_type_to_db("preOrdered").unwrap(),
            "PRE_ORDERED"
        );
    }
}
