use super::entities::RailwayCompanyRow;
use super::entities::{ManufacturerRow, RailwayModelRow, RollingStockRow};
use crate::catalog::domain::manufacturer::Manufacturer;
use crate::catalog::domain::railway_company::PeriodOfActivity;
use crate::catalog::domain::railway_company::RailwayCompany;
use crate::catalog::domain::railway_model::RailwayModel;
use crate::catalog::domain::railway_model::RollingStock;
use crate::catalog::domain::railway_model::RollingStockCategory;
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
        Ok(RailwayModel {
            id: row.id,
            manufacturer_id: row.manufacturer_id,
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
            pending_events: Vec::new(),
        })
    }
}

impl TryFrom<RollingStockRow> for RollingStock {
    type Error = DomainError;

    fn try_from(row: RollingStockRow) -> Result<Self, Self::Error> {
        let category = row.category;
        match category {
            RollingStockCategory::Locomotive => Ok(RollingStock::Locomotive {
                id: row.id,
                friendly_name: row.friendly_name,
                series: row.series,
                series_code: row.series_code,
                road_number: row.road_number,
                railway_id: row.railway_company_id,
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
                railway_id: row.railway_company_id,
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
                railway_id: row.railway_company_id,
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
                railway_id: row.railway_company_id,
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
                railway_id: row.railway_company_id,
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
        fn it_should_convert_row_to_domain() {
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
                version: 0,
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
            assert_eq!(domain.country_code, Some("IT".to_string()));
        }

        mod railway_mapper_tests {
            use super::*;
            use crate::catalog::domain::railway_company::{RailwayCompanyId, RailwayStatus};
            use crate::catalog::infrastructure::entities::RailwayCompanyRow;
            use chrono::{DateTime, NaiveDate};
            use pretty_assertions::assert_eq;
            use std::convert::TryFrom;

            #[test]
            fn it_should_convert_row_to_domain() {
                let utc_timestamp = DateTime::from_timestamp(0, 0)
                    .expect("invalid timestamp")
                    .naive_utc();

                let railway_company_id = RailwayCompanyId::try_from("trn:railway-company:fs")
                    .expect("invalid railway company id");

                let row = RailwayCompanyRow {
                    id: railway_company_id.clone(),
                    name: "Name".to_string(),
                    registered_company_name: Some("Registered Company Name".to_string()),
                    country_code: Some("IT".to_string()),
                    status: Some(RailwayStatus::Merged),
                    operating_since: Some(NaiveDate::from_ymd_opt(1905, 1, 1).unwrap()),
                    operating_until: Some(NaiveDate::from_ymd_opt(1925, 2, 1).unwrap()),
                    created_at: utc_timestamp,
                    updated_at: utc_timestamp,
                    version: 0,
                };

                let domain = RailwayCompany::try_from(row).expect("mapping should succeed");

                assert_eq!(domain.id, railway_company_id);
                assert_eq!(domain.name, "Name");
                assert_eq!(
                    domain.registered_company_name.as_deref(),
                    Some("Registered Company Name")
                );
                assert_eq!(
                    domain.period_of_activity,
                    Some(PeriodOfActivity {
                        operating_since: Some(NaiveDate::from_ymd_opt(1905, 1, 1).unwrap()),
                        operating_until: Some(NaiveDate::from_ymd_opt(1925, 2, 1).unwrap()),
                        status: RailwayStatus::Merged,
                    })
                );
                assert_eq!(domain.country_code, Some("IT".to_string()));
            }
        }
    }

    mod railway_model_mapper_tests {
        use super::*;
        use crate::catalog::domain::manufacturer::ManufacturerId;
        use crate::catalog::domain::railway_company::RailwayCompanyId;
        use crate::catalog::domain::railway_model::{
            AvailabilityStatus, Control, DccInterface, DeliveryDate, ProductCode, RailwayModelId,
            RollingStockId, ServiceLevel,
        };
        use crate::catalog::domain::railway_model::{Category, PowerMethod};
        use crate::catalog::domain::railway_model::{
            ElectricMultipleUnitType, FreightCarType, LocomotiveType, PassengerCarType, RailcarType,
        };
        use crate::catalog::domain::scale::Scale;
        use chrono::DateTime;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_convert_railway_model_row_to_domain() {
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
                details: Some("Detailed description".to_string()),
                power_method: PowerMethod::DC,
                scale: Scale::H0,
                epoch: "III".into(),
                category: Category::Locomotives,
                delivery_date: Some(DeliveryDate::Year(2023)),
                availability_status: Some(AvailabilityStatus::Available),
                created_at: utc_timestamp,
                updated_at: utc_timestamp,
                version: 0,
            };

            let domain = RailwayModel::try_from(row).expect("mapping should succeed");
            assert_eq!(domain.id, id);
            assert_eq!(domain.manufacturer_id, manufacturer_id);
            assert_eq!(domain.product_code, product_code);
            assert_eq!(domain.description, "Test model");
            assert_eq!(domain.details.as_deref(), Some("Detailed description"));
            assert_eq!(domain.power_method, PowerMethod::DC);
            assert_eq!(domain.scale, Scale::H0);
            assert_eq!(domain.epoch, "III".into());
            assert_eq!(domain.category, Category::Locomotives);
            assert_eq!(domain.delivery_date, Some(DeliveryDate::Year(2023)));
            assert_eq!(
                domain.availability_status,
                Some(AvailabilityStatus::Available)
            );
            assert_eq!(domain.rolling_stocks.len(), 0);
        }

        #[test]
        fn it_should_convert_locomotive_rolling_stock_row_to_domain() {
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
                livery: Some("Livery".to_string()),
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
                series: Some("Series 1".to_string()),
                depot: Some("Depot".to_string()),
                electric_multiple_unit_type: None,
                freight_car_type: None,
                locomotive_type: Some(LocomotiveType::DieselLocomotive),
                passenger_car_type: None,
                railcar_type: None,
                service_level: None,
                dcc_interface: Some(DccInterface::Nem652),
                control: Some(Control::DccReady),
                is_dummy: false,
            };

            let domain = RollingStock::try_from(row).expect("mapping should succeed");
            match domain {
                RollingStock::Locomotive {
                    id,
                    railway_id,
                    livery,
                    length_over_buffer,
                    technical_specifications,
                    friendly_name,
                    series_code,
                    road_number,
                    series,
                    depot,
                    locomotive_type,
                    dcc_interface,
                    control,
                    is_dummy,
                } => {
                    assert_eq!(id, id);
                    assert_eq!(railway_id, railway_company_id);
                    assert_eq!(livery.as_deref(), Some("Livery"));
                    assert_eq!(friendly_name.as_deref(), Some("Class X"));
                    assert_eq!(series_code, "123");
                    assert_eq!(road_number.as_deref(), Some("123"));
                    assert_eq!(series.as_deref(), Some("Series 1"));
                    assert_eq!(depot.as_deref(), Some("Depot"));
                    assert_eq!(locomotive_type, LocomotiveType::DieselLocomotive);
                    assert_eq!(dcc_interface, Some(DccInterface::Nem652));
                    assert_eq!(control, Some(Control::DccReady));
                    assert_eq!(is_dummy, false);
                    assert_eq!(technical_specifications, None);
                    assert_eq!(length_over_buffer, None);
                }
                _ => panic!("expected locomotive variant"),
            };
        }

        #[test]
        fn it_should_convert_freight_car_rolling_stock_row_to_domain() {
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
                livery: Some("livery".to_string()),
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
                series_code: "Eaos".to_string(),
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
            match domain {
                RollingStock::FreightCar {
                    id,
                    friendly_name,
                    series_code,
                    road_number,
                    freight_car_type,
                    livery,
                    ..
                } => {
                    assert_eq!(id, id);
                    assert_eq!(friendly_name.as_deref(), Some("Freight Type"));
                    assert_eq!(series_code, "Eaos");
                    assert_eq!(road_number, None);
                    assert_eq!(freight_car_type, Some(FreightCarType::AutoTransportCars));
                    assert_eq!(livery, Some("livery".to_string()));
                }
                _ => panic!("expected freight car variant"),
            }
        }

        #[test]
        fn it_should_convert_passenger_car_rolling_stock_row_to_domain() {
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
                livery: Some("Livery".to_string()),
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
                service_level: Some(ServiceLevel::First),
                dcc_interface: None,
                control: None,
                is_dummy: true,
            };

            let domain = RollingStock::try_from(row).expect("mapping should succeed");
            match domain {
                RollingStock::PassengerCar {
                    id,
                    friendly_name,
                    series_code,
                    road_number,
                    passenger_car_type,
                    livery,
                    service_level,
                    ..
                } => {
                    assert_eq!(id, id);
                    assert_eq!(friendly_name.as_deref(), Some("Coach Type"));
                    assert_eq!(series_code, "C1");
                    assert_eq!(road_number.as_deref(), Some("C1"));
                    assert_eq!(passenger_car_type, Some(PassengerCarType::BaggageCar));
                    assert_eq!(livery.as_deref(), Some("Livery"));
                    assert_eq!(service_level, Some(ServiceLevel::First));
                }
                _ => panic!("expected passenger car variant"),
            }
        }

        #[test]
        fn it_should_convert_emu_rolling_stock_row_to_domain() {
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
                livery: Some("Livery".to_string()),
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
                series: Some("Series".to_string()),
                depot: Some("Depot".to_string()),
                electric_multiple_unit_type: Some(ElectricMultipleUnitType::DrivingCar),
                freight_car_type: None,
                locomotive_type: None,
                passenger_car_type: None,
                railcar_type: None,
                service_level: None,
                dcc_interface: Some(DccInterface::Nem652),
                control: Some(Control::DccReady),
                is_dummy: false,
            };

            let domain = RollingStock::try_from(row).expect("mapping should succeed");
            match domain {
                RollingStock::ElectricMultipleUnit {
                    id,
                    friendly_name,
                    series_code,
                    series,
                    road_number,
                    depot,
                    electric_multiple_unit_type,
                    livery,
                    control,
                    dcc_interface,
                    is_dummy,
                    ..
                } => {
                    assert_eq!(id, id);
                    assert_eq!(friendly_name.as_deref(), Some("EMU Type"));
                    assert_eq!(series_code, "EMU1");
                    assert_eq!(series.as_deref(), Some("Series"));
                    assert_eq!(depot.as_deref(), Some("Depot"));
                    assert_eq!(road_number.as_deref(), Some("EMU1"));
                    assert_eq!(
                        electric_multiple_unit_type,
                        ElectricMultipleUnitType::DrivingCar
                    );
                    assert_eq!(livery.as_deref(), Some("Livery"));
                    assert_eq!(control, Some(Control::DccReady));
                    assert_eq!(dcc_interface, Some(DccInterface::Nem652));
                    assert_eq!(is_dummy, false);
                }
                _ => panic!("expected electric multiple unit variant"),
            }
        }

