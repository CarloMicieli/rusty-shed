use crate::data_management::domain::DataManagementError;
use log::warn;

/// Convert schema category value (camelCase) to DB value (SCREAMING_SNAKE_CASE).
pub(crate) fn schema_category_to_db(
    schema_value: &str,
) -> Result<&'static str, DataManagementError> {
    match schema_value {
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
        "shop" => Ok("SHOP"),
        "private" => Ok("PRIVATE"),
        "marketplace" => Ok("MARKETPLACE"),
        "distributor" => Ok("DISTRIBUTOR"),
        other => Err(DataManagementError::SchemaViolation(format!(
            "Unknown seller type: '{other}'"
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
}
