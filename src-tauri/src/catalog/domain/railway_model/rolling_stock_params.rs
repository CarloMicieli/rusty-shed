use crate::catalog::application::railway_model_use_case_input::{
    CouplingInput, CreateRollingStockInput, LengthOverBuffersInput, TechnicalSpecificationsInput,
};
use crate::catalog::domain::railway_company::RailwayCompanyId;
use crate::catalog::domain::railway_model::{
    Control, Coupling, CouplingSocket, DccInterface, ElectricMultipleUnitType, FreightCarType,
    LengthOverBuffers, LocomotiveType, PassengerCarType, Radius, RailcarType, ServiceLevel,
    TechnicalSpecifications,
};
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::validation::ValidationContext;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

#[derive(Debug, Clone)]
pub enum RollingStockParams {
    /// an electric multiple unit rolling stock
    ElectricMultipleUnitParams {
        railway_company_id: RailwayCompanyId,
        /// the livery description
        livery: Option<String>,
        /// the overall length
        length_over_buffers: Option<LengthOverBuffers>,
        /// the technical specifications
        technical_specifications: Option<TechnicalSpecifications>,
        /// the electric multiple unit friendly name
        friendly_name: String,
        /// the series code (eg. a short code identifying the series)
        series_code: Option<String>,
        /// the identification marking for this electric multiple unit
        road_number: Option<String>,
        /// the prototype series information
        series: Option<String>,
        /// the depot name
        depot: Option<String>,
        /// the electric multiple unit type
        electric_multiple_unit_type: ElectricMultipleUnitType,
        /// the dcc interface
        dcc_interface: Option<DccInterface>,
        /// the control
        control: Option<Control>,
        /// indicate whether the rolling stock has a motor or not
        is_dummy: bool,
    },
    /// a freight car rolling stock
    FreightCarParams {
        railway_company_id: RailwayCompanyId,
        /// the livery description
        livery: Option<String>,
        /// the overall length
        length_over_buffers: Option<LengthOverBuffers>,
        /// the technical specifications
        technical_specifications: Option<TechnicalSpecifications>,
        /// the freight car friendly name
        friendly_name: String,
        /// the series code
        series_code: Option<String>,
        /// the prototype series information
        series: Option<String>,
        /// the identification marking for this freight car
        road_number: Option<String>,
        /// the freight car type
        freight_car_type: Option<FreightCarType>,
    },
    /// a locomotive rolling stock
    LocomotiveParams {
        railway_company_id: RailwayCompanyId,
        /// the livery description
        livery: Option<String>,
        /// the overall length
        length_over_buffers: Option<LengthOverBuffers>,
        /// the technical specification
        technical_specifications: Option<TechnicalSpecifications>,
        /// the locomotive friendly name
        friendly_name: String,
        /// the series code
        series_code: Option<String>,
        /// the identification marking for this locomotive
        road_number: String,
        /// the prototype series information
        series: Option<String>,
        /// the depot name
        depot: Option<String>,
        /// the locomotive type
        locomotive_type: LocomotiveType,
        /// the dcc interface
        dcc_interface: Option<DccInterface>,
        /// the control
        control: Option<Control>,
        /// indicate whether the rolling stock has a motor or not
        is_dummy: bool,
    },
    /// a passenger car rolling stock
    PassengerCarParams {
        railway_company_id: RailwayCompanyId,
        /// the livery description
        livery: Option<String>,
        /// the overall length
        length_over_buffers: Option<LengthOverBuffers>,
        /// the technical specifications
        technical_specifications: Option<TechnicalSpecifications>,
        /// the passenger car friendly name
        friendly_name: String,
        /// the series code
        series_code: Option<String>,
        /// the identification marking for this passenger car
        road_number: Option<String>,
        /// the prototype series information
        series: Option<String>,
        /// the passenger car type
        passenger_car_type: Option<PassengerCarType>,
        /// the travel class for this passenger car. Passenger cars can have multiple service
        /// levels (ie, '1st/2nd')
        service_level: Option<ServiceLevel>,
    },
    /// a railcar rolling stock
    RailcarParams {
        /// the railway for this rolling stock
        railway_company_id: RailwayCompanyId,
        /// the livery description
        livery: Option<String>,
        /// the overall length
        length_over_buffers: Option<LengthOverBuffers>,
        /// the technical specifications
        technical_specifications: Option<TechnicalSpecifications>,
        /// the railcar friendly name
        friendly_name: String,
        /// the series code
        series_code: Option<String>,
        /// the identification marking for this railcar
        road_number: Option<String>,
        /// the railcar series
        series: Option<String>,
        /// the depot name
        depot: Option<String>,
        /// the railcar type
        railcar_type: RailcarType,
        /// the dcc interface
        dcc_interface: Option<DccInterface>,
        /// the control
        control: Option<Control>,
        /// indicate whether the rolling stock has a motor or not
        is_dummy: bool,
    },
}

