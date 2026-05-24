use crate::catalog::application::{
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
            } => build_locomotive_params(
                ValidationContext::default(),
                LocomotiveInputData {
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
                },
            ),
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
            } => build_electric_multiple_unit_params(
                ValidationContext::default(),
                ElectricMultipleUnitInputData {
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
                },
            ),
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
            } => build_railcar_params(
                ValidationContext::default(),
                RailcarInputData {
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
                },
            ),
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
            } => build_passenger_car_params(
                ValidationContext::default(),
                PassengerCarInputData {
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
                },
            ),
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
            } => build_freight_car_params(
                ValidationContext::default(),
                FreightCarInputData {
                    railway_company_id,
                    friendly_name,
                    series_code,
                    road_number,
                    series,
                    livery,
                    freight_car_type,
                    length_over_buffers,
                    technical_specifications,
                },
            ),
        }
    }
}

struct LocomotiveInputData {
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
    length_over_buffers: Option<LengthOverBuffersInput>,
    technical_specifications: Option<TechnicalSpecificationsInput>,
}

struct ElectricMultipleUnitInputData {
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
    length_over_buffers: Option<LengthOverBuffersInput>,
    technical_specifications: Option<TechnicalSpecificationsInput>,
}

struct RailcarInputData {
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
    length_over_buffers: Option<LengthOverBuffersInput>,
    technical_specifications: Option<TechnicalSpecificationsInput>,
}

struct PassengerCarInputData {
    railway_company_id: String,
    friendly_name: String,
    series_code: String,
    road_number: Option<String>,
    series: Option<String>,
    livery: Option<String>,
    passenger_car_type: String,
    service_level: Option<String>,
    length_over_buffers: Option<LengthOverBuffersInput>,
    technical_specifications: Option<TechnicalSpecificationsInput>,
}

struct FreightCarInputData {
    railway_company_id: String,
    friendly_name: String,
    series_code: String,
    road_number: Option<String>,
    series: Option<String>,
    livery: Option<String>,
    freight_car_type: Option<String>,
    length_over_buffers: Option<LengthOverBuffersInput>,
    technical_specifications: Option<TechnicalSpecificationsInput>,
}

fn build_locomotive_params(
    mut ctx: ValidationContext,
    input: LocomotiveInputData,
) -> Result<RollingStockParams, DomainError> {
    let company_id = validate_company_id(&mut ctx, &input.railway_company_id);
    let locomotive_type = ctx.collect(
        "locomotive_type",
        input.locomotive_type.parse::<LocomotiveType>(),
    );
    let control = validate_opt_parse(&mut ctx, "control", input.control);
    let dcc = validate_opt_parse(&mut ctx, "dcc_interface", input.dcc_interface);
    let length = validate_length(&mut ctx, input.length_over_buffers);
    let specs = validate_specs(&mut ctx, input.technical_specifications, true);
    ctx.finish()?;
    let railway_company_id = company_id
        .ok_or_else(|| DomainError::Validation("invalid railway_company_id".to_string()))?;
    let locomotive_type = locomotive_type
        .ok_or_else(|| DomainError::Validation("invalid locomotive_type".to_string()))?;

    Ok(RollingStockParams::LocomotiveParams {
        railway_company_id,
        friendly_name: input.friendly_name,
        series_code: format_series_code(input.series_code),
        road_number: input.road_number,
        series: input.series,
        depot: input.depot,
        livery: input.livery,
        locomotive_type,
        is_dummy: input.is_dummy.unwrap_or(false),
        control,
        dcc_interface: dcc,
        length_over_buffers: length,
        technical_specifications: specs,
    })
}

