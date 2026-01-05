use super::entities::RailwayCompanyRow;
use super::entities::{ManufacturerRow, RailwayModelRow, RollingStockRow};
use crate::catalog::domain::manufacturer::Manufacturer;
use crate::catalog::domain::railway_company::PeriodOfActivity;
use crate::catalog::domain::railway_company::RailwayCompany;
use crate::catalog::domain::railway_model::RollingStock;
use crate::catalog::domain::railway_model::RollingStockCategory;
use crate::catalog::domain::railway_model::RollingStockRailway;
use crate::catalog::domain::railway_model::{RailwayModel, RailwayModelManufacturer};
use crate::core::domain::domain_error::DomainError;
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
/// `Err(DomainError)` when validation fails (invalid id or status).
///
/// # Errors
///
/// Errors produced by underlying parsers/validators are propagated and wrapped
/// into a `DomainError`.
impl TryFrom<ManufacturerRow> for Manufacturer {
    type Error = DomainError;

    fn try_from(row: ManufacturerRow) -> Result<Self, Self::Error> {
        let website_url: Option<Url> = match row.website_url {
            Some(s) => {
                if s.trim().is_empty() {
                    None
                } else {
                    Some(Url::parse(&s).map_err(|e| DomainError::Validation(e.to_string()))?)
                }
            }
            None => None,
        };

        Ok(Manufacturer {
            id: row.id,
            name: row.name,
            registered_company_name: row.registered_company_name,
            country_code: row.country_code,
            status: row.status,
            website_url,
        })
    }
}

impl TryFrom<RailwayModelRow> for RailwayModel {
    type Error = DomainError;

    fn try_from(row: RailwayModelRow) -> Result<Self, Self::Error> {
        let manufacturer = RailwayModelManufacturer {
            manufacturer_id: row.manufacturer_id,
            display: row.manufacturer_name,
        };

        Ok(RailwayModel {
            id: row.id,
            manufacturer,
            product_code: row.product_code,
            description: row.description,
            details: row.details,
            power_method: row.power_method,
            scale: row.scale,
            epoch: row.epoch,
            category: row.category,
            delivery_date: row.delivery_date,
            availability_status: row.availability_status,
            rolling_stocks: Vec::new(),
        })
    }
}

impl TryFrom<RollingStockRow> for RollingStock {
    type Error = DomainError;

