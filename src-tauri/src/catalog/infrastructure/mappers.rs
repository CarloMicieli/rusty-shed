use super::entities::RailwayCompanyRow;
use super::entities::{ManufacturerRow, RailwayModelRow, RollingStockRow};
use crate::catalog::domain::category::RollingStockCategory;
use crate::catalog::domain::control::Control;
use crate::catalog::domain::dcc_interface::DccInterface;
use crate::catalog::domain::delivery_date::DeliveryDate;
use crate::catalog::domain::epoch::{Epoch, EpochKind};
use crate::catalog::domain::length_over_buffers::LengthOverBuffers;
use crate::catalog::domain::manufacturer::Manufacturer;
use crate::catalog::domain::manufacturer_id::ManufacturerId;
use crate::catalog::domain::manufacturer_status::ManufacturerStatus;
use crate::catalog::domain::period_of_activity::PeriodOfActivity;
use crate::catalog::domain::product_code::ProductCode;
use crate::catalog::domain::railway_company::RailwayCompany;
use crate::catalog::domain::railway_company_id::RailwayCompanyId;
use crate::catalog::domain::railway_model::RailwayModel;
use crate::catalog::domain::railway_model_id::RailwayModelId;
use crate::catalog::domain::railway_status::RailwayStatus;
use crate::catalog::domain::rolling_stock::RollingStock;
use crate::catalog::domain::rolling_stock_id::RollingStockId;
use crate::catalog::domain::rolling_stock_railway::RollingStockRailway;
use crate::catalog::domain::scale::Scale;
use crate::catalog::domain::technical_specifications::TechnicalSpecifications;
use anyhow::anyhow;
use chrono::NaiveDate;
use url::Url;

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

        let website_url: Option<Url> = match row.website_url {
            Some(s) => {
                if s.trim().is_empty() {
                    None
                } else {
                    Some(Url::parse(&s).map_err(|e| anyhow!("invalid website_url: {}", e))?)
                }
            }
            None => None,
        };

        Ok(Manufacturer {
            id,
            name: row.name,
            registered_company_name: row.registered_company_name,
            country_code: row.country_code,
            status,
            website_url,
        })
    }
}

impl TryFrom<RailwayModelRow> for RailwayModel {
    type Error = anyhow::Error;

    fn try_from(row: RailwayModelRow) -> Result<Self, Self::Error> {
        let id = RailwayModelId::try_from(row.id)
            .map_err(|e| anyhow!("invalid railway model id: {}", e))?;

        let product_code = ProductCode::try_from(row.product_code)
            .map_err(|e| anyhow!("invalid product code: {}", e))?;

        let scale =
            Scale::try_from(row.scale.as_str()).map_err(|e| anyhow!("invalid scale: {}", e))?;

        let epoch_kind =
            EpochKind::try_from(row.epoch.as_str()).map_err(|e| anyhow!("invalid epoch: {}", e))?;
        let epoch = Epoch::from(epoch_kind);

        let delivery_date = match row.delivery_date {
            Some(s) => {
                Some(DeliveryDate::parse(&s).map_err(|e| anyhow!("invalid delivery_date: {}", e))?)
            }
            None => None,
        };

        Ok(RailwayModel {
            id,
            manufacturer: row.manufacturer_id,
            product_code,
            description: row.description,
            details: row.details,
            power_method: row.power_method,
            scale,
            epoch,
            category: row.category,
            delivery_date,
            availability_status: row.availability_status,
            rolling_stocks: Vec::new(),
        })
    }
}

impl TryFrom<RollingStockRow> for RollingStock {
    type Error = anyhow::Error;

