use super::entities::ManufacturerRow;
use super::entities::RailwayCompanyRow;
use crate::catalog::domain::manufacturer::Manufacturer;
use crate::catalog::domain::manufacturer_id::ManufacturerId;
use crate::catalog::domain::manufacturer_status::ManufacturerStatus;
use crate::catalog::domain::period_of_activity::PeriodOfActivity;
use crate::catalog::domain::railway_company::RailwayCompany;
use crate::catalog::domain::railway_company_id::RailwayCompanyId;
use crate::catalog::domain::railway_status::RailwayStatus;
use anyhow::anyhow;
use chrono::NaiveDate;

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

/// Convert a `RailwayCompanyRow` (database representation) into the domain
/// `RailwayCompany` type.
///
/// # Arguments
///
/// * `row` - The owned `RailwayCompanyRow` to convert. Fields are validated
///   and transformed into domain types (for example: parsing the ID).
///
/// # Returns
///
/// Returns `Ok(RailwayCompany)` when conversion and validation succeed, or
/// `Err(anyhow::Error)` when validation fails (invalid id or period of activity).
///
/// # Errors
///
/// Errors produced by underlying parsers/validators are propagated and wrapped
/// into an `anyhow::Error`.
impl TryFrom<RailwayCompanyRow> for RailwayCompany {
    type Error = anyhow::Error;

    fn try_from(row: RailwayCompanyRow) -> Result<Self, Self::Error> {
        let id = RailwayCompanyId::try_from(row.id)
            .map_err(|e| anyhow!("invalid railway company id: {e}"))?;

        // 1. Capture presence flags before moving row fields
        let has_status = row.status.is_some();
        let has_since = row.operating_since.is_some();
        let has_until = row.operating_until.is_some();

        // 2. Parse values
        let status = row
            .status
            .map(|s| s.parse::<RailwayStatus>())
            .transpose()
            .map_err(|e| anyhow!("invalid railway status: {e}"))?
            .unwrap_or(RailwayStatus::Active);

        let operating_since = row
            .operating_since
            .map(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d"))
            .transpose()
            .map_err(|e| anyhow!("invalid operating_since date: {e}"))?;

        let operating_until = row
            .operating_until
            .map(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d"))
            .transpose()
            .map_err(|e| anyhow!("invalid operating_until date: {e}"))?;

        // 3. Build the period only if at least one field existed in the DB
        let period_of_activity =
            (has_status || has_since || has_until).then_some(PeriodOfActivity {
                status,
                operating_since,
                operating_until,
            });

        Ok(RailwayCompany {
            id,
            name: row.name,
            registered_company_name: row.registered_company_name,
            country_code: row.country_code,
            period_of_activity,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod manufacturer_mapper_tests {
        use super::*;
        use crate::catalog::infrastructure::entities::ManufacturerRow;
        use chrono::DateTime;
        use pretty_assertions::assert_eq;
        use std::convert::TryFrom;

        #[test]
        fn mapper_converts_row_to_domain() {
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

        mod railway_mapper_tests {
            use super::*;
            use crate::catalog::infrastructure::entities::RailwayCompanyRow;
            use chrono::DateTime;
            use pretty_assertions::assert_eq;
            use std::convert::TryFrom;

            #[test]
            fn railway_mapper_converts_row_to_domain() {
                let utc_timestamp = DateTime::from_timestamp(0, 0)
                    .expect("invalid timestamp")
                    .naive_utc();

                let row = RailwayCompanyRow {
                    id: "RC-1".to_string(),
                    name: "Ferrovie dello Stato".to_string(),
                    registered_company_name: Some("FS S.p.A.".to_string()),
                    country_code: Some("IT".to_string()),
                    status: Some("ACTIVE".to_string()),
                    operating_since: None,
                    operating_until: None,
                    created_at: utc_timestamp,
                    updated_at: utc_timestamp,
                };

                let domain = RailwayCompany::try_from(row).expect("mapping should succeed");

                assert_eq!(&*domain.id, "RC-1");
                assert_eq!(domain.name, "Ferrovie dello Stato");
                assert_eq!(domain.registered_company_name.as_deref(), Some("FS S.p.A."));
                assert_eq!(
                    domain.period_of_activity,
                    Some(PeriodOfActivity {
                        operating_since: None,
                        operating_until: None,
                        status: RailwayStatus::Active,
                    })
                );
            }
        }
    }
}