    fn try_from(row: RollingStockRow) -> Result<Self, Self::Error> {
        let category = row.category;

        let railway = RollingStockRailway {
            railway_company_id: row.railway_company_id,
            display: row.railway_company_name,
        };

        match category {
            RollingStockCategory::Locomotive => Ok(RollingStock::Locomotive {
                id: row.id,
                friendly_name: row.friendly_name,
                series: row.series,
                series_code: row.series_code,
                road_number: row.road_number,
                railway,
                locomotive_type: row.locomotive_type.unwrap_or_default(),
                depot: row.depot,
                livery: row.livery,
                is_dummy: row.is_dummy,
                length_over_buffer: None,
                technical_specifications: None,
                dcc_interface: row.dcc_interface,
                control: row.control,
            }),
            RollingStockCategory::FreightCar => Ok(RollingStock::FreightCar {
                id: row.id,
                friendly_name: row.friendly_name,
                series_code: row.series_code,
                road_number: row.road_number,
                railway,
                freight_car_type: row.freight_car_type,
                livery: row.livery,
                length_over_buffer: None,
                technical_specifications: None,
            }),
            RollingStockCategory::PassengerCar => Ok(RollingStock::PassengerCar {
                id: row.id,
                friendly_name: row.friendly_name,
                series_code: row.series_code,
                series: row.series,
                road_number: row.road_number,
                railway,
                passenger_car_type: row.passenger_car_type,
                service_level: row.service_level,
                livery: row.livery,
                length_over_buffer: None,
                technical_specifications: None,
            }),
            RollingStockCategory::ElectricMultipleUnit => Ok(RollingStock::ElectricMultipleUnit {
                id: row.id,
                friendly_name: row.friendly_name,
                series: row.series,
                series_code: row.series_code,
                road_number: row.road_number,
                railway,
                electric_multiple_unit_type: row.electric_multiple_unit_type.unwrap_or_default(),
                depot: row.depot,
                livery: row.livery,
                is_dummy: row.is_dummy,
                length_over_buffer: None,
                technical_specifications: None,
                dcc_interface: row.dcc_interface,
                control: row.control,
            }),
            RollingStockCategory::Railcar => Ok(RollingStock::Railcar {
                id: row.id,
                friendly_name: row.friendly_name,
                series: row.series,
                series_code: row.series_code,
                road_number: row.road_number,
                railway,
                railcar_type: row.railcar_type.unwrap_or_default(),
                depot: row.depot,
                livery: row.livery,
                is_dummy: row.is_dummy,
                length_over_buffer: None,
                technical_specifications: None,
                dcc_interface: row.dcc_interface,
                control: row.control,
            }),
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
/// `Err(DomainError)` when validation fails (invalid id or period of activity).
///
/// # Errors
///
/// Errors produced by underlying parsers/validators are propagated and wrapped
/// into a `DomainError`.
impl TryFrom<RailwayCompanyRow> for RailwayCompany {
    type Error = DomainError;

    fn try_from(row: RailwayCompanyRow) -> Result<Self, Self::Error> {
        let has_status = row.status.is_some();
        let has_since = row.operating_since.is_some();
        let has_until = row.operating_until.is_some();

        // 3. Build the period only if at least one field existed in the DB
        let period_of_activity =
            (has_status || has_since || has_until).then_some(PeriodOfActivity {
                status: row.status.unwrap_or_default(),
                operating_since: row.operating_since,
                operating_until: row.operating_until,
            });

        Ok(RailwayCompany {
            id: row.id,
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
        use crate::catalog::domain::manufacturer::{ManufacturerId, ManufacturerStatus};
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
            let id =
                ManufacturerId::try_from("trn:manufacturer:mn-1").expect("invalid manufacturer id");
            let row = ManufacturerRow {
                id: id.clone(),
                name: "ACME Models".to_string(),
                registered_company_name: Some("ACME Corporation".to_string()),
                status: ManufacturerStatus::Active,
                country_code: Some("IT".to_string()),
                website_url: Some("https://www.acmetreni.com".to_string()),
                created_at: utc_timestamp,
                updated_at: utc_timestamp,
            };

            let domain = Manufacturer::try_from(row).expect("mapping should succeed");

            assert_eq!(domain.id, id);
            assert_eq!(domain.name, "ACME Models");
            assert_eq!(domain.status, ManufacturerStatus::Active);
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
            use crate::catalog::domain::railway_company::{RailwayCompanyId, RailwayStatus};
            use crate::catalog::infrastructure::entities::RailwayCompanyRow;
            use chrono::DateTime;
            use pretty_assertions::assert_eq;
            use std::convert::TryFrom;

            #[test]
            fn railway_mapper_converts_row_to_domain() {
                let utc_timestamp = DateTime::from_timestamp(0, 0)
                    .expect("invalid timestamp")
                    .naive_utc();

                let railway_company_id = RailwayCompanyId::try_from("trn:railway-company:fs")
                    .expect("invalid railway company id");

                let row = RailwayCompanyRow {
                    id: railway_company_id.clone(),
                    name: "Ferrovie dello Stato".to_string(),
                    registered_company_name: Some("FS S.p.A.".to_string()),
                    country_code: Some("IT".to_string()),
                    status: Some(RailwayStatus::Active),
                    operating_since: None,
                    operating_until: None,
                    created_at: utc_timestamp,
                    updated_at: utc_timestamp,
                };

                let domain = RailwayCompany::try_from(row).expect("mapping should succeed");

                assert_eq!(domain.id, railway_company_id);
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
        use crate::catalog::domain::manufacturer::ManufacturerId;
        use crate::catalog::domain::railway_company::RailwayCompanyId;
        use crate::catalog::domain::railway_model::{
            AvailabilityStatus, DeliveryDate, ProductCode, RailwayModelId, RollingStockId,
        };
        use crate::catalog::domain::railway_model::{Category, PowerMethod};
        use crate::catalog::domain::railway_model::{
            ElectricMultipleUnitType, FreightCarType, LocomotiveType, PassengerCarType, RailcarType,
        };
        use crate::catalog::domain::scale::Scale;
        use chrono::DateTime;
        use pretty_assertions::assert_eq;

        #[test]
        fn railway_model_row_maps_to_domain() {
            let utc_timestamp = DateTime::from_timestamp(0, 0)
                .expect("invalid timestamp")
                .naive_utc();

            let id = RailwayModelId::try_from("trn:railway-model:mn-1:ACME-100")
                .expect("invalid railway model id");
            let manufacturer_id =
                ManufacturerId::try_from("trn:manufacturer:acme").expect("invalid manufacturer id");
            let product_code = ProductCode::try_from("ACME-100").expect("invalid product code");
            let row = RailwayModelRow {
                id: id.clone(),
                manufacturer_id: manufacturer_id.clone(),
                manufacturer_name: "ACME Models".to_string(),
                product_code: product_code.clone(),
                description: "Test model".to_string(),
                details: None,
                power_method: PowerMethod::DC,
                scale: Scale::H0,
                epoch: "III".into(),
                category: Category::Locomotives,
                delivery_date: Some(DeliveryDate::Year(2023)),
                availability_status: Some(AvailabilityStatus::Available),
                created_at: utc_timestamp,
                updated_at: utc_timestamp,
            };

            let domain = RailwayModel::try_from(row).expect("mapping should succeed");
            let manufacturer = RailwayModelManufacturer {
                manufacturer_id: manufacturer_id.clone(),
                display: "ACME Models".to_string(),
            };
            assert_eq!(domain.id, id);
            assert_eq!(domain.manufacturer, manufacturer);
            assert_eq!(domain.product_code, product_code);
            assert_eq!(domain.description, "Test model");
            assert_eq!(domain.rolling_stocks.len(), 0);
        }

        #[test]
        fn rolling_stock_row_maps_to_domain_locomotive() {
            let id = RollingStockId::default();
            let railway_model_id = RailwayModelId::try_from("trn:railway-model:mn-1:ACME-100")
                .expect("invalid railway model id");
            let railway_company_id = RailwayCompanyId::try_from("trn:railway-company:fs")
                .expect("invalid railway model id");
            let row = RollingStockRow {
                id: id.clone(),
                railway_model_id: railway_model_id.clone(),
                category: RollingStockCategory::Locomotive,
                railway_company_id: railway_company_id.clone(),
                railway_company_name: "Ferrovie dello Stato".to_string(),
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
                is_dummy: false,
            };

            let domain = RollingStock::try_from(row).expect("mapping should succeed");
            assert_eq!(domain.id_as_ref(), &id);
        }

        #[test]
        fn rolling_stock_row_maps_to_domain_freight_car() {
            let id = RollingStockId::default();
            let railway_model_id = RailwayModelId::try_from("trn:railway-model:mn-1:ACME-100")
                .expect("invalid railway model id");
            let railway_company_id = RailwayCompanyId::try_from("trn:railway-company:fs")
                .expect("invalid railway model id");
            let row = RollingStockRow {
                id: id.clone(),
                railway_model_id: railway_model_id.clone(),
                category: RollingStockCategory::FreightCar,
                railway_company_id: railway_company_id.clone(),
                railway_company_name: "Ferrovie dello Stato".to_string(),
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
                is_dummy: true,
            };

            let domain = RollingStock::try_from(row).expect("mapping should succeed");
            assert_eq!(domain.id_as_ref(), &id);
        }

        #[test]
        fn rolling_stock_row_maps_to_domain_passenger_car() {
            let id = RollingStockId::default();
            let railway_model_id = RailwayModelId::try_from("trn:railway-model:mn-1:ACME-100")
                .expect("invalid railway model id");
            let railway_company_id = RailwayCompanyId::try_from("trn:railway-company:fs")
                .expect("invalid railway model id");
            let row = RollingStockRow {
                id: id.clone(),
                railway_model_id: railway_model_id.clone(),
                category: RollingStockCategory::PassengerCar,
                railway_company_id: railway_company_id.clone(),
                railway_company_name: "Ferrovie dello Stato".to_string(),
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
                is_dummy: true,
            };

            let domain = RollingStock::try_from(row).expect("mapping should succeed");
            assert_eq!(domain.id_as_ref(), &id);
        }

        #[test]
        fn rolling_stock_row_maps_to_domain_emu() {
            let id = RollingStockId::default();
            let railway_model_id = RailwayModelId::try_from("trn:railway-model:mn-1:ACME-100")
                .expect("invalid railway model id");
            let railway_company_id = RailwayCompanyId::try_from("trn:railway-company:fs")
                .expect("invalid railway model id");
            let row = RollingStockRow {
                id: id.clone(),
                railway_model_id: railway_model_id.clone(),
                category: RollingStockCategory::ElectricMultipleUnit,
                railway_company_id: railway_company_id.clone(),
                railway_company_name: "Ferrovie dello Stato".to_string(),
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
                is_dummy: false,
            };

            let domain = RollingStock::try_from(row).expect("mapping should succeed");
            assert_eq!(domain.id_as_ref(), &id);
        }

        #[test]
        fn rolling_stock_row_maps_to_domain_railcar() {
            let id = RollingStockId::default();
            let railway_model_id = RailwayModelId::try_from("trn:railway-model:mn-1:ACME-100")
                .expect("invalid railway model id");
            let railway_company_id = RailwayCompanyId::try_from("trn:railway-company:fs")
                .expect("invalid railway model id");
            let row = RollingStockRow {
                id: id.clone(),
                railway_model_id: railway_model_id.clone(),
                category: RollingStockCategory::Railcar,
                railway_company_id: railway_company_id.clone(),
                railway_company_name: "Ferrovie dello Stato".to_string(),
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
                is_dummy: false,
            };

            let domain = RollingStock::try_from(row).expect("mapping should succeed");
            assert_eq!(domain.id_as_ref(), &id);
        }
    }
}