    fn try_from(row: RollingStockRow) -> Result<Self, Self::Error> {
        let id = row
            .id
            .parse::<RollingStockId>()
            .map_err(|e| anyhow!("invalid rolling stock id: {}", e))?;

        let railway_company_id =
            crate::catalog::domain::railway_company_id::RailwayCompanyId::try_from(
                row.railway_company_id.clone(),
            )
            .map_err(|e| anyhow!("invalid railway company id: {}", e))?;

        let railway = RollingStockRailway::new(railway_company_id, &row.railway_company_id);
        let category = row.category;

        let is_dummy = row.is_dummy != 0;

        match category {
            RollingStockCategory::Locomotive => {
                let road_number = row.road_number.as_deref().unwrap_or("");

                Ok(RollingStock::new_locomotive(
                    id,
                    row.friendly_name.as_deref(),
                    Some(row.series_code.as_str()),
                    road_number,
                    row.series.as_deref(),
                    railway,
                    row.locomotive_type.unwrap_or_default(),
                    row.depot.as_deref(),
                    row.livery.as_deref(),
                    is_dummy,
                    None::<LengthOverBuffers>,
                    None::<Control>,
                    None::<DccInterface>,
                    None::<TechnicalSpecifications>,
                ))
            }
            RollingStockCategory::FreightCar => {
                // Map: previously type_name -> friendly_name
                Ok(RollingStock::new_freight_car(
                    id,
                    row.friendly_name.as_deref().unwrap_or(""),
                    Some(row.series_code.as_str()),
                    row.road_number.as_deref(),
                    railway,
                    row.freight_car_type,
                    row.livery.as_deref(),
                    None,
                    None,
                ))
            }
            RollingStockCategory::PassengerCar => Ok(RollingStock::new_passenger_car(
                id,
                row.friendly_name.as_deref().unwrap_or(""),
                Some(row.series_code.as_str()),
                row.road_number.as_deref(),
                row.series.as_deref(),
                railway,
                row.passenger_car_type,
                None,
                row.livery.as_deref(),
                None,
                None,
            )),
            RollingStockCategory::ElectricMultipleUnit => {
                Ok(RollingStock::new_electric_multiple_unit(
                    id,
                    row.friendly_name.as_deref().unwrap_or(""),
                    Some(row.series_code.as_str()),
                    row.road_number.as_deref(),
                    row.series.as_deref(),
                    railway,
                    row.electric_multiple_unit_type.unwrap_or_default(),
                    row.depot.as_deref(),
                    row.livery.as_deref(),
                    is_dummy,
                    None,
                    None,
                    None,
                    None,
                ))
            }
            RollingStockCategory::Railcar => Ok(RollingStock::new_railcar(
                id,
                row.friendly_name.as_deref().unwrap_or(""),
                Some(row.series_code.as_str()),
                row.road_number.as_deref(),
                row.series.as_deref(),
                railway,
                row.railcar_type.unwrap_or_default(),
                row.depot.as_deref(),
                row.livery.as_deref(),
                is_dummy,
                None,
                None,
                None,
                None,
            )),
        }
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
        use url::Url;

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
                website_url: Some("https://www.acmetreni.com".to_string()),
                created_at: utc_timestamp,
                updated_at: utc_timestamp,
            };

            let domain = Manufacturer::try_from(row).expect("mapping should succeed");

