use garde::Validate;
use serde::Deserialize;

use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

use crate::{
    catalog::application::{
        CouplingInput, CreateRailwayModelInput, CreateRollingStockInput, DeleteRollingStockInput,
        LengthOverBuffersInput, RailwayModelTextField, SaveRailwayModelInput,
        SearchRailwayModelsInput, SetRollingStockCouplerInput, SimplifiedRollingStockInput,
        TechnicalSpecificationsInput, UpdateRailwayModelClassificationInput,
        UpdateRailwayModelDeliveryDateInput, UpdateRailwayModelTextInput,
        UpdateRollingStockCategoryInput, UpdateRollingStockDccInput,
        UpdateRollingStockIdentificationInput, UpdateRollingStockRailwayCompanyInput,
        UpdateRollingStockServiceLevelInput, UpdateRollingStockSpecificationsInput,
        UpdateRollingStockSubcategoryInput, UpsertRailwayModelTranslationInput,
    },
    catalog::domain::railway_company::RailwayCompanyId,
    catalog::domain::railway_model::RollingStockCategory,
    catalog::domain::railway_model::{
        BodyShellType, Category, ChassisType, Control, CouplerTypeId, CouplingSocket, DccInterface,
        DeliveryDate, Epoch, FeatureFlag, LengthOverBuffers, RailwayModelId, RollingStockId,
        RollingStockSpecPatch, ServiceLevel,
    },
    catalog::domain::scale::Scale,
    collecting::domain::OwnedRollingStockId,
    core::domain::length::Length,
    core::domain::{Language, domain_error::DomainError},
};

/// Arguments for creating a new railway model (transport from IPC to application).
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct CreateRailwayModelArgs {
    /// ID of the manufacturer (non-empty TRN string).
    #[garde(length(min = 1))]
    pub manufacturer_id: String,
    /// Product code of the railway model (1–20 characters).
    #[garde(length(min = 1, max = 20))]
    pub product_code: String,
    /// Description of the railway model (1–500 characters).
    #[garde(length(min = 1, max = 500))]
    pub description: String,
    /// Additional details about the railway model.
    pub details: Option<String>,
    /// Power method of the railway model (AC / DC / TRIX_EXPRESS).
    #[garde(custom(crate::catalog::domain::railway_model::power_method::validate_power_method))]
    pub power_method: String,
    /// Scale of the railway model (H0 / N / Z / etc.).
    #[garde(custom(crate::catalog::domain::scale::scale::validate_scale))]
    pub scale: String,
    /// Epoch of the railway model (I / II / IIa / III/IV / Vm / etc.).
    #[garde(length(min = 1, max = 10))]
    pub epoch: String,
    /// Category of the railway model (LOCOMOTIVES / FREIGHT_CARS / etc.).
    #[garde(custom(crate::catalog::domain::railway_model::category::validate_category))]
    pub category: String,
    /// Optional delivery date of the railway model.
    pub delivery_date: Option<String>,
    /// Optional availability status (ANNOUNCED / AVAILABLE / CANCELLED / DISCONTINUED).
    #[garde(custom(crate::catalog::domain::railway_model::availability_status::validate_opt_availability_status))]
    pub availability_status: Option<String>,
    /// Rolling stock items associated with the railway model (at least one required).
    #[garde(length(min = 1))]
    pub rolling_stocks: Vec<CreateRollingStockArgs>,
}

/// Input for creating a rolling stock (tagged union by category).
#[derive(Debug, Clone, Deserialize, specta::Type)]
#[serde(tag = "category", rename_all = "camelCase")]
pub enum CreateRollingStockArgs {
    Locomotive {
        railway_company_id: String,
        friendly_name: String,
        series_code: String,
        road_number: String,
        series: Option<String>,
        depot: Option<String>,
        livery: Option<String>,
        locomotive_type: String,
        is_dummy: Option<bool>,
        control: Option<String>,
        dcc_interface: Option<String>,
        length_over_buffers: Option<LengthOverBuffersArgs>,
        technical_specifications: Option<TechnicalSpecificationsArgs>,
    },
    PassengerCar {
        railway_company_id: String,
        friendly_name: String,
        series_code: String,
        road_number: Option<String>,
        series: Option<String>,
        livery: Option<String>,
        passenger_car_type: String,
        service_level: Option<String>,
        length_over_buffers: Option<LengthOverBuffersArgs>,
        technical_specifications: Option<TechnicalSpecificationsArgs>,
    },
    FreightCar {
        railway_company_id: String,
        friendly_name: String,
        series_code: String,
        road_number: Option<String>,
        series: Option<String>,
        livery: Option<String>,
        freight_car_type: Option<String>,
        length_over_buffers: Option<LengthOverBuffersArgs>,
        technical_specifications: Option<TechnicalSpecificationsArgs>,
    },
    Railcar {
        railway_company_id: String,
        friendly_name: String,
        series_code: String,
        road_number: Option<String>,
        series: Option<String>,
        depot: Option<String>,
        livery: Option<String>,
        railcar_type: Option<String>,
        is_dummy: Option<bool>,
        control: Option<String>,
        dcc_interface: Option<String>,
        length_over_buffers: Option<LengthOverBuffersArgs>,
        technical_specifications: Option<TechnicalSpecificationsArgs>,
    },
    ElectricMultipleUnit {
        railway_company_id: String,
        friendly_name: String,
        series_code: String,
        road_number: Option<String>,
        series: Option<String>,
        depot: Option<String>,
        livery: Option<String>,
        electric_multiple_unit_type: String,
        is_dummy: Option<bool>,
        control: Option<String>,
        dcc_interface: Option<String>,
        length_over_buffers: Option<LengthOverBuffersArgs>,
        technical_specifications: Option<TechnicalSpecificationsArgs>,
    },
}

/// Length measurements over buffers for a rolling stock item.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct LengthOverBuffersArgs {
    /// Length in millimeters (must be non-negative when provided).
    #[garde(range(min = 0.0))]
    pub millimeters: Option<f64>,
    /// Length in inches (must be non-negative when provided).
    #[garde(range(min = 0.0))]
    pub inches: Option<f64>,
}

/// Optional technical specifications for a rolling stock item.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct TechnicalSpecificationsArgs {
    /// Minimum radius the rolling stock can navigate (must be non-negative when provided).
    #[garde(range(min = 0.0))]
    pub minimum_radius: Option<f64>,
    /// Coupling details.
    pub coupling: Option<CouplingArgs>,
    /// Flywheel details (YES / NO / NOT_APPLICABLE).
    #[garde(custom(
        crate::catalog::domain::railway_model::feature_flag::validate_opt_feature_flag
    ))]
    pub flywheel_fitted: Option<String>,
    /// Body shell details (PLASTIC / METAL_DIE_CAST).
    #[garde(custom(
        crate::catalog::domain::railway_model::body_shell_type::validate_opt_body_shell_type
    ))]
    pub body_shell: Option<String>,
    /// Chassis details (PLASTIC / METAL_DIE_CAST).
    #[garde(custom(
        crate::catalog::domain::railway_model::chassis_type::validate_opt_chassis_type
    ))]
    pub chassis: Option<String>,
    /// Presence of interior lighting (YES / NO / NOT_APPLICABLE).
    #[garde(custom(
        crate::catalog::domain::railway_model::feature_flag::validate_opt_feature_flag
    ))]
    pub interior_lights: Option<String>,
    /// Presence of headlights or other lights (YES / NO / NOT_APPLICABLE).
    #[garde(custom(
        crate::catalog::domain::railway_model::feature_flag::validate_opt_feature_flag
    ))]
    pub lights: Option<String>,
    /// Presence of sprung buffers (YES / NO / NOT_APPLICABLE).
    #[garde(custom(
        crate::catalog::domain::railway_model::feature_flag::validate_opt_feature_flag
    ))]
    pub sprung_buffers: Option<String>,
}