impl RollingStockParams {
    /// Get a reference to the technical specifications, if any.
    /// Returns `None` if no technical specifications are set.
    pub fn technical_specifications(&self) -> Option<&TechnicalSpecifications> {
        let technical_specifications = match self {
            RollingStockParams::ElectricMultipleUnitParams {
                technical_specifications,
                ..
            } => technical_specifications,
            RollingStockParams::LocomotiveParams {
                technical_specifications,
                ..
            } => technical_specifications,
            RollingStockParams::PassengerCarParams {
                technical_specifications,
                ..
            } => technical_specifications,
            RollingStockParams::FreightCarParams {
                technical_specifications,
                ..
            } => technical_specifications,
            RollingStockParams::RailcarParams {
                technical_specifications,
                ..
            } => technical_specifications,
        };

        technical_specifications.as_ref()
    }

    /// Get a reference to the length over buffers, if any.
    /// Returns `None` if no length over buffers is set.
    pub fn length_over_buffers(&self) -> Option<&LengthOverBuffers> {
        let length_over_buffers = match self {
            RollingStockParams::ElectricMultipleUnitParams {
                length_over_buffers,
                ..
            } => length_over_buffers,
            RollingStockParams::LocomotiveParams {
                length_over_buffers,
                ..
            } => length_over_buffers,
            RollingStockParams::PassengerCarParams {
                length_over_buffers,
                ..
            } => length_over_buffers,
            RollingStockParams::FreightCarParams {
                length_over_buffers,
                ..
            } => length_over_buffers,
            RollingStockParams::RailcarParams {
                length_over_buffers,
                ..
            } => length_over_buffers,
        };

        length_over_buffers.as_ref()
    }
}

impl TryFrom<CreateRollingStockInput> for RollingStockParams {
    type Error = DomainError;

    fn try_from(input: CreateRollingStockInput) -> Result<Self, Self::Error> {
        let mut ctx = ValidationContext::default();

        let params = match input {
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
                let company_id = validate_company_id(&mut ctx, &railway_company_id);
                let locomotive_type =
                    ctx.collect("locomotive_type", locomotive_type.parse::<LocomotiveType>());
                let control = validate_opt_parse(&mut ctx, "control", control);
                let dcc = validate_opt_parse(&mut ctx, "dcc_interface", dcc_interface);
                let length = validate_length(&mut ctx, length_over_buffers);
                let specs = validate_specs(&mut ctx, technical_specifications, true);

                // Checkpoint: Stop if validation failed
                ctx.finish()?;

                RollingStockParams::LocomotiveParams {
                    railway_company_id: company_id.unwrap(),
                    friendly_name,
                    series_code: format_series_code(series_code),
                    road_number,
                    series,
                    depot,
                    livery,
                    locomotive_type: locomotive_type.unwrap(),
                    is_dummy: is_dummy.unwrap_or(false),
                    control,
                    dcc_interface: dcc,
                    length_over_buffers: length,
                    technical_specifications: specs,
                }
            }
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
                let company_id = validate_company_id(&mut ctx, &railway_company_id);
                let electric_multiple_unit_type = ctx.collect(
                    "electric_multiple_unit_type",
                    electric_multiple_unit_type.parse::<ElectricMultipleUnitType>(),
                );
                let control = validate_opt_parse(&mut ctx, "control", control);
                let dcc = validate_opt_parse(&mut ctx, "dcc_interface", dcc_interface);
                let length = validate_length(&mut ctx, length_over_buffers);
                let specs = validate_specs(&mut ctx, technical_specifications, true);

                // Checkpoint: Stop if validation failed
                ctx.finish()?;

                RollingStockParams::ElectricMultipleUnitParams {
                    railway_company_id: company_id.unwrap(),
                    friendly_name,
                    series_code: format_series_code(series_code),
                    road_number,
                    series,
                    depot,
                    livery,
                    electric_multiple_unit_type: electric_multiple_unit_type.unwrap(),
                    is_dummy: is_dummy.unwrap_or(false),
                    control,
                    dcc_interface: dcc,
                    length_over_buffers: length,
                    technical_specifications: specs,
                }
            }
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
                let company_id = validate_company_id(&mut ctx, &railway_company_id);
                let railcar_type = railcar_type
                    .and_then(|s| ctx.collect("railcar_type", s.parse::<RailcarType>()));
                let control = validate_opt_parse(&mut ctx, "control", control);
                let dcc = validate_opt_parse(&mut ctx, "dcc_interface", dcc_interface);
                let length = validate_length(&mut ctx, length_over_buffers);
                let specs = validate_specs(&mut ctx, technical_specifications, true);

