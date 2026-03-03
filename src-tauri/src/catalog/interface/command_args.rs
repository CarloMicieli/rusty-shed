use garde::Validate;
use serde::Deserialize;

use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

use crate::{
    catalog::application::{
        CouplingInput, CreateRailwayModelInput, CreateRollingStockInput, LengthOverBuffersInput,
        RailwayModelTextField, SaveRailwayModelInput, SearchRailwayModelsInput,
        SimplifiedRollingStockInput, TechnicalSpecificationsInput,
        UpdateRailwayModelClassificationInput, UpdateRailwayModelTextInput,
        UpdateRollingStockDccInput, UpdateRollingStockIdentificationInput,
        UpdateRollingStockRailwayCompanyInput, UpdateRollingStockSpecificationsInput,
        UpsertRailwayModelTranslationInput,
    },
    catalog::domain::railway_company::RailwayCompanyId,
    catalog::domain::railway_model::{
        BodyShellType, ChassisType, Control, CouplingSocket, DccInterface, Epoch, FeatureFlag,
        LengthOverBuffers, RailwayModelId, RollingStockId, RollingStockSpecPatch,
    },
    catalog::domain::scale::Scale,
    core::domain::length::Length,
    core::domain::{Language, domain_error::DomainError},
};

/// Arguments for creating a new railway model (transport from IPC to application).
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct CreateRailwayModelArgs {
    /// ID of the manufacturer.
    pub manufacturer_id: String,
    /// Product code of the railway model.
    pub product_code: String,
    /// Description of the railway model.
    pub description: String,
    /// Additional details about the railway model.
    pub details: Option<String>,
    /// Power method of the railway model.
    pub power_method: String,
    /// Scale of the railway model.
    pub scale: String,
    /// Epoch of the railway model.
    pub epoch: String,
    /// Category of the railway model.
    pub category: String,
    /// Optional delivery date of the railway model.
    pub delivery_date: Option<String>,
    /// Optional availability status of the railway model.
    pub availability_status: Option<String>,
    /// Rolling stock items associated with the railway model.
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
    /// Length in millimeters.
    pub millimeters: Option<f64>,
    /// Length in inches.
    pub inches: Option<f64>,
}

/// Optional technical specifications for a rolling stock item.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct TechnicalSpecificationsArgs {
    /// Minimum radius the rolling stock can navigate.
    pub minimum_radius: Option<f64>,
    /// Coupling details.
    pub coupling: Option<CouplingArgs>,
    /// Flywheel details.
    pub flywheel_fitted: Option<String>,
    /// Body shell details.
    pub body_shell: Option<String>,
    /// Chassis details.
    pub chassis: Option<String>,
    /// Presence of interior lighting.
    pub interior_lights: Option<String>,
    /// Presence of headlights or other lights.
    pub lights: Option<String>,
    /// Presence of sprung buffers.
    pub sprung_buffers: Option<String>,
}

/// Coupling details for a rolling stock item.
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct CouplingArgs {
    /// Type of coupling used.
    pub socket: String,
    /// Type of coupling head used.
    pub close_couplers: Option<String>,
    /// Presence of digital shunting couplers.
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
#[derive(Debug, Clone, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SimplifiedRailwayModelArgs {
    pub manufacturer_id: String,
    pub product_code: String,
    pub description: String,
    pub category: String,
    pub scale: String,
    pub epoch: String,
    pub power_method: String,
    pub rolling_stocks: Vec<SimplifiedRollingStockArgs>,
}

#[derive(Debug, Clone, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SimplifiedRollingStockArgs {
    pub railway_company_id: String,
    pub series_code: String,
    pub road_number: Option<String>,
    pub locomotive_type: Option<String>,
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
                locomotive_type: rs.locomotive_type,
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

/// Arguments for updating the constrained classification fields (scale and/or epoch) of a
/// railway model via a badge picker.
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
}

impl From<UpdateRailwayModelClassificationArgs> for UpdateRailwayModelClassificationInput {
    fn from(args: UpdateRailwayModelClassificationArgs) -> Self {
        Self {
            railway_model_id: args.railway_model_id,
            scale: args.scale,
            epoch: args.epoch,
        }
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

    // ── Technical ──────────────────────────────────────────────────────────
    pub flywheel_fitted: Option<bool>,
    pub body_shell: Option<String>,
    pub chassis: Option<String>,
    pub interior_lights: Option<String>,
    pub lights: Option<String>,

    // ── Control ────────────────────────────────────────────────────────────
    /// Only relevant for motorised rolling stock (Locomotive, EMU, Railcar).
    pub dcc_interface: Option<DccInterface>,
    pub control: Option<Control>,

    // ── Coupling ───────────────────────────────────────────────────────────
    pub coupling_socket: Option<String>,
    pub close_couplers: Option<bool>,
    pub digital_shunting: Option<bool>,
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
                flywheel_fitted: bool_to_flag(args.flywheel_fitted),
                body_shell: parse_opt::<BodyShellType>(args.body_shell, "body_shell")?,
                chassis: parse_opt::<ChassisType>(args.chassis, "chassis")?,
                interior_lights: parse_opt::<FeatureFlag>(args.interior_lights, "interior_lights")?,
                lights: parse_opt::<FeatureFlag>(args.lights, "lights")?,
                dcc_interface: args.dcc_interface,
                control: args.control,
                coupling_socket: parse_opt::<CouplingSocket>(
                    args.coupling_socket,
                    "coupling_socket",
                )?,
                close_couplers: bool_to_flag(args.close_couplers),
                digital_shunting: bool_to_flag(args.digital_shunting),
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