/// Coupling details for a rolling stock item.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct CouplingArgs {
    /// Type of coupling used (NONE / NEM_355 / NEM_356 / NEM_357 / NEM_359 / NEM_360 / NEM_362 / NEM_365).
    #[garde(custom(
        crate::catalog::domain::railway_model::coupling_socket::validate_coupling_socket
    ))]
    pub socket: String,
    /// Type of coupling head used (YES / NO / NOT_APPLICABLE).
    #[garde(custom(
        crate::catalog::domain::railway_model::feature_flag::validate_opt_feature_flag
    ))]
    pub close_couplers: Option<String>,
    /// Presence of digital shunting couplers (YES / NO / NOT_APPLICABLE).
    #[garde(custom(
        crate::catalog::domain::railway_model::feature_flag::validate_opt_feature_flag
    ))]
    pub digital_shunting: Option<String>,
}

impl TryFrom<CreateRailwayModelArgs> for CreateRailwayModelInput {
    type Error = DomainError;

    fn try_from(args: CreateRailwayModelArgs) -> Result<Self, Self::Error> {
        let rolling_stocks = args
            .rolling_stocks
            .iter()
            .map(|rs_args| CreateRollingStockInput::try_from(rs_args.clone()))
            .collect::<Result<Vec<CreateRollingStockInput>, DomainError>>()?;

        Ok(CreateRailwayModelInput {
            manufacturer_id: args.manufacturer_id,
            product_code: args.product_code,
            description: args.description,
            details: args.details,
            power_method: args.power_method,
            scale: args.scale,
            epoch: args.epoch,
            category: args.category,
            delivery_date: args.delivery_date,
            availability_status: args.availability_status,
            rolling_stocks,
        })
    }
}

/// Simplified arguments for creating a railway model and optionally a small set
/// of rolling stocks. This is a lighter-weight payload used by the
/// `add_railway_model_to_collection` and `add_railway_model_to_wish_list` commands.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct SimplifiedRailwayModelArgs {
    #[garde(length(min = 1))]
    pub manufacturer_id: String,
    #[garde(length(min = 1, max = 20))]
    pub product_code: String,
    #[garde(length(min = 1, max = 500))]
    pub description: String,
    #[garde(custom(crate::catalog::domain::railway_model::category::validate_category))]
    pub category: String,
    #[garde(custom(crate::catalog::domain::scale::scale::validate_scale))]
    pub scale: String,
    #[garde(length(min = 1, max = 10))]
    pub epoch: String,
    #[garde(custom(crate::catalog::domain::railway_model::power_method::validate_power_method))]
    pub power_method: String,
    pub rolling_stocks: Vec<SimplifiedRollingStockArgs>,
}

#[derive(Debug, Clone, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SimplifiedRollingStockArgs {
    pub railway_company_id: String,
    pub series_code: String,
    pub road_number: Option<String>,
    pub subcategory: Option<String>,
    pub category: String,
}

impl TryFrom<SimplifiedRailwayModelArgs> for SaveRailwayModelInput {
    type Error = DomainError;

    fn try_from(args: SimplifiedRailwayModelArgs) -> Result<Self, Self::Error> {
        let rolling_stocks = args
            .rolling_stocks
            .into_iter()
            .map(|rs| SimplifiedRollingStockInput {
                railway_company_id: rs.railway_company_id,
                series_code: rs.series_code,
                road_number: rs.road_number,
                subcategory: rs.subcategory,
                category: rs.category,
            })
            .collect();

        Ok(SaveRailwayModelInput {
            manufacturer_id: args.manufacturer_id,
            product_code: args.product_code,
            description: args.description,
            category: args.category,
            scale: args.scale,
            epoch: args.epoch,
            power_method: args.power_method,
            rolling_stocks,
        })
    }
}

impl TryFrom<CreateRollingStockArgs> for CreateRollingStockInput {
    type Error = DomainError;

    fn try_from(args: CreateRollingStockArgs) -> Result<Self, Self::Error> {
        match args {
            CreateRollingStockArgs::Locomotive {
                railway_company_id,
                friendly_name,
                series_code,
                road_number,
                series,
                depot,
                livery,
                locomotive_type,
                is_dummy,
                control,
                dcc_interface,
                length_over_buffers,
                technical_specifications,
            } => {
                let length_over_buffers = match length_over_buffers {
                    Some(l) => Some(LengthOverBuffersInput::try_from(l)?),
                    None => None,
                };

                let technical_specifications = match technical_specifications {
                    Some(t) => Some(TechnicalSpecificationsInput::try_from(t)?),
                    None => None,
                };

                Ok(CreateRollingStockInput::Locomotive {
                    railway_company_id,
                    friendly_name,
                    series_code,
                    road_number,
                    series,
                    depot,
                    livery,
                    locomotive_type,
                    is_dummy,
                    control,
                    dcc_interface,
                    length_over_buffers,
                    technical_specifications,
                })
            }
            CreateRollingStockArgs::PassengerCar {
                railway_company_id,
                friendly_name,
                series_code,
                road_number,
                series,
                livery,
                passenger_car_type,
                service_level,
                length_over_buffers,
                technical_specifications,
            } => {
                let length_over_buffers = match length_over_buffers {
                    Some(l) => Some(LengthOverBuffersInput::try_from(l)?),
                    None => None,
                };

                let technical_specifications = match technical_specifications {
                    Some(t) => Some(TechnicalSpecificationsInput::try_from(t)?),
                    None => None,
                };

                Ok(CreateRollingStockInput::PassengerCar {
                    railway_company_id,
                    friendly_name,
                    series_code,
                    road_number,
                    series,
                    livery,
                    passenger_car_type,
                    service_level,
                    length_over_buffers,
                    technical_specifications,
                })
            }
            CreateRollingStockArgs::FreightCar {
                railway_company_id,
                friendly_name,
                series_code,
                road_number,
                series,
                livery,
                freight_car_type,
                length_over_buffers,
                technical_specifications,
            } => {
                let length_over_buffers = match length_over_buffers {
                    Some(l) => Some(LengthOverBuffersInput::try_from(l)?),
                    None => None,
                };

                let technical_specifications = match technical_specifications {
                    Some(t) => Some(TechnicalSpecificationsInput::try_from(t)?),
                    None => None,
                };

                Ok(CreateRollingStockInput::FreightCar {
                    railway_company_id,
                    friendly_name,
                    series_code,
                    road_number,
                    series,
                    livery,
                    freight_car_type,
                    length_over_buffers,
                    technical_specifications,
                })
            }
            CreateRollingStockArgs::Railcar {
                railway_company_id,
                friendly_name,
                series_code,
                road_number,
                series,
                depot,
                livery,
                railcar_type,
                is_dummy,
                control,
                dcc_interface,
                length_over_buffers,
                technical_specifications,
            } => {
                let length_over_buffers = match length_over_buffers {
                    Some(l) => Some(LengthOverBuffersInput::try_from(l)?),
                    None => None,
                };

                let technical_specifications = match technical_specifications {
                    Some(t) => Some(TechnicalSpecificationsInput::try_from(t)?),
                    None => None,
                };

                Ok(CreateRollingStockInput::Railcar {
                    railway_company_id,
                    friendly_name,
                    series_code,
                    road_number,
                    series,
                    depot,
                    livery,
                    railcar_type,
                    is_dummy,
                    control,
                    dcc_interface,
                    length_over_buffers,
                    technical_specifications,
                })
            }
            CreateRollingStockArgs::ElectricMultipleUnit {
                railway_company_id,
                friendly_name,
                series_code,
                road_number,
                series,
                depot,
                livery,
                electric_multiple_unit_type,
                is_dummy,
                control,
                dcc_interface,
                length_over_buffers,
                technical_specifications,
            } => {
                let length_over_buffers = match length_over_buffers {
                    Some(l) => Some(LengthOverBuffersInput::try_from(l)?),
                    None => None,
                };

                let technical_specifications = match technical_specifications {
                    Some(t) => Some(TechnicalSpecificationsInput::try_from(t)?),
                    None => None,
                };

                Ok(CreateRollingStockInput::ElectricMultipleUnit {
                    railway_company_id,
                    friendly_name,
                    series_code,
                    road_number,
                    series,
                    depot,
                    livery,
                    electric_multiple_unit_type,
                    is_dummy,
                    control,
                    dcc_interface,
                    length_over_buffers,
                    technical_specifications,
                })
            }
        }
    }
}