fn build_electric_multiple_unit_params(
    mut ctx: ValidationContext,
    input: ElectricMultipleUnitInputData,
) -> Result<RollingStockParams, DomainError> {
    let company_id = validate_company_id(&mut ctx, &input.railway_company_id);
    let emu_type = ctx.collect(
        "electric_multiple_unit_type",
        input
            .electric_multiple_unit_type
            .parse::<ElectricMultipleUnitType>(),
    );
    let control = validate_opt_parse(&mut ctx, "control", input.control);
    let dcc = validate_opt_parse(&mut ctx, "dcc_interface", input.dcc_interface);
    let length = validate_length(&mut ctx, input.length_over_buffers);
    let specs = validate_specs(&mut ctx, input.technical_specifications, true);
    ctx.finish()?;
    let railway_company_id = company_id
        .ok_or_else(|| DomainError::Validation("invalid railway_company_id".to_string()))?;
    let electric_multiple_unit_type = emu_type.ok_or_else(|| {
        DomainError::Validation("invalid electric_multiple_unit_type".to_string())
    })?;

    Ok(RollingStockParams::ElectricMultipleUnitParams {
        railway_company_id,
        friendly_name: input.friendly_name,
        series_code: format_series_code(input.series_code),
        road_number: input.road_number,
        series: input.series,
        depot: input.depot,
        livery: input.livery,
        electric_multiple_unit_type,
        is_dummy: input.is_dummy.unwrap_or(false),
        control,
        dcc_interface: dcc,
        length_over_buffers: length,
        technical_specifications: specs,
    })
}

fn build_railcar_params(
    mut ctx: ValidationContext,
    input: RailcarInputData,
) -> Result<RollingStockParams, DomainError> {
    let company_id = validate_company_id(&mut ctx, &input.railway_company_id);
    let railcar_type = input
        .railcar_type
        .and_then(|s| ctx.collect("railcar_type", s.parse::<RailcarType>()));
    let control = validate_opt_parse(&mut ctx, "control", input.control);
    let dcc = validate_opt_parse(&mut ctx, "dcc_interface", input.dcc_interface);
    let length = validate_length(&mut ctx, input.length_over_buffers);
    let specs = validate_specs(&mut ctx, input.technical_specifications, true);
    ctx.finish()?;
    let railway_company_id = company_id
        .ok_or_else(|| DomainError::Validation("invalid railway_company_id".to_string()))?;
    let railcar_type =
        railcar_type.ok_or_else(|| DomainError::Validation("invalid railcar_type".to_string()))?;

    Ok(RollingStockParams::RailcarParams {
        railway_company_id,
        friendly_name: input.friendly_name,
        series_code: format_series_code(input.series_code),
        road_number: input.road_number,
        series: input.series,
        depot: input.depot,
        livery: input.livery,
        railcar_type,
        is_dummy: input.is_dummy.unwrap_or(false),
        control,
        dcc_interface: dcc,
        length_over_buffers: length,
        technical_specifications: specs,
    })
}

fn build_passenger_car_params(
    mut ctx: ValidationContext,
    input: PassengerCarInputData,
) -> Result<RollingStockParams, DomainError> {
    let company_id = validate_company_id(&mut ctx, &input.railway_company_id);
    let passenger_car_type = ctx.collect(
        "passenger_car_type",
        input.passenger_car_type.parse::<PassengerCarType>(),
    );
    let length = validate_length(&mut ctx, input.length_over_buffers);
    let specs = validate_specs(&mut ctx, input.technical_specifications, true);
    let service_level = input
        .service_level
        .and_then(|s| ctx.collect("service_level", s.parse::<ServiceLevel>()));
    ctx.finish()?;
    let railway_company_id = company_id
        .ok_or_else(|| DomainError::Validation("invalid railway_company_id".to_string()))?;

    Ok(RollingStockParams::PassengerCarParams {
        railway_company_id,
        friendly_name: input.friendly_name,
        series_code: format_series_code(input.series_code),
        road_number: input.road_number,
        series: input.series,
        livery: input.livery,
        passenger_car_type,
        length_over_buffers: length,
        technical_specifications: specs,
        service_level,
    })
}