            assert_eq!(&*domain.id, "trn:manufacturer:mn-1");
            assert_eq!(domain.name, "ACME Models");
            assert_eq!(
                domain.registered_company_name.as_deref(),
                Some("ACME Corporation")
            );
            assert_eq!(
                domain.website_url,
                Some(Url::parse("https://www.acmetreni.com").unwrap())
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

    mod railway_model_mapper_tests {
        use super::*;
        use crate::catalog::domain::availability_status::AvailabilityStatus;
        use crate::catalog::domain::category::{
            ElectricMultipleUnitType, FreightCarType, LocomotiveType, PassengerCarType, RailcarType,
        };
        use crate::catalog::domain::{Category, PowerMethod};
        use chrono::DateTime;
        use pretty_assertions::assert_eq;

        #[test]
        fn railway_model_row_maps_to_domain() {
            let utc_timestamp = DateTime::from_timestamp(0, 0)
                .expect("invalid timestamp")
                .naive_utc();

            let row = RailwayModelRow {
                id: "trn:railway-model:mn-1:ACME-100".to_string(),
                manufacturer_id: "MN-1".to_string(),
                product_code: "ACME-100".to_string(),
                description: "Test model".to_string(),
                details: None,
                power_method: PowerMethod::DC,
                scale: "H0".to_string(),
                epoch: "III".to_string(),
                category: Category::Locomotives,
                delivery_date: Some("2025".to_string()),
                availability_status: Some(AvailabilityStatus::Available),
                created_at: utc_timestamp,
                updated_at: utc_timestamp,
            };

            let domain = RailwayModel::try_from(row).expect("mapping should succeed");
            assert_eq!(&*domain.id, "trn:railway-model:mn-1:ACME-100");
            assert_eq!(domain.product_code.0, "ACME-100");
            assert_eq!(domain.description, "Test model");
            assert_eq!(domain.rolling_stocks.len(), 0);
        }

        #[test]
        fn rolling_stock_row_maps_to_domain_locomotive() {
            let id = RollingStockId::default();
            let row = RollingStockRow {
                id: id.to_string(),
                railway_model_id: "RM-1".to_string(),
                category: RollingStockCategory::Locomotive,
                railway_company_id: "RC-1".to_string(),
                livery: Some("red".to_string()),
                length_inches: None,
                length_millimeters: None,
                technical_minimum_radius_mm: None,
                technical_coupling_close_couplers: None,
                technical_coupling_socket: None,
                technical_coupling_digital_shunting: None,
                technical_flywheel_fitted: None,
                technical_body_shell: None,
                technical_chassis: None,
                technical_interior_lights: None,
                technical_lights: None,
                technical_sprung_buffers: None,
                friendly_name: Some("Class X".to_string()),
                series_code: "123".to_string(),
                road_number: Some("123".to_string()),
                series: None,
                depot: None,
                electric_multiple_unit_type: None,
                freight_car_type: None,
                locomotive_type: Some(LocomotiveType::DieselLocomotive),
                passenger_car_type: None,
                railcar_type: None,
                service_level: None,
                dcc_interface: None,
                control: None,
                is_dummy: 0,
            };

            let domain = RollingStock::try_from(row).expect("mapping should succeed");
            assert_eq!(domain.id_as_ref(), &id);
        }

        #[test]
        fn rolling_stock_row_maps_to_domain_freight_car() {
            let id = RollingStockId::default();
            let row = RollingStockRow {
                id: id.to_string(),
                railway_model_id: "RM-1".to_string(),
                category: RollingStockCategory::FreightCar,
                railway_company_id: "RC-1".to_string(),
                livery: None,
                length_inches: None,
                length_millimeters: None,
                technical_minimum_radius_mm: None,
                technical_coupling_close_couplers: None,
                technical_coupling_socket: None,
                technical_coupling_digital_shunting: None,
                technical_flywheel_fitted: None,
                technical_body_shell: None,
                technical_chassis: None,
                technical_interior_lights: None,
                technical_lights: None,
                technical_sprung_buffers: None,
                friendly_name: Some("Freight Type".to_string()),
                series_code: "".to_string(),
                road_number: None,
                series: None,
                depot: None,
                electric_multiple_unit_type: None,
                freight_car_type: Some(FreightCarType::AutoTransportCars),
                locomotive_type: None,
                passenger_car_type: None,
                railcar_type: None,
                service_level: None,
                dcc_interface: None,
                control: None,
                is_dummy: 0,
            };

            let domain = RollingStock::try_from(row).expect("mapping should succeed");
            assert_eq!(domain.id_as_ref(), &id);
        }

        #[test]
        fn rolling_stock_row_maps_to_domain_passenger_car() {
            let id = RollingStockId::default();
            let row = RollingStockRow {
                id: id.to_string(),
                railway_model_id: "RM-1".to_string(),
                category: RollingStockCategory::PassengerCar,
                railway_company_id: "RC-1".to_string(),
                livery: Some("blue".to_string()),
                length_inches: None,
                length_millimeters: None,
                technical_minimum_radius_mm: None,
                technical_coupling_close_couplers: None,
                technical_coupling_socket: None,
                technical_coupling_digital_shunting: None,
                technical_flywheel_fitted: None,
                technical_body_shell: None,
                technical_chassis: None,
                technical_interior_lights: None,
                technical_lights: None,
                technical_sprung_buffers: None,
                friendly_name: Some("Coach Type".to_string()),
                series_code: "C1".to_string(),
                road_number: Some("C1".to_string()),
                series: None,
                depot: None,
                electric_multiple_unit_type: None,
                freight_car_type: None,
                locomotive_type: None,
                passenger_car_type: Some(PassengerCarType::BaggageCar),
                railcar_type: None,
                service_level: None,
                dcc_interface: None,
                control: None,
                is_dummy: 0,
            };

            let domain = RollingStock::try_from(row).expect("mapping should succeed");
            assert_eq!(domain.id_as_ref(), &id);
        }

        #[test]
        fn rolling_stock_row_maps_to_domain_emu() {
            let id = RollingStockId::default();
            let row = RollingStockRow {
                id: id.to_string(),
                railway_model_id: "RM-1".to_string(),
                category: RollingStockCategory::ElectricMultipleUnit,
                railway_company_id: "RC-1".to_string(),
                livery: None,
                length_inches: None,
                length_millimeters: None,
                technical_minimum_radius_mm: None,
                technical_coupling_close_couplers: None,
                technical_coupling_socket: None,
                technical_coupling_digital_shunting: None,
                technical_flywheel_fitted: None,
                technical_body_shell: None,
                technical_chassis: None,
                technical_interior_lights: None,
                technical_lights: None,
                technical_sprung_buffers: None,
                friendly_name: Some("EMU Type".to_string()),
                series_code: "EMU1".to_string(),
                road_number: Some("EMU1".to_string()),
                series: None,
                depot: None,
                electric_multiple_unit_type: Some(ElectricMultipleUnitType::DrivingCar),
                freight_car_type: None,
                locomotive_type: None,
                passenger_car_type: None,
                railcar_type: None,
                service_level: None,
                dcc_interface: None,
                control: None,
                is_dummy: 0,
            };

            let domain = RollingStock::try_from(row).expect("mapping should succeed");
            assert_eq!(domain.id_as_ref(), &id);
        }

        #[test]
        fn rolling_stock_row_maps_to_domain_railcar() {
            let id = RollingStockId::default();
            let row = RollingStockRow {
                id: id.to_string(),
                railway_model_id: "RM-1".to_string(),
                category: RollingStockCategory::Railcar,
                railway_company_id: "RC-1".to_string(),
                livery: None,
                length_inches: None,
                length_millimeters: None,
                technical_minimum_radius_mm: None,
                technical_coupling_close_couplers: None,
                technical_coupling_socket: None,
                technical_coupling_digital_shunting: None,
                technical_flywheel_fitted: None,
                technical_body_shell: None,
                technical_chassis: None,
                technical_interior_lights: None,
                technical_lights: None,
                technical_sprung_buffers: None,
                friendly_name: Some("Railcar Type".to_string()),
                series_code: "RC-01".to_string(),
                road_number: Some("RC-01".to_string()),
                series: None,
                depot: None,
                electric_multiple_unit_type: None,
                freight_car_type: None,
                locomotive_type: None,
                passenger_car_type: None,
                railcar_type: Some(RailcarType::TrailerCar),
                service_level: None,
                dcc_interface: None,
                control: None,
                is_dummy: 0,
            };

            let domain = RollingStock::try_from(row).expect("mapping should succeed");
            assert_eq!(domain.id_as_ref(), &id);
        }
    }
}