impl TryFrom<LengthOverBuffersArgs> for LengthOverBuffersInput {
    type Error = DomainError;

    fn try_from(args: LengthOverBuffersArgs) -> Result<Self, Self::Error> {
        Ok(LengthOverBuffersInput {
            millimeters: args.millimeters,
            inches: args.inches,
        })
    }
}

impl TryFrom<TechnicalSpecificationsArgs> for TechnicalSpecificationsInput {
    type Error = DomainError;

    fn try_from(args: TechnicalSpecificationsArgs) -> Result<Self, Self::Error> {
        let coupling = match args.coupling {
            Some(coupling_args) => Some(CouplingInput {
                socket: coupling_args.socket,
                close_couplers: coupling_args.close_couplers,
                digital_shunting: coupling_args.digital_shunting,
            }),
            None => None,
        };

        Ok(TechnicalSpecificationsInput {
            minimum_radius: args.minimum_radius,
            coupling,
            flywheel_fitted: args.flywheel_fitted,
            body_shell: args.body_shell,
            chassis: args.chassis,
            interior_lights: args.interior_lights,
            lights: args.lights,
            sprung_buffers: args.sprung_buffers,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_convert_locomotive_with_nested_fields() {
        let args = CreateRollingStockArgs::Locomotive {
            railway_company_id: "rc".to_string(),
            friendly_name: "Friendly".to_string(),
            series_code: "S1".to_string(),
            road_number: "123".to_string(),
            series: Some("SeriesA".to_string()),
            depot: Some("DepotX".to_string()),
            livery: Some("Red".to_string()),
            locomotive_type: "Diesel".to_string(),
            is_dummy: Some(false),
            control: Some("Manual".to_string()),
            dcc_interface: Some("DCC".to_string()),
            length_over_buffers: Some(LengthOverBuffersArgs {
                millimeters: Some(100.0),
                inches: Some(3.93),
            }),
            technical_specifications: Some(TechnicalSpecificationsArgs {
                minimum_radius: Some(250.0),
                coupling: Some(CouplingArgs {
                    socket: "NEM".to_string(),
                    close_couplers: Some("Yes".to_string()),
                    digital_shunting: Some("No".to_string()),
                }),
                flywheel_fitted: Some("Yes".to_string()),
                body_shell: Some("Plastic".to_string()),
                chassis: Some("Metal".to_string()),
                interior_lights: Some("Yes".to_string()),
                lights: Some("LED".to_string()),
                sprung_buffers: Some("No".to_string()),
            }),
        };

        let input = CreateRollingStockInput::try_from(args).expect("conversion failed");

        match input {
            CreateRollingStockInput::Locomotive {
                railway_company_id,
                friendly_name,
                series_code,
                road_number,
                series,
                depot,
                livery,
                locomotive_type,
                is_dummy,
                control,
                dcc_interface,
                length_over_buffers,
                technical_specifications,
            } => {
                assert_eq!(railway_company_id, "rc");
                assert_eq!(friendly_name, "Friendly");
                assert_eq!(series_code, "S1");
                assert_eq!(road_number, "123");
                assert_eq!(series.as_deref(), Some("SeriesA"));
                assert_eq!(depot.as_deref(), Some("DepotX"));
                assert_eq!(livery.as_deref(), Some("Red"));
                assert_eq!(locomotive_type, "Diesel");
                assert_eq!(is_dummy, Some(false));
                assert_eq!(control.as_deref(), Some("Manual"));
                assert_eq!(dcc_interface.as_deref(), Some("DCC"));

                let lob = length_over_buffers.expect("length_over_buffers missing");
                assert_eq!(lob.millimeters, Some(100.0));
                assert_eq!(lob.inches, Some(3.93));

                let ts = technical_specifications.expect("technical_specifications missing");
                assert_eq!(ts.minimum_radius, Some(250.0));
                let coupling = ts.coupling.expect("coupling missing");
                assert_eq!(coupling.socket, "NEM");
                assert_eq!(coupling.close_couplers.as_deref(), Some("Yes"));
                assert_eq!(coupling.digital_shunting.as_deref(), Some("No"));
            }
            _ => panic!("expected locomotive variant"),
        }
    }

    #[test]
    fn it_should_convert_passenger_car_minimal_fields() {
        let args = CreateRollingStockArgs::PassengerCar {
            railway_company_id: "pc_rc".to_string(),
            friendly_name: "Passenger".to_string(),
            series_code: "PC1".to_string(),
            road_number: None,
            series: None,
            livery: None,
            passenger_car_type: "Coach".to_string(),
            service_level: None,
            length_over_buffers: None,
            technical_specifications: None,
        };

        let input = CreateRollingStockInput::try_from(args).expect("conversion failed");

        match input {
            CreateRollingStockInput::PassengerCar {
                railway_company_id,
                friendly_name,
                series_code,
                road_number,
                series,
                livery,
                passenger_car_type,
                service_level,
                length_over_buffers,
                technical_specifications,
            } => {
                assert_eq!(railway_company_id, "pc_rc");
                assert_eq!(friendly_name, "Passenger");
                assert_eq!(series_code, "PC1");
                assert!(road_number.is_none());
                assert!(series.is_none());
                assert!(livery.is_none());
                assert_eq!(passenger_car_type, "Coach");
                assert!(service_level.is_none());
                assert!(length_over_buffers.is_none());
                assert!(technical_specifications.is_none());
            }
            _ => panic!("expected passenger car variant"),
        }
    }

    #[test]
    fn it_should_convert_freight_car_minimal_fields() {
        let args = CreateRollingStockArgs::FreightCar {
            railway_company_id: "fc_rc".to_string(),
            friendly_name: "Freight".to_string(),
            series_code: "FC1".to_string(),
            road_number: None,
            series: None,
            livery: None,
            freight_car_type: Some("Box".to_string()),
            length_over_buffers: None,
            technical_specifications: None,
        };

        let input = CreateRollingStockInput::try_from(args).expect("conversion failed");

        match input {
            CreateRollingStockInput::FreightCar {
                railway_company_id,
                friendly_name,
                series_code,
                road_number,
                series,
                livery,
                freight_car_type,
                length_over_buffers,
                technical_specifications,
            } => {
                assert_eq!(railway_company_id, "fc_rc");
                assert_eq!(friendly_name, "Freight");
                assert_eq!(series_code, "FC1");
                assert!(road_number.is_none());
                assert!(series.is_none());
                assert!(livery.is_none());
                assert_eq!(freight_car_type.as_deref(), Some("Box"));
                assert!(length_over_buffers.is_none());
                assert!(technical_specifications.is_none());
            }
            _ => panic!("expected freight car variant"),
        }
    }

    #[test]
    fn it_should_convert_railcar_with_nested_fields() {
        let args = CreateRollingStockArgs::Railcar {
            railway_company_id: "rc_rc".to_string(),
            friendly_name: "RailcarX".to_string(),
            series_code: "R1".to_string(),
            road_number: Some("900".to_string()),
            series: Some("RSeries".to_string()),
            depot: Some("DepotY".to_string()),
            livery: Some("Blue".to_string()),
            railcar_type: Some("DMU".to_string()),
            is_dummy: Some(true),
            control: Some("Automatic".to_string()),
            dcc_interface: Some("None".to_string()),
            length_over_buffers: Some(LengthOverBuffersArgs {
                millimeters: Some(150.0),
                inches: Some(5.90),
            }),
            technical_specifications: Some(TechnicalSpecificationsArgs {
                minimum_radius: Some(300.0),
                coupling: None,
                flywheel_fitted: None,
                body_shell: Some("Steel".to_string()),
                chassis: None,
                interior_lights: None,
                lights: None,
                sprung_buffers: None,
            }),
        };

        let input = CreateRollingStockInput::try_from(args).expect("conversion failed");

        match input {
            CreateRollingStockInput::Railcar {
                railway_company_id,
                friendly_name,
                series_code,
                road_number,
                series,
                depot,
                livery,
                railcar_type,
                is_dummy,
                control,
                dcc_interface,
                length_over_buffers,
                technical_specifications,
            } => {
                assert_eq!(railway_company_id, "rc_rc");
                assert_eq!(friendly_name, "RailcarX");
                assert_eq!(series_code, "R1");
                assert_eq!(road_number.as_deref(), Some("900"));
                assert_eq!(series.as_deref(), Some("RSeries"));
                assert_eq!(depot.as_deref(), Some("DepotY"));
                assert_eq!(livery.as_deref(), Some("Blue"));
                assert_eq!(railcar_type.as_deref(), Some("DMU"));
                assert_eq!(is_dummy, Some(true));
                assert_eq!(control.as_deref(), Some("Automatic"));
                assert_eq!(dcc_interface.as_deref(), Some("None"));

                let lob = length_over_buffers.expect("length_over_buffers missing");
                assert_eq!(lob.millimeters, Some(150.0));
                assert_eq!(lob.inches, Some(5.90));

                let ts = technical_specifications.expect("technical_specifications missing");
                assert_eq!(ts.minimum_radius, Some(300.0));
                assert_eq!(ts.body_shell.as_deref(), Some("Steel"));
                assert!(ts.coupling.is_none());
            }
            _ => panic!("expected railcar variant"),
        }
    }

    #[test]
    fn it_should_convert_electric_multiple_unit_with_nested_fields() {
        let args = CreateRollingStockArgs::ElectricMultipleUnit {
            railway_company_id: "emu_rc".to_string(),
            friendly_name: "EMU-1".to_string(),
            series_code: "EMU1".to_string(),
            road_number: Some("77".to_string()),
            series: Some("EMU-Series".to_string()),
            depot: Some("Central".to_string()),
            livery: Some("Green".to_string()),
            electric_multiple_unit_type: "Electric".to_string(),
            is_dummy: Some(false),
            control: Some("Remote".to_string()),
            dcc_interface: Some("DCC-EMU".to_string()),
            length_over_buffers: Some(LengthOverBuffersArgs {
                millimeters: Some(200.0),
                inches: Some(7.87),
            }),
            technical_specifications: Some(TechnicalSpecificationsArgs {
                minimum_radius: None,
                coupling: Some(CouplingArgs {
                    socket: "Kadee".to_string(),
                    close_couplers: None,
                    digital_shunting: Some("Supported".to_string()),
                }),
                flywheel_fitted: None,
                body_shell: None,
                chassis: None,
                interior_lights: Some("Yes".to_string()),
                lights: Some("Halogen".to_string()),
                sprung_buffers: Some("Yes".to_string()),
            }),
        };

        let input = CreateRollingStockInput::try_from(args).expect("conversion failed");

        match input {
            CreateRollingStockInput::ElectricMultipleUnit {
                railway_company_id,
                friendly_name,
                series_code,
                road_number,
                series,
                depot,
                livery,
                electric_multiple_unit_type,
                is_dummy,
                control,
                dcc_interface,
                length_over_buffers,
                technical_specifications,
            } => {
                assert_eq!(railway_company_id, "emu_rc");
                assert_eq!(friendly_name, "EMU-1");
                assert_eq!(series_code, "EMU1");
                assert_eq!(road_number.as_deref(), Some("77"));
                assert_eq!(series.as_deref(), Some("EMU-Series"));
                assert_eq!(depot.as_deref(), Some("Central"));
                assert_eq!(livery.as_deref(), Some("Green"));
                assert_eq!(electric_multiple_unit_type, "Electric");
                assert_eq!(is_dummy, Some(false));
                assert_eq!(control.as_deref(), Some("Remote"));
                assert_eq!(dcc_interface.as_deref(), Some("DCC-EMU"));

                let lob = length_over_buffers.expect("length_over_buffers missing");
                assert_eq!(lob.millimeters, Some(200.0));
                assert_eq!(lob.inches, Some(7.87));

                let ts = technical_specifications.expect("technical_specifications missing");
                let coupling = ts.coupling.expect("coupling missing");
                assert_eq!(coupling.socket, "Kadee");
                assert_eq!(coupling.digital_shunting.as_deref(), Some("Supported"));
                assert_eq!(ts.interior_lights.as_deref(), Some("Yes"));
                assert_eq!(ts.lights.as_deref(), Some("Halogen"));
                assert_eq!(ts.sprung_buffers.as_deref(), Some("Yes"));
            }
            _ => panic!("expected EMU variant"),
        }
    }

    #[test]
    fn it_should_convert_create_railway_model_args_with_rolling_stocks() {
        let args = CreateRailwayModelArgs {
            manufacturer_id: "manu_1".to_string(),
            product_code: "P100".to_string(),
            description: "A test model".to_string(),
            details: Some("Detailed info".to_string()),
            power_method: "Electric".to_string(),
            scale: "HO".to_string(),
            epoch: "IV".to_string(),
            category: "Diesel".to_string(),
            delivery_date: Some("2026-01-01".to_string()),
            availability_status: Some("InStock".to_string()),
            rolling_stocks: vec![
                CreateRollingStockArgs::Locomotive {
                    railway_company_id: "rc".to_string(),
                    friendly_name: "Friendly".to_string(),
                    series_code: "S1".to_string(),
                    road_number: "123".to_string(),
                    series: None,
                    depot: None,
                    livery: None,
                    locomotive_type: "Diesel".to_string(),
                    is_dummy: None,
                    control: None,
                    dcc_interface: None,
                    length_over_buffers: None,
                    technical_specifications: None,
                },
                CreateRollingStockArgs::PassengerCar {
                    railway_company_id: "pc_rc".to_string(),
                    friendly_name: "Passenger".to_string(),
                    series_code: "PC1".to_string(),
                    road_number: None,
                    series: None,
                    livery: None,
                    passenger_car_type: "Coach".to_string(),
                    service_level: None,
                    length_over_buffers: None,
                    technical_specifications: None,
                },
            ],
        };

        let input = CreateRailwayModelInput::try_from(args).expect("conversion failed");

        assert_eq!(input.manufacturer_id, "manu_1");
        assert_eq!(input.product_code, "P100");
        assert_eq!(input.description, "A test model");
        assert_eq!(input.details.as_deref(), Some("Detailed info"));
        assert_eq!(input.power_method, "Electric");
        assert_eq!(input.scale, "HO");
        assert_eq!(input.epoch, "IV");
        assert_eq!(input.category, "Diesel");
        assert_eq!(input.delivery_date.as_deref(), Some("2026-01-01"));
        assert_eq!(input.availability_status.as_deref(), Some("InStock"));

        assert_eq!(input.rolling_stocks.len(), 2);

        match &input.rolling_stocks[0] {
            CreateRollingStockInput::Locomotive {
                railway_company_id,
                road_number,
                ..
            } => {
                assert_eq!(railway_company_id, "rc");
                assert_eq!(road_number, "123");
            }
            _ => panic!("expected locomotive in first position"),
        }

        match &input.rolling_stocks[1] {
            CreateRollingStockInput::PassengerCar {
                railway_company_id,
                passenger_car_type,
                ..
            } => {
                assert_eq!(railway_company_id, "pc_rc");
                assert_eq!(passenger_car_type, "Coach");
            }
            _ => panic!("expected passenger car in second position"),
        }
    }
}

// ---------------------------------------------------------------------------
// UpdateRailwayModelText args
// ---------------------------------------------------------------------------

/// Transport args for updating a single free-text field on a `RailwayModel`.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRailwayModelTextArgs {
    /// The railway model to update.
    pub railway_model_id: RailwayModelId,
    /// Which free-text field to update.
    pub field: RailwayModelTextField,
    /// New value. An empty string for `Details` clears the field; an empty
    /// string for `Description` is rejected by the domain.
    pub value: String,
    /// Language code for the translation to update.
    pub lang: Language,
}

impl From<UpdateRailwayModelTextArgs> for UpdateRailwayModelTextInput {
    fn from(args: UpdateRailwayModelTextArgs) -> Self {
        Self {
            railway_model_id: args.railway_model_id,
            field: args.field,
            value: args.value,
            lang: args.lang,
        }
    }
}

/// Arguments for updating a rolling stock's identification fields (series_code, road_number,
/// livery, depot) via an in-place inline edit.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRollingStockIdentificationArgs {
    /// The parent railway model.
    pub railway_model_id: RailwayModelId,
    /// The rolling stock unit to update.
    pub rolling_stock_id: RollingStockId,
    /// New series code (required, non-empty).
    pub series_code: String,
    /// Optional road number; empty string or absent means clear.
    pub road_number: Option<String>,
    /// Optional livery; empty string or absent means clear.
    pub livery: Option<String>,
    /// Optional depot; empty string or absent means clear.
    pub depot: Option<String>,
}

impl From<UpdateRollingStockIdentificationArgs> for UpdateRollingStockIdentificationInput {
    fn from(args: UpdateRollingStockIdentificationArgs) -> Self {
        Self {
            railway_model_id: args.railway_model_id,
            rolling_stock_id: args.rolling_stock_id,
            series_code: args.series_code,
            road_number: args.road_number,
            livery: args.livery,
            depot: args.depot,
        }
    }
}

// ---------------------------------------------------------------------------
// UpdateRailwayModelClassification args
// ---------------------------------------------------------------------------

/// Arguments for updating the constrained classification fields (scale, epoch, and/or category)
/// of a railway model via a badge picker.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRailwayModelClassificationArgs {
    /// The railway model to update.
    pub railway_model_id: RailwayModelId,
    /// New scale value, if being updated.
    pub scale: Option<Scale>,
    /// New epoch value, if being updated.
    pub epoch: Option<Epoch>,
    /// New category value, if being updated.
    pub category: Option<Category>,
}