fn build_freight_car_params(
    mut ctx: ValidationContext,
    input: FreightCarInputData,
) -> Result<RollingStockParams, DomainError> {
    let company_id = validate_company_id(&mut ctx, &input.railway_company_id);
    let freight_car_type = input
        .freight_car_type
        .and_then(|s| ctx.collect("freight_car_type", s.parse::<FreightCarType>()));
    let length = validate_length(&mut ctx, input.length_over_buffers);
    let specs = validate_specs(&mut ctx, input.technical_specifications, true);
    ctx.finish()?;
    let railway_company_id = company_id
        .ok_or_else(|| DomainError::Validation("invalid railway_company_id".to_string()))?;

    Ok(RollingStockParams::FreightCarParams {
        railway_company_id,
        friendly_name: input.friendly_name,
        series_code: format_series_code(input.series_code),
        road_number: input.road_number,
        series: input.series,
        livery: input.livery,
        freight_car_type,
        length_over_buffers: length,
        technical_specifications: specs,
    })
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
    use crate::catalog::domain::railway_model::TechnicalSpecificationsBuilder;
    use crate::core::domain::length::Length;
    use pretty_assertions::assert_eq;
    use rust_decimal_macros::dec;

    fn sample_length_over_buffers() -> LengthOverBuffers {
        LengthOverBuffers::from_millimeters(Length::Millimeters(dec!(42)))
    }

    fn sample_technical_specifications() -> TechnicalSpecifications {
        TechnicalSpecificationsBuilder::default().build()
    }

    fn assert_accessor_values(params: &RollingStockParams) {
        let expected_length = sample_length_over_buffers();
        let expected_technical_specifications = sample_technical_specifications();

        assert_eq!(params.length_over_buffers(), Some(&expected_length));
        assert_eq!(
            params.technical_specifications(),
            Some(&expected_technical_specifications)
        );
    }

    #[test]
    fn it_should_return_the_optional_accessors_for_all_variants() {
        let railway_company_id = RailwayCompanyId::try_from("trn:railway-company:ry-1")
            .expect("valid railway company id");
        let length_over_buffers = sample_length_over_buffers();
        let technical_specifications = sample_technical_specifications();

        let params = vec![
            RollingStockParams::ElectricMultipleUnitParams {
                railway_company_id: railway_company_id.clone(),
                livery: None,
                length_over_buffers: Some(length_over_buffers.clone()),
                technical_specifications: Some(technical_specifications.clone()),
                friendly_name: "EMU".to_string(),
                series_code: None,
                road_number: None,
                series: None,
                depot: None,
                electric_multiple_unit_type: ElectricMultipleUnitType::DrivingCar,
                dcc_interface: None,
                control: None,
                is_dummy: false,
            },
            RollingStockParams::FreightCarParams {
                railway_company_id: railway_company_id.clone(),
                livery: None,
                length_over_buffers: Some(length_over_buffers.clone()),
                technical_specifications: Some(technical_specifications.clone()),
                friendly_name: "Freight".to_string(),
                series_code: None,
                series: None,
                road_number: None,
                freight_car_type: None,
            },
            RollingStockParams::LocomotiveParams {
                railway_company_id: railway_company_id.clone(),
                livery: None,
                length_over_buffers: Some(length_over_buffers.clone()),
                technical_specifications: Some(technical_specifications.clone()),
                friendly_name: "Loco".to_string(),
                series_code: None,
                road_number: "123".to_string(),
                series: None,
                depot: None,
                locomotive_type: LocomotiveType::ElectricLocomotive,
                dcc_interface: None,
                control: None,
                is_dummy: false,
            },
            RollingStockParams::PassengerCarParams {
                railway_company_id: railway_company_id.clone(),
                livery: None,
                length_over_buffers: Some(length_over_buffers.clone()),
                technical_specifications: Some(technical_specifications.clone()),
                friendly_name: "Passenger".to_string(),
                series_code: None,
                road_number: None,
                series: None,
                passenger_car_type: None,
                service_level: None,
            },
            RollingStockParams::RailcarParams {
                railway_company_id,
                livery: None,
                length_over_buffers: Some(length_over_buffers),
                technical_specifications: Some(technical_specifications),
                friendly_name: "Railcar".to_string(),
                series_code: None,
                road_number: None,
                series: None,
                depot: None,
                railcar_type: RailcarType::PowerCar,
                dcc_interface: None,
                control: None,
                is_dummy: false,
            },
        ];

        for param in &params {
            assert_accessor_values(param);
        }
    }

    #[test]
    fn it_should_convert_locomotive_input_success() {
        let input = CreateRollingStockInput::Locomotive {
            railway_company_id: "trn:railway-company:ry-1".to_string(),
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

    #[test]
    fn it_should_convert_electric_multiple_unit_input_success() {
        let input = CreateRollingStockInput::ElectricMultipleUnit {
            railway_company_id: "trn:railway-company:ry-1".to_string(),
            friendly_name: "Test EMU".to_string(),
            series_code: "EMU-SC".to_string(),
            road_number: Some("EMU-1".to_string()),
            series: None,
            depot: None,
            livery: None,
            electric_multiple_unit_type: "DRIVING_CAR".to_string(),
            is_dummy: Some(true),
            control: None,
            dcc_interface: None,
            length_over_buffers: None,
            technical_specifications: None,
        };

        let params = RollingStockParams::try_from(input).expect("conversion should succeed");
        assert!(matches!(
            params,
            RollingStockParams::ElectricMultipleUnitParams { is_dummy: true, .. }
        ));
    }

    #[test]
    fn it_should_convert_passenger_car_input_success() {
        let input = CreateRollingStockInput::PassengerCar {
            railway_company_id: "trn:railway-company:ry-1".to_string(),
            friendly_name: "Test Passenger".to_string(),
            series_code: "PC-SC".to_string(),
            road_number: Some("PC-1".to_string()),
            series: None,
            livery: None,
            passenger_car_type: "OPEN_COACH".to_string(),
            service_level: Some("1".to_string()),
            length_over_buffers: None,
            technical_specifications: None,
        };

        let params = RollingStockParams::try_from(input).expect("conversion should succeed");
        assert!(matches!(
            params,
            RollingStockParams::PassengerCarParams {
                service_level: Some(ServiceLevel::First),
                ..
            }
        ));
    }

    #[test]
    fn it_should_convert_freight_car_input_success() {
        let input = CreateRollingStockInput::FreightCar {
            railway_company_id: "trn:railway-company:ry-1".to_string(),
            friendly_name: "Test Freight".to_string(),
            series_code: "FC-SC".to_string(),
            road_number: Some("FC-1".to_string()),
            series: None,
            livery: None,
            freight_car_type: Some("FLAT_WAGON".to_string()),
            length_over_buffers: None,
            technical_specifications: None,
        };

        let params = RollingStockParams::try_from(input).expect("conversion should succeed");
        assert!(matches!(
            params,
            RollingStockParams::FreightCarParams { .. }
        ));
    }

    #[test]
    fn it_should_convert_railcar_input_success() {
        let input = CreateRollingStockInput::Railcar {
            railway_company_id: "trn:railway-company:ry-1".to_string(),
            friendly_name: "Test Railcar".to_string(),
            series_code: "RC-SC".to_string(),
            road_number: Some("RC-1".to_string()),
            series: None,
            depot: None,
            livery: None,
            railcar_type: Some("POWER_CAR".to_string()),
            is_dummy: Some(false),
            control: None,
            dcc_interface: None,
            length_over_buffers: None,
            technical_specifications: None,
        };

        let params = RollingStockParams::try_from(input).expect("conversion should succeed");
        assert!(matches!(params, RollingStockParams::RailcarParams { .. }));
    }
}
