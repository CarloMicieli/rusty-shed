use super::entities::ManufacturerRow;
use crate::catalog::domain::manufacturer::Manufacturer;
use crate::catalog::domain::manufacturer_id::ManufacturerId;
use crate::catalog::domain::manufacturer_status::ManufacturerStatus;
use anyhow::anyhow;

/// Convert a `ManufacturerRow` (database representation) into the domain
/// `Manufacturer` type.
///
/// # Arguments
///
/// * `row` - The owned `ManufacturerRow` to convert. Fields are validated and
///   transformed into domain types (for example: parsing the ID and status).
///
/// # Returns
///
/// Returns `Ok(Manufacturer)` when conversion and validation succeed, or
/// `Err(anyhow::Error)` when validation fails (invalid id or status).
///
/// # Errors
///
/// Errors produced by underlying parsers/validators are propagated and wrapped
/// into an `anyhow::Error`.
impl TryFrom<ManufacturerRow> for Manufacturer {
    type Error = anyhow::Error;

    fn try_from(row: ManufacturerRow) -> Result<Self, Self::Error> {
        let id = ManufacturerId::try_from(row.id)
            .map_err(|e| anyhow!("invalid manufacturer id: {}", e))?;

        let status = row
            .status
            .parse::<ManufacturerStatus>()
            .map_err(|e| anyhow!("invalid manufacturer status: {}", e))?;

        Ok(Manufacturer {
            id,
            name: row.name,
            registered_company_name: row.registered_company_name,
            country_code: row.country_code,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::DateTime;
    use pretty_assertions::assert_eq;

    #[test]
    fn mapper_converts_row_to_domain() {
        use crate::catalog::infrastructure::entities::ManufacturerRow;
        use std::convert::TryFrom;

        let utc_timestamp = DateTime::from_timestamp(0, 0)
            .expect("invalid timestamp")
            .naive_utc();

        let row = ManufacturerRow {
            id: "MN-1".to_string(),
            name: "ACME Models".to_string(),
            registered_company_name: Some("ACME Corporation".to_string()),
            status: "ACTIVE".to_string(),
            country_code: Some("IT".to_string()),
            created_at: utc_timestamp,
            updated_at: utc_timestamp,
        };

        let domain = Manufacturer::try_from(row).expect("mapping should succeed");

        assert_eq!(&*domain.id, "MN-1");
        assert_eq!(domain.name, "ACME Models");
        assert_eq!(
            domain.registered_company_name.as_deref(),
            Some("ACME Corporation")
        );
    }
}