impl From<UpdateRailwayModelClassificationArgs> for UpdateRailwayModelClassificationInput {
    fn from(args: UpdateRailwayModelClassificationArgs) -> Self {
        Self {
            railway_model_id: args.railway_model_id,
            scale: args.scale,
            epoch: args.epoch,
            category: args.category,
        }
    }
}

// ---------------------------------------------------------------------------
// UpdateRailwayModelDeliveryDate args
// ---------------------------------------------------------------------------

/// Arguments for updating the delivery date of a railway model.
///
/// Pass `delivery_date` as `None` or an empty string to clear the value;
/// otherwise the string is parsed via [`DeliveryDate::parse`].
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRailwayModelDeliveryDateArgs {
    /// The railway model to update.
    pub railway_model_id: RailwayModelId,
    /// New delivery date string, or `None` / empty string to clear.
    pub delivery_date: Option<String>,
}

impl TryFrom<UpdateRailwayModelDeliveryDateArgs> for UpdateRailwayModelDeliveryDateInput {
    type Error = DomainError;

    fn try_from(args: UpdateRailwayModelDeliveryDateArgs) -> Result<Self, Self::Error> {
        let delivery_date =
            match args.delivery_date.as_deref() {
                None | Some("") => None,
                Some(s) => Some(DeliveryDate::parse(s).map_err(|e| {
                    DomainError::Validation(format!("invalid delivery_date: {}", e))
                })?),
            };

        Ok(Self {
            railway_model_id: args.railway_model_id,
            delivery_date,
        })
    }
}