                // Checkpoint: Stop if validation failed
                ctx.finish()?;

                RollingStockParams::RailcarParams {
                    railway_company_id: company_id.unwrap(),
                    friendly_name,
                    series_code: format_series_code(series_code),
                    road_number,
                    series,
                    depot,
                    livery,
                    railcar_type: railcar_type.unwrap(),
                    is_dummy: is_dummy.unwrap_or(false),
                    control,
                    dcc_interface: dcc,
                    length_over_buffers: length,
                    technical_specifications: specs,
                }
            }
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
                let company_id = validate_company_id(&mut ctx, &railway_company_id);
                let passenger_car_type = ctx.collect(
                    "passenger_car_type",
                    passenger_car_type.parse::<PassengerCarType>(),
                );
                let length = validate_length(&mut ctx, length_over_buffers);
                let specs = validate_specs(&mut ctx, technical_specifications, true);
                let service_level = service_level
                    .and_then(|s| ctx.collect("service_level", s.parse::<ServiceLevel>()));

                // Checkpoint: Stop if validation failed
                ctx.finish()?;

                RollingStockParams::PassengerCarParams {
                    railway_company_id: company_id.unwrap(),
                    friendly_name,
                    series_code: format_series_code(series_code),
                    road_number,
                    series,
                    livery,
                    passenger_car_type,
                    length_over_buffers: length,
                    technical_specifications: specs,
                    service_level,
                }
            }
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
                let company_id = validate_company_id(&mut ctx, &railway_company_id);
                let freight_car_type = freight_car_type
                    .and_then(|s| ctx.collect("freight_car_type", s.parse::<FreightCarType>()));
                let length = validate_length(&mut ctx, length_over_buffers);
                let specs = validate_specs(&mut ctx, technical_specifications, true);

                // Checkpoint: Stop if validation failed
                ctx.finish()?;

                RollingStockParams::FreightCarParams {
                    railway_company_id: company_id.unwrap(),
                    friendly_name,
                    series_code: format_series_code(series_code),
                    road_number,
                    series,
                    livery,
                    freight_car_type,
                    length_over_buffers: length,
                    technical_specifications: specs,
                }
            }
        };

        Ok(params)
    }
}

fn format_series_code(code: String) -> Option<String> {
    if code.is_empty() { None } else { Some(code) }
}

fn validate_company_id(ctx: &mut ValidationContext, id: &str) -> Option<RailwayCompanyId> {
    ctx.collect("railway_company_id", RailwayCompanyId::try_from(id))
}

fn validate_opt_parse<T>(
    ctx: &mut ValidationContext,
    field: &str,
    value: Option<String>,
) -> Option<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    value.and_then(|s| ctx.collect(field, s.parse::<T>()))
}

fn validate_length(
    ctx: &mut ValidationContext,
    lob: Option<LengthOverBuffersInput>,
) -> Option<LengthOverBuffers> {
    lob.and_then(|l| {
        let mm = l.millimeters.and_then(Decimal::from_f64);
        let inches = l.inches.and_then(Decimal::from_f64);
        ctx.collect("length_over_buffers", LengthOverBuffers::new(inches, mm))
    })
}