        #[test]
        fn it_should_convert_railcar_rolling_stock_row_to_domain() {
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
                livery: Some("Livery".to_string()),
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
                series: Some("Series".to_string()),
                depot: Some("Depot".to_string()),
                electric_multiple_unit_type: None,
                freight_car_type: None,
                locomotive_type: None,
                passenger_car_type: None,
                railcar_type: Some(RailcarType::TrailerCar),
                service_level: None,
                dcc_interface: Some(DccInterface::Nem652),
                control: Some(Control::DccReady),
                is_dummy: false,
            };

            let domain = RollingStock::try_from(row).expect("mapping should succeed");
            match domain {
                RollingStock::Railcar {
                    id,
                    friendly_name,
                    series_code,
                    series,
                    depot,
                    road_number,
                    railcar_type,
                    livery,
                    control,
                    dcc_interface,
                    is_dummy,
                    ..
                } => {
                    assert_eq!(id, id);
                    assert_eq!(friendly_name.as_deref(), Some("Railcar Type"));
                    assert_eq!(series_code, "RC-01");
                    assert_eq!(road_number.as_deref(), Some("RC-01"));
                    assert_eq!(railcar_type, RailcarType::TrailerCar);
                    assert_eq!(series.as_deref(), Some("Series"));
                    assert_eq!(depot.as_deref(), Some("Depot"));
                    assert_eq!(livery.as_deref(), Some("Livery"));
                    assert_eq!(control, Some(Control::DccReady));
                    assert_eq!(dcc_interface, Some(DccInterface::Nem652));
                    assert_eq!(is_dummy, false);
                }
                _ => panic!("expected railcar variant"),
            }
        }
    }
}