// ---------------------------------------------------------------------------
// UpdateRollingStockRailwayCompany args
// ---------------------------------------------------------------------------

/// Arguments for updating the railway company of a rolling stock unit via a badge picker.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRollingStockRailwayCompanyArgs {
    /// The parent railway model.
    pub railway_model_id: RailwayModelId,
    /// The rolling stock unit to update.
    pub rolling_stock_id: RollingStockId,
    /// The new railway company id (must exist in the database).
    pub railway_company_id: RailwayCompanyId,
}

impl From<UpdateRollingStockRailwayCompanyArgs> for UpdateRollingStockRailwayCompanyInput {
    fn from(args: UpdateRollingStockRailwayCompanyArgs) -> Self {
        Self {
            railway_model_id: args.railway_model_id,
            rolling_stock_id: args.rolling_stock_id,
            railway_company_id: args.railway_company_id,
        }
    }
}

// ---------------------------------------------------------------------------
// UpdateRollingStockCategory args
// ---------------------------------------------------------------------------

/// Arguments for changing the category (variant) of a rolling stock unit.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRollingStockCategoryArgs {
    /// The parent railway model.
    pub railway_model_id: RailwayModelId,
    /// The rolling stock unit to update.
    pub rolling_stock_id: RollingStockId,
    /// The new rolling stock category.
    pub category: RollingStockCategory,
}

impl From<UpdateRollingStockCategoryArgs> for UpdateRollingStockCategoryInput {
    fn from(args: UpdateRollingStockCategoryArgs) -> Self {
        Self {
            railway_model_id: args.railway_model_id,
            rolling_stock_id: args.rolling_stock_id,
            category: args.category,
        }
    }
}

/// Arguments for deleting a rolling stock unit from a railway model.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct DeleteRollingStockArgs {
    /// The parent railway model.
    pub railway_model_id: RailwayModelId,
    /// The rolling stock unit to delete.
    pub rolling_stock_id: RollingStockId,
}

impl From<DeleteRollingStockArgs> for DeleteRollingStockInput {
    fn from(args: DeleteRollingStockArgs) -> Self {
        Self {
            railway_model_id: args.railway_model_id,
            rolling_stock_id: args.rolling_stock_id,
        }
    }
}

// ---------------------------------------------------------------------------
// UpdateRollingStockSpecifications args
// ---------------------------------------------------------------------------

/// Full technical specification payload for a RollingStock unit.
/// Saves all four drawer sections (Identification, Technical, Control, Coupling) atomically.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRollingStockSpecificationsArgs {
    /// The parent railway model.
    pub railway_model_id: RailwayModelId,
    /// The rolling stock unit to update.
    pub rolling_stock_id: RollingStockId,

    // ── Identification ─────────────────────────────────────────────────────
    /// Required — must be non-empty.
    pub series_code: String,
    pub road_number: Option<String>,
    pub livery: Option<String>,
    pub depot: Option<String>,
    pub series: Option<String>,
    pub friendly_name: Option<String>,

    // ── Technical ──────────────────────────────────────────────────────────
    pub flywheel_fitted: Option<bool>,
    pub body_shell: Option<String>,
    pub chassis: Option<String>,
    pub interior_lights: Option<String>,
    pub lights: Option<String>,
    pub sprung_buffers: Option<bool>,

    // ── Control ────────────────────────────────────────────────────────────
    /// Only relevant for motorised rolling stock (Locomotive, EMU, Railcar).
    pub dcc_interface: Option<DccInterface>,
    pub control: Option<Control>,

    // ── Coupling ───────────────────────────────────────────────────────────
    pub coupling_socket: Option<String>,
    pub close_couplers: Option<bool>,
    pub digital_shunting: Option<bool>,
    pub is_dummy: Option<bool>,
}

impl TryFrom<UpdateRollingStockSpecificationsArgs> for UpdateRollingStockSpecificationsInput {
    type Error = DomainError;