fn validate_specs(
    ctx: &mut ValidationContext,
    specs: Option<TechnicalSpecificationsInput>,
    include_full_details: bool, // Locomotives need more than FreightCars
) -> Option<TechnicalSpecifications> {
    specs.map(|ts| {
        let min_radius = ts.minimum_radius.and_then(|v| {
            Decimal::from_f64(v)
                .map(|d| {
                    ctx.collect(
                        "technical_specifications.minimum_radius",
                        Radius::from_millimeters(d),
                    )
                })
                .unwrap_or_else(|| {
                    ctx.push_error(
                        "technical_specifications.minimum_radius",
                        "invalid_format",
                        "invalid decimal",
                    );
                    None
                })
        });

        if !include_full_details {
            return TechnicalSpecifications {
                minimum_radius: min_radius,
                ..Default::default()
            };
        }

        // Detailed parsing for Locomotives/EMUs
        TechnicalSpecifications {
            minimum_radius: min_radius,
            coupling: validate_coupling(ctx, ts.coupling),
            flywheel_fitted: validate_opt_parse(
                ctx,
                "technical_specifications.flywheel_fitted",
                ts.flywheel_fitted,
            ),
            body_shell: validate_opt_parse(
                ctx,
                "technical_specifications.body_shell",
                ts.body_shell,
            ),
            // ... add others
            ..Default::default()
        }
    })
}

fn validate_coupling(
    ctx: &mut ValidationContext,
    coupling_input: Option<CouplingInput>,
) -> Option<Coupling> {
    coupling_input.and_then(|c| {
        // 1. Mandatory field for a coupling: The Socket
        let socket = ctx.collect(
            "technical_specifications.coupling.socket",
            c.socket.parse::<CouplingSocket>(),
        );

        // 2. Optional Feature Flags
        let close = validate_opt_parse(
            ctx,
            "technical_specifications.coupling.close_couplers",
            c.close_couplers,
        );
        let digital = validate_opt_parse(
            ctx,
            "technical_specifications.coupling.digital_shunting",
            c.digital_shunting,
        );

        // 3. We only return the Coupling object if the mandatory socket was valid
        socket.map(|s| Coupling {
            socket: Some(s),
            close_couplers: close,
            digital_shunting: digital,
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_convert_locomotive_input_success() {
        let input = CreateRollingStockInput::Locomotive {
            railway_company_id: "RY-1".to_string(),
            friendly_name: "Test Loco".to_string(),
            series_code: "SC".to_string(),
            road_number: "RN-1".to_string(),
            series: None,
            depot: None,
            livery: None,
            locomotive_type: "ELECTRIC_LOCOMOTIVE".to_string(),
            is_dummy: Some(false),
            control: None,
            dcc_interface: None,
            length_over_buffers: None,
            technical_specifications: None,
        };

        let params = RollingStockParams::try_from(input).expect("conversion should succeed");

        match params {
            RollingStockParams::LocomotiveParams {
                friendly_name,
                road_number,
                locomotive_type,
                ..
            } => {
                assert_eq!(friendly_name, "Test Loco");
                assert_eq!(road_number, "RN-1");
                assert_eq!(locomotive_type, LocomotiveType::ElectricLocomotive);
            }
            _ => panic!("expected locomotive params"),
        }
    }

    #[test]
    fn it_should_accumulate_validation_errors() {
        let input = CreateRollingStockInput::Locomotive {
            railway_company_id: "".to_string(),
            friendly_name: "Test Loco".to_string(),
            series_code: "".to_string(),
            road_number: "RN-1".to_string(),
            series: None,
            depot: None,
            livery: None,
            locomotive_type: "NOT_A_TYPE".to_string(),
            is_dummy: Some(false),
            control: Some("INVALID_CONTROL".to_string()),
            dcc_interface: Some("INVALID_DCC".to_string()),
            length_over_buffers: None,
            technical_specifications: None,
        };

        let res = RollingStockParams::try_from(input);
        assert!(res.is_err());
        if let Err(DomainError::ValidationError(map)) = res {
            assert!(map.contains_key("railway_company_id"));
            assert!(map.contains_key("locomotive_type"));
            assert!(map.contains_key("control"));
            assert!(map.contains_key("dcc_interface"));
        } else {
            panic!("expected validation error");
        }
    }
}