    fn try_from(args: UpdateRollingStockSpecificationsArgs) -> Result<Self, Self::Error> {
        fn bool_to_flag(b: Option<bool>) -> Option<FeatureFlag> {
            b.map(|v| if v { FeatureFlag::Yes } else { FeatureFlag::No })
        }

        fn parse_opt<T: std::str::FromStr>(
            s: Option<String>,
            field: &str,
        ) -> Result<Option<T>, DomainError> {
            match s {
                None => Ok(None),
                Some(ref v) if v.is_empty() => Ok(None),
                Some(v) => v.parse::<T>().map(Some).map_err(|_| {
                    DomainError::Validation(format!("invalid value for {field}: {v}"))
                }),
            }
        }

        Ok(Self {
            railway_model_id: args.railway_model_id,
            rolling_stock_id: args.rolling_stock_id,
            spec: RollingStockSpecPatch {
                series_code: args.series_code,
                road_number: args.road_number,
                livery: args.livery,
                depot: args.depot,
                series: args.series,
                friendly_name: args.friendly_name,
                flywheel_fitted: bool_to_flag(args.flywheel_fitted),
                body_shell: parse_opt::<BodyShellType>(args.body_shell, "body_shell")?,
                chassis: parse_opt::<ChassisType>(args.chassis, "chassis")?,
                interior_lights: parse_opt::<FeatureFlag>(args.interior_lights, "interior_lights")?,
                lights: parse_opt::<FeatureFlag>(args.lights, "lights")?,
                sprung_buffers: bool_to_flag(args.sprung_buffers),
                dcc_interface: args.dcc_interface,
                control: args.control,
                coupling_socket: parse_opt::<CouplingSocket>(
                    args.coupling_socket,
                    "coupling_socket",
                )?,
                close_couplers: bool_to_flag(args.close_couplers),
                digital_shunting: bool_to_flag(args.digital_shunting),
                is_dummy: args.is_dummy,
            },
        })
    }
}

// ---------------------------------------------------------------------------
// UpsertRailwayModelTranslation args
// ---------------------------------------------------------------------------

/// Arguments for creating or replacing a translation for one language on a railway model.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpsertRailwayModelTranslationArgs {
    /// The railway model to update.
    #[garde(skip)]
    pub railway_model_id: RailwayModelId,
    /// Language code.
    #[garde(skip)]
    pub lang: Language,
    /// Description text. Required non-empty for "en"; optional for "it".
    #[garde(skip)]
    pub description: Option<String>,
    /// Details text. Optional for all languages.
    #[garde(skip)]
    pub details: Option<String>,
}

impl From<UpsertRailwayModelTranslationArgs> for UpsertRailwayModelTranslationInput {
    fn from(args: UpsertRailwayModelTranslationArgs) -> Self {
        Self {
            railway_model_id: args.railway_model_id,
            lang: args.lang,
            description: args.description,
            details: args.details,
        }
    }
}

// ---------------------------------------------------------------------------
// SearchRailwayModels args
// ---------------------------------------------------------------------------

/// Arguments for full-text search across railway model translations.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SearchRailwayModelsArgs {
    /// Search query. Minimum 2 characters.
    #[garde(length(min = 2, max = 500))]
    pub query: String,
}

impl From<SearchRailwayModelsArgs> for SearchRailwayModelsInput {
    fn from(args: SearchRailwayModelsArgs) -> Self {
        Self { query: args.query }
    }
}

/// Arguments for adding a new rolling stock variant to an existing Railway Model.
///
/// Follows ADR-8: Args suffix, derives Debug/Clone/Validate/Type/Deserialize.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct AddRollingStockToModelArgs {
    /// The parent railway model identifier (TRN string).
    pub railway_model_id: String,

    /// The railway company that operated this rolling stock (TRN string).
    pub railway_company_id: String,

    /// Rolling stock category. One of: LOCOMOTIVE, ELECTRIC_MULTIPLE_UNIT,
    /// FREIGHT_CAR, PASSENGER_CAR, RAILCAR.
    pub category: String,

    /// Series code identifying this variant (required, non-empty).
    #[garde(length(min = 1))]
    pub series_code: String,

    /// Optional road/fleet number.
    pub road_number: Option<String>,

    /// Optional livery description.
    pub livery: Option<String>,

    /// Optional depot name.
    pub depot: Option<String>,

    /// Optional control type (Control enum serialized as string, e.g. "DCC_READY").
    pub control: Option<String>,

    /// Optional DCC decoder interface connector (e.g. "NEXT_18", "MTC_21").
    pub dcc_interface: Option<String>,

    /// Optional coupling socket standard (e.g. "NEM_362", "NEM_360").
    pub coupling_socket: Option<String>,

    /// Optional short-coupler flag. Only meaningful when `coupling_socket` is provided.
    pub close_couplers: Option<bool>,

    /// Optional category-specific sub-type string (e.g. "ELECTRIC_LOCOMOTIVE", "OPEN_COACH").
    pub sub_type: Option<String>,

    /// Optional display/friendly name for the rolling stock.
    pub friendly_name: Option<String>,

    /// Optional prototype this rolling stock is linked to (TRN string).
    pub prototype_id: Option<String>,
    pub is_dummy: Option<bool>,
}

// ---------------------------------------------------------------------------
// UpdateRollingStockDcc args
// ---------------------------------------------------------------------------

/// Arguments for updating the control type, DCC interface, and length of a single rolling stock
/// unit. Only these three fields are updated; all other technical specifications remain unchanged.
#[derive(Debug, Clone, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRollingStockDccArgs {
    /// The parent railway model.
    pub railway_model_id: RailwayModelId,
    /// The rolling stock unit to update.
    pub rolling_stock_id: RollingStockId,
    /// Optional control type; `None` clears the field.
    pub control: Option<Control>,
    /// Optional DCC interface connector; `None` clears the field.
    pub dcc_interface: Option<DccInterface>,
    /// Optional length in millimeters; takes precedence over inches when both are provided.
    pub length_millimeters: Option<f64>,
    /// Optional length in inches; used only when `length_millimeters` is absent.
    pub length_inches: Option<f64>,
}

impl From<UpdateRollingStockDccArgs> for UpdateRollingStockDccInput {
    fn from(args: UpdateRollingStockDccArgs) -> Self {
        let length_over_buffers = match (
            args.length_millimeters.and_then(Decimal::from_f64),
            args.length_inches.and_then(Decimal::from_f64),
        ) {
            (Some(mm), _) if mm > Decimal::ZERO => {
                Some(LengthOverBuffers::from_millimeters(Length::Millimeters(mm)))
            }
            (_, Some(inches)) if inches > Decimal::ZERO => {
                Some(LengthOverBuffers::from_inches(Length::Inches(inches)))
            }
            _ => None,
        };

        Self {
            railway_model_id: args.railway_model_id,
            rolling_stock_id: args.rolling_stock_id,
            control: args.control,
            dcc_interface: args.dcc_interface,
            length_over_buffers,
        }
    }
}

// ---------------------------------------------------------------------------
// UpdateRollingStockSubcategory args
// ---------------------------------------------------------------------------

/// Arguments for changing the subcategory (type field) of a rolling stock unit.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRollingStockSubcategoryArgs {
    /// The parent railway model.
    pub railway_model_id: RailwayModelId,
    /// The rolling stock unit to update.
    pub rolling_stock_id: RollingStockId,
    /// The new subcategory string (e.g. "ELECTRIC_LOCOMOTIVE", "GONDOLA").
    pub subcategory: String,
}

impl From<UpdateRollingStockSubcategoryArgs> for UpdateRollingStockSubcategoryInput {
    fn from(args: UpdateRollingStockSubcategoryArgs) -> Self {
        Self {
            railway_model_id: args.railway_model_id,
            rolling_stock_id: args.rolling_stock_id,
            subcategory: args.subcategory,
        }
    }
}

// ---------------------------------------------------------------------------
// UpdateRollingStockServiceLevel args
// ---------------------------------------------------------------------------

/// Arguments for changing the service level of a rolling stock unit.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRollingStockServiceLevelArgs {
    /// The parent railway model.
    pub railway_model_id: RailwayModelId,
    /// The rolling stock unit to update.
    pub rolling_stock_id: RollingStockId,
    /// The new service level; `None` clears the field.
    pub service_level: Option<ServiceLevel>,
}

impl From<UpdateRollingStockServiceLevelArgs> for UpdateRollingStockServiceLevelInput {
    fn from(args: UpdateRollingStockServiceLevelArgs) -> Self {
        Self {
            railway_model_id: args.railway_model_id,
            rolling_stock_id: args.rolling_stock_id,
            service_level: args.service_level,
        }
    }
}

/// Arguments for setting (or clearing) the installed coupler on an owned rolling stock.
#[derive(Debug, Clone, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SetRollingStockCouplerArgs {
    /// The owned rolling stock to update.
    pub owned_rolling_stock_id: OwnedRollingStockId,
    /// The coupler type to install; `None` clears the current value.
    pub coupler_type_id: Option<CouplerTypeId>,
}

impl From<SetRollingStockCouplerArgs> for SetRollingStockCouplerInput {
    fn from(args: SetRollingStockCouplerArgs) -> Self {
        Self {
            owned_rolling_stock_id: args.owned_rolling_stock_id,
            coupler_type_id: args.coupler_type_id,
        }
    }
}

/// Result returned by `add_rolling_stock_to_model`.
///
/// Contains the catalog rolling stock ID and, when the model is part of a
/// collection, the owned rolling stock ID created for that collection item.
/// When the model exists only in a wishlist there is no collection ownership
/// record yet, so `owned_rolling_stock_id` is `None`.
#[derive(Debug, Clone, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct AddRollingStockResult {
    /// Catalog rolling stock identifier.
    pub rolling_stock_id: RollingStockId,
    /// The owned rolling stock row created for the collection item, if any.
    pub owned_rolling_stock_id: Option<OwnedRollingStockId>,
}

// ---------------------------------------------------------------------------
// Garde validation tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod garde_tests {
    use super::*;
    use garde::Validate;

    fn one_locomotive() -> CreateRollingStockArgs {
        CreateRollingStockArgs::Locomotive {
            railway_company_id: "DB".to_string(),
            friendly_name: "BR 01".to_string(),
            series_code: "01".to_string(),
            road_number: "01 001".to_string(),
            series: None,
            depot: None,
            livery: None,
            locomotive_type: "STEAM_LOCOMOTIVE".to_string(),
            is_dummy: None,
            control: None,
            dcc_interface: None,
            length_over_buffers: None,
            technical_specifications: None,
        }
    }

    fn valid_create_railway_model() -> CreateRailwayModelArgs {
        CreateRailwayModelArgs {
            manufacturer_id: "trn:manufacturer:acme".to_string(),
            product_code: "60100".to_string(),
            description: "Steam locomotive".to_string(),
            details: None,
            power_method: "DC".to_string(),
            scale: "H0".to_string(),
            epoch: "IV".to_string(),
            category: "LOCOMOTIVES".to_string(),
            delivery_date: None,
            availability_status: None,
            rolling_stocks: vec![one_locomotive()],
        }
    }

    // --- CreateRailwayModelArgs ---

    #[test]
    fn create_railway_model_valid_passes() {
        assert!(valid_create_railway_model().validate().is_ok());
    }

    #[test]
    fn create_railway_model_empty_manufacturer_id_fails() {
        let args = CreateRailwayModelArgs {
            manufacturer_id: "".to_string(),
            ..valid_create_railway_model()
        };
        let report = args.validate().unwrap_err();
        let paths: Vec<_> = report
            .into_inner()
            .into_iter()
            .map(|(p, _)| p.to_string())
            .collect();
        assert!(paths.iter().any(|p| p == "manufacturer_id"), "{paths:?}");
    }

    #[test]
    fn create_railway_model_product_code_too_long_fails() {
        let args = CreateRailwayModelArgs {
            product_code: "X".repeat(21),
            ..valid_create_railway_model()
        };
        let report = args.validate().unwrap_err();
        let paths: Vec<_> = report
            .into_inner()
            .into_iter()
            .map(|(p, _)| p.to_string())
            .collect();
        assert!(paths.iter().any(|p| p == "product_code"), "{paths:?}");
    }

    #[test]
    fn create_railway_model_empty_description_fails() {
        let args = CreateRailwayModelArgs {
            description: "".to_string(),
            ..valid_create_railway_model()
        };
        assert!(args.validate().is_err());
    }

    #[test]
    fn create_railway_model_invalid_power_method_fails() {
        let args = CreateRailwayModelArgs {
            power_method: "STEAM".to_string(),
            ..valid_create_railway_model()
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors.iter().any(|(p, e)| p.to_string() == "power_method"
                && e.to_string().contains("error_invalid_power_method")),
            "{errors:?}"
        );
    }

    #[test]
    fn create_railway_model_invalid_scale_fails() {
        let args = CreateRailwayModelArgs {
            scale: "HO".to_string(), // correct is "H0" (zero, not letter O)
            ..valid_create_railway_model()
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors
                .iter()
                .any(|(p, e)| p.to_string() == "scale"
                    && e.to_string().contains("error_invalid_scale")),
            "{errors:?}"
        );
    }

    #[test]
    fn create_railway_model_invalid_category_fails() {
        let args = CreateRailwayModelArgs {
            category: "TRAINS".to_string(),
            ..valid_create_railway_model()
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors.iter().any(|(p, e)| p.to_string() == "category"
                && e.to_string().contains("error_invalid_category")),
            "{errors:?}"
        );
    }

    #[test]
    fn create_railway_model_invalid_availability_status_fails() {
        let args = CreateRailwayModelArgs {
            availability_status: Some("SOLD_OUT".to_string()),
            ..valid_create_railway_model()
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors
                .iter()
                .any(|(p, e)| p.to_string() == "availability_status"
                    && e.to_string().contains("error_invalid_availability_status")),
            "{errors:?}"
        );
    }

    #[test]
    fn create_railway_model_empty_rolling_stocks_fails() {
        let args = CreateRailwayModelArgs {
            rolling_stocks: vec![],
            ..valid_create_railway_model()
        };
        let report = args.validate().unwrap_err();
        let paths: Vec<_> = report
            .into_inner()
            .into_iter()
            .map(|(p, _)| p.to_string())
            .collect();
        assert!(paths.iter().any(|p| p == "rolling_stocks"), "{paths:?}");
    }

    // --- LengthOverBuffersArgs ---

    #[test]
    fn length_over_buffers_valid_passes() {
        let args = LengthOverBuffersArgs {
            millimeters: Some(150.0),
            inches: Some(5.9),
        };
        assert!(args.validate().is_ok());
    }

    #[test]
    fn length_over_buffers_negative_mm_fails() {
        let args = LengthOverBuffersArgs {
            millimeters: Some(-1.0),
            inches: None,
        };
        let report = args.validate().unwrap_err();
        let paths: Vec<_> = report
            .into_inner()
            .into_iter()
            .map(|(p, _)| p.to_string())
            .collect();
        assert!(paths.iter().any(|p| p == "millimeters"), "{paths:?}");
    }

    #[test]
    fn length_over_buffers_negative_inches_fails() {
        let args = LengthOverBuffersArgs {
            millimeters: None,
            inches: Some(-0.1),
        };
        let report = args.validate().unwrap_err();
        let paths: Vec<_> = report
            .into_inner()
            .into_iter()
            .map(|(p, _)| p.to_string())
            .collect();
        assert!(paths.iter().any(|p| p == "inches"), "{paths:?}");
    }

    // --- TechnicalSpecificationsArgs ---

    fn valid_tech_specs() -> TechnicalSpecificationsArgs {
        TechnicalSpecificationsArgs {
            minimum_radius: Some(360.0),
            coupling: None,
            flywheel_fitted: Some("YES".to_string()),
            body_shell: Some("PLASTIC".to_string()),
            chassis: Some("METAL_DIE_CAST".to_string()),
            interior_lights: Some("NO".to_string()),
            lights: Some("YES".to_string()),
            sprung_buffers: Some("NOT_APPLICABLE".to_string()),
        }
    }

    #[test]
    fn tech_specs_valid_passes() {
        assert!(valid_tech_specs().validate().is_ok());
    }

    #[test]
    fn tech_specs_negative_radius_fails() {
        let args = TechnicalSpecificationsArgs {
            minimum_radius: Some(-10.0),
            ..valid_tech_specs()
        };
        let report = args.validate().unwrap_err();
        let paths: Vec<_> = report
            .into_inner()
            .into_iter()
            .map(|(p, _)| p.to_string())
            .collect();
        assert!(paths.iter().any(|p| p == "minimum_radius"), "{paths:?}");
    }

    #[test]
    fn tech_specs_invalid_flywheel_fails() {
        let args = TechnicalSpecificationsArgs {
            flywheel_fitted: Some("UNKNOWN".to_string()),
            ..valid_tech_specs()
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors
                .iter()
                .any(|(p, e)| p.to_string() == "flywheel_fitted"
                    && e.to_string().contains("error_invalid_feature_flag")),
            "{errors:?}"
        );
    }

    #[test]
    fn tech_specs_invalid_body_shell_fails() {
        let args = TechnicalSpecificationsArgs {
            body_shell: Some("WOOD".to_string()),
            ..valid_tech_specs()
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors.iter().any(|(p, e)| p.to_string() == "body_shell"
                && e.to_string().contains("error_invalid_body_shell_type")),
            "{errors:?}"
        );
    }

    #[test]
    fn tech_specs_invalid_chassis_fails() {
        let args = TechnicalSpecificationsArgs {
            chassis: Some("CARBON_FIBER".to_string()),
            ..valid_tech_specs()
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors.iter().any(|(p, e)| p.to_string() == "chassis"
                && e.to_string().contains("error_invalid_chassis_type")),
            "{errors:?}"
        );
    }

    // --- CouplingArgs ---

    #[test]
    fn coupling_valid_passes() {
        let args = CouplingArgs {
            socket: "NEM_362".to_string(),
            close_couplers: Some("YES".to_string()),
            digital_shunting: None,
        };
        assert!(args.validate().is_ok());
    }

    #[test]
    fn coupling_invalid_socket_fails() {
        let args = CouplingArgs {
            socket: "KADEE".to_string(),
            close_couplers: None,
            digital_shunting: None,
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors.iter().any(|(p, e)| p.to_string() == "socket"
                && e.to_string().contains("error_invalid_coupling_socket")),
            "{errors:?}"
        );
    }

    #[test]
    fn coupling_invalid_close_couplers_fails() {
        let args = CouplingArgs {
            socket: "NONE".to_string(),
            close_couplers: Some("MAYBE".to_string()),
            digital_shunting: None,
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors.iter().any(|(p, e)| p.to_string() == "close_couplers"
                && e.to_string().contains("error_invalid_feature_flag")),
            "{errors:?}"
        );
    }

    // --- SimplifiedRailwayModelArgs ---

    fn valid_simplified_model() -> SimplifiedRailwayModelArgs {
        SimplifiedRailwayModelArgs {
            manufacturer_id: "trn:manufacturer:acme".to_string(),
            product_code: "60100".to_string(),
            description: "Steam locomotive".to_string(),
            category: "LOCOMOTIVES".to_string(),
            scale: "H0".to_string(),
            epoch: "IV".to_string(),
            power_method: "DC".to_string(),
            rolling_stocks: vec![],
        }
    }

    #[test]
    fn simplified_model_valid_passes() {
        assert!(valid_simplified_model().validate().is_ok());
    }

    #[test]
    fn simplified_model_invalid_scale_fails() {
        let args = SimplifiedRailwayModelArgs {
            scale: "HO".to_string(),
            ..valid_simplified_model()
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors
                .iter()
                .any(|(p, e)| p.to_string() == "scale"
                    && e.to_string().contains("error_invalid_scale")),
            "{errors:?}"
        );
    }

    #[test]
    fn simplified_model_invalid_power_method_fails() {
        let args = SimplifiedRailwayModelArgs {
            power_method: "DIESEL".to_string(),
            ..valid_simplified_model()
        };
        let report = args.validate().unwrap_err();
        let errors: Vec<_> = report.into_inner();
        assert!(
            errors.iter().any(|(p, e)| p.to_string() == "power_method"
                && e.to_string().contains("error_invalid_power_method")),
            "{errors:?}"
        );
    }

    #[test]
    fn update_rolling_stock_specifications_try_from_maps_booleans_and_optionals() {
        let args = UpdateRollingStockSpecificationsArgs {
            railway_model_id: RailwayModelId::try_from("trn:railway-model:acme:rm-1")
                .expect("valid railway model id"),
            rolling_stock_id: RollingStockId::try_from(
                "trn:rolling-stock:3302b9a7-252c-4b41-8de2-eb71efb1888e",
            )
            .expect("valid rolling stock id"),
            series_code: "E444".to_string(),
            road_number: Some("001".to_string()),
            livery: Some("Blue".to_string()),
            depot: None,
            series: None,
            friendly_name: Some("Ligure".to_string()),
            flywheel_fitted: Some(true),
            body_shell: None,
            chassis: None,
            interior_lights: None,
            lights: None,
            sprung_buffers: Some(false),
            dcc_interface: None,
            control: None,
            coupling_socket: None,
            close_couplers: Some(true),
            digital_shunting: Some(false),
            is_dummy: Some(false),
        };

        let input =
            UpdateRollingStockSpecificationsInput::try_from(args).expect("conversion should work");

        assert_eq!(input.spec.series_code, "E444");
        assert_eq!(input.spec.road_number.as_deref(), Some("001"));
        assert_eq!(input.spec.friendly_name.as_deref(), Some("Ligure"));
        assert_eq!(input.spec.flywheel_fitted, Some(FeatureFlag::Yes));
        assert_eq!(input.spec.sprung_buffers, Some(FeatureFlag::No));
        assert_eq!(input.spec.close_couplers, Some(FeatureFlag::Yes));
        assert_eq!(input.spec.digital_shunting, Some(FeatureFlag::No));
        assert_eq!(input.spec.is_dummy, Some(false));
    }

    #[test]
    fn update_rolling_stock_specifications_try_from_rejects_invalid_coupling_socket() {
        let args = UpdateRollingStockSpecificationsArgs {
            railway_model_id: RailwayModelId::try_from("trn:railway-model:acme:rm-1")
                .expect("valid railway model id"),
            rolling_stock_id: RollingStockId::try_from(
                "trn:rolling-stock:3302b9a7-252c-4b41-8de2-eb71efb1888e",
            )
            .expect("valid rolling stock id"),
            series_code: "E444".to_string(),
            road_number: None,
            livery: None,
            depot: None,
            series: None,
            friendly_name: None,
            flywheel_fitted: None,
            body_shell: None,
            chassis: None,
            interior_lights: None,
            lights: None,
            sprung_buffers: None,
            dcc_interface: None,
            control: None,
            coupling_socket: Some("INVALID_SOCKET".to_string()),
            close_couplers: None,
            digital_shunting: None,
            is_dummy: None,
        };

        let err = match UpdateRollingStockSpecificationsInput::try_from(args) {
            Ok(_) => panic!("invalid coupling_socket should fail conversion"),
            Err(err) => err,
        };

        assert!(
            err.to_string().contains("coupling_socket"),
            "unexpected error: {err}"
        );
    }
}
