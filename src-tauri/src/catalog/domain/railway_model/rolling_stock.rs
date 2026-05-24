use crate::catalog::domain::railway_company::RailwayCompanyId;
use crate::catalog::domain::railway_model::ServiceLevel;
use crate::catalog::domain::railway_model::body_shell_type::BodyShellType;
use crate::catalog::domain::railway_model::category::{
    ElectricMultipleUnitType, FreightCarType, LocomotiveType, PassengerCarType, RailcarType,
    RollingStockCategory,
};
use crate::catalog::domain::railway_model::chassis_type::ChassisType;
use crate::catalog::domain::railway_model::control::Control;
use crate::catalog::domain::railway_model::coupling::Coupling;
use crate::catalog::domain::railway_model::coupling_socket::CouplingSocket;
use crate::catalog::domain::railway_model::dcc_interface::DccInterface;
use crate::catalog::domain::railway_model::feature_flag::FeatureFlag;
use crate::catalog::domain::railway_model::length_over_buffers::LengthOverBuffers;
use crate::catalog::domain::railway_model::rolling_stock_id::RollingStockId;
use crate::catalog::domain::railway_model::technical_specifications::TechnicalSpecifications;
use crate::core::domain::domain_error::DomainError;
#[cfg(test)]
use crate::core::domain::identifiers::Identifier;
#[cfg(test)]
use rand::RngExt;

/// A patch containing all fields that can be updated in the technical specification drawer.
#[derive(Debug, Clone)]
pub struct RollingStockSpecPatch {
    /// Series code — required, non-empty.
    pub series_code: String,
    /// Optional road number.
    pub road_number: Option<String>,
    /// Optional livery description.
    pub livery: Option<String>,
    /// Optional depot name.
    pub depot: Option<String>,
    /// Optional prototype series name.
    pub series: Option<String>,
    /// Optional friendly name (nickname).
    pub friendly_name: Option<String>,
    /// Optional flywheel flag.
    pub flywheel_fitted: Option<FeatureFlag>,
    /// Optional body shell material.
    pub body_shell: Option<BodyShellType>,
    /// Optional chassis material.
    pub chassis: Option<ChassisType>,
    /// Optional interior lights flag.
    pub interior_lights: Option<FeatureFlag>,
    /// Optional lights flag.
    pub lights: Option<FeatureFlag>,
    /// Optional sprung buffers flag.
    pub sprung_buffers: Option<FeatureFlag>,
    /// Optional DCC interface type.
    pub dcc_interface: Option<DccInterface>,
    /// Optional control type.
    pub control: Option<Control>,
    /// Optional coupling socket type.
    pub coupling_socket: Option<CouplingSocket>,
    /// Optional close couplers flag.
    pub close_couplers: Option<FeatureFlag>,
    /// Optional digital shunting flag.
    pub digital_shunting: Option<FeatureFlag>,
    /// Optional dummy unit flag.
    pub is_dummy: Option<bool>,
}

/// A focused patch for control type, DCC interface, and length — applied without
/// touching any other technical specification fields.
#[derive(Debug, Clone)]
pub struct RollingStockDccPatch {
    /// Optional control type.
    pub control: Option<Control>,
    /// Optional DCC interface connector.
    pub dcc_interface: Option<DccInterface>,
    /// Optional length over buffers.
    pub length_over_buffers: Option<LengthOverBuffers>,
}

#[derive(Debug, Clone)]
pub enum RollingStock {
    /// an electric multiple unit rolling stock
    ElectricMultipleUnit {
        /// the unique identifier for this rolling stock
        id: RollingStockId,
        /// the railway identifier for this rolling stock
        railway_id: RailwayCompanyId,
        /// the livery description
        livery: Option<String>,
        /// the overall length
        length_over_buffer: Option<LengthOverBuffers>,
        /// the technical specifications
        technical_specifications: Option<TechnicalSpecifications>,
        /// the electric multiple unit friendly name
        friendly_name: Option<String>,
        /// the series code (eg. a short code identifying the series)
        series_code: String,
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
    FreightCar {
        /// the unique identifier for this rolling stock
        id: RollingStockId,
        /// the railway identifier for this rolling stock
        railway_id: RailwayCompanyId,
        /// the livery description
        livery: Option<String>,
        /// the overall length
        length_over_buffer: Option<LengthOverBuffers>,
        /// the technical specifications
        technical_specifications: Option<TechnicalSpecifications>,
        /// the freight car friendly name
        friendly_name: Option<String>,
        /// the series code
        series_code: String,
        /// the identification marking for this freight car
        road_number: Option<String>,
        /// the freight car type
        freight_car_type: Option<FreightCarType>,
    },
    /// a locomotive rolling stock
    Locomotive {
        /// the unique identifier for this rolling stock
        id: RollingStockId,
        /// the railway identifier for this rolling stock
        railway_id: RailwayCompanyId,
        /// the livery description
        livery: Option<String>,
        /// the overall length
        length_over_buffer: Option<LengthOverBuffers>,
        /// the technical specification
        technical_specifications: Option<TechnicalSpecifications>,
        /// the locomotive friendly name
        friendly_name: Option<String>,
        /// the series code
        series_code: String,
        /// the identification marking for this locomotive
        road_number: Option<String>,
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
    PassengerCar {
        /// the unique identifier for this rolling stock
        id: RollingStockId,
        /// the railway identifier for this rolling stock
        railway_id: RailwayCompanyId,
        /// the livery description
        livery: Option<String>,
        /// the overall length
        length_over_buffer: Option<LengthOverBuffers>,
        /// the technical specifications
        technical_specifications: Option<TechnicalSpecifications>,
        /// the passenger car friendly name
        friendly_name: Option<String>,
        /// the series code
        series_code: String,
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
    Railcar {
        /// the unique identifier for this rolling stock
        id: RollingStockId,
        /// the railway identifier for this rolling stock
        railway_id: RailwayCompanyId,
        /// the livery description
        livery: Option<String>,
        /// the overall length
        length_over_buffer: Option<LengthOverBuffers>,
        /// the technical specifications
        technical_specifications: Option<TechnicalSpecifications>,
        /// the railcar friendly name
        friendly_name: Option<String>,
        /// the series code
        series_code: String,
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

#[cfg(test)]
impl fake::Dummy<fake::Faker> for RollingStock {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &fake::Faker, rng: &mut R) -> Self {
        let serial = rng.random_range(1..=999);
        let locomotive_type = match rng.random_range(0..=2) {
            0 => LocomotiveType::ElectricLocomotive,
            1 => LocomotiveType::DieselLocomotive,
            _ => LocomotiveType::SteamLocomotive,
        };

        Self::Locomotive {
            id: RollingStockId::default(),
            railway_id: RailwayCompanyId::new_from_parts(&["fs"]),
            livery: None,
            length_over_buffer: None,
            technical_specifications: None,
            friendly_name: None,
            series_code: format!("E.{serial}"),
            road_number: Some(format!("{serial:03}")),
            series: None,
            depot: None,
            locomotive_type,
            dcc_interface: None,
            control: None,
            is_dummy: rng.random(),
        }
    }
}

impl RollingStock {
    /// The category for this rolling stock
    pub fn category(&self) -> RollingStockCategory {
        match self {
            RollingStock::ElectricMultipleUnit { .. } => RollingStockCategory::ElectricMultipleUnit,
            RollingStock::Locomotive { .. } => RollingStockCategory::Locomotive,
            RollingStock::FreightCar { .. } => RollingStockCategory::FreightCar,
            RollingStock::PassengerCar { .. } => RollingStockCategory::PassengerCar,
            RollingStock::Railcar { .. } => RollingStockCategory::Railcar,
        }
    }

    /// The unique identifier for this rolling stock
    pub fn id_as_ref(&self) -> &RollingStockId {
        match self {
            RollingStock::ElectricMultipleUnit { id, .. } => id,
            RollingStock::Locomotive { id, .. } => id,
            RollingStock::FreightCar { id, .. } => id,
            RollingStock::PassengerCar { id, .. } => id,
            RollingStock::Railcar { id, .. } => id,
        }
    }

    /// The livery for this rolling stock
    pub fn livery(&self) -> Option<&str> {
        match self {
            RollingStock::ElectricMultipleUnit { livery, .. } => livery.as_deref(),
            RollingStock::Locomotive { livery, .. } => livery.as_deref(),
            RollingStock::FreightCar { livery, .. } => livery.as_deref(),
            RollingStock::PassengerCar { livery, .. } => livery.as_deref(),
            RollingStock::Railcar { livery, .. } => livery.as_deref(),
        }
    }

    /// The overall length for this rolling stock
    pub fn length_over_buffer(&self) -> Option<&LengthOverBuffers> {
        match self {
            RollingStock::ElectricMultipleUnit {
                length_over_buffer, ..
            } => length_over_buffer.as_ref(),
            RollingStock::Locomotive {
                length_over_buffer, ..
            } => length_over_buffer.as_ref(),
            RollingStock::FreightCar {
                length_over_buffer, ..
            } => length_over_buffer.as_ref(),
            RollingStock::PassengerCar {
                length_over_buffer, ..
            } => length_over_buffer.as_ref(),
            RollingStock::Railcar {
                length_over_buffer, ..
            } => length_over_buffer.as_ref(),
        }
    }

    /// The railway identifier for this rolling stock
    pub fn railway_id(&self) -> &RailwayCompanyId {
        match self {
            RollingStock::ElectricMultipleUnit { railway_id, .. } => railway_id,
            RollingStock::Locomotive { railway_id, .. } => railway_id,
            RollingStock::FreightCar { railway_id, .. } => railway_id,
            RollingStock::PassengerCar { railway_id, .. } => railway_id,
            RollingStock::Railcar { railway_id, .. } => railway_id,
        }
    }

    /// The road number for this rolling stock
    pub fn road_number(&self) -> Option<&str> {
        match self {
            RollingStock::ElectricMultipleUnit { road_number, .. } => road_number.as_deref(),
            RollingStock::Locomotive { road_number, .. } => road_number.as_deref(),
            RollingStock::FreightCar { road_number, .. } => road_number.as_deref(),
            RollingStock::PassengerCar { road_number, .. } => road_number.as_deref(),
            RollingStock::Railcar { road_number, .. } => road_number.as_deref(),
        }
    }

    /// The technical specification for this rolling stock
    pub fn technical_specifications(&self) -> Option<&TechnicalSpecifications> {
        match self {
            RollingStock::ElectricMultipleUnit {
                technical_specifications: tech_specs,
                ..
            } => tech_specs.as_ref(),
            RollingStock::Locomotive {
                technical_specifications: tech_specs,
                ..
            } => tech_specs.as_ref(),
            RollingStock::FreightCar {
                technical_specifications: tech_specs,
                ..
            } => tech_specs.as_ref(),
            RollingStock::PassengerCar {
                technical_specifications: tech_specs,
                ..
            } => tech_specs.as_ref(),
            RollingStock::Railcar {
                technical_specifications: tech_specs,
                ..
            } => tech_specs.as_ref(),
        }
    }

    /// The control method for this rolling stock
    pub fn control(&self) -> Option<Control> {
        match self {
            RollingStock::ElectricMultipleUnit {
                control: Some(control),
                ..
            } => Some(*control),
            RollingStock::Locomotive {
                control: Some(control),
                ..
            } => Some(*control),
            RollingStock::Railcar {
                control: Some(control),
                ..
            } => Some(*control),
            _ => None,
        }
    }

    /// The dcc interface for this rolling stock
    pub fn dcc_interface(&self) -> Option<DccInterface> {
        match self {
            RollingStock::ElectricMultipleUnit {
                dcc_interface: Some(dcc_interface),
                ..
            } => Some(*dcc_interface),
            RollingStock::Locomotive {
                dcc_interface: Some(dcc_interface),
                ..
            } => Some(*dcc_interface),
            RollingStock::Railcar {
                dcc_interface: Some(dcc_interface),
                ..
            } => Some(*dcc_interface),
            _ => None,
        }
    }

    /// Apply an identification patch (series_code, road_number, livery, depot) to this rolling
    /// stock and return a JSON object representing only the changed fields.
    pub fn apply_identification_patch(
        &mut self,
        series_code: String,
        road_number: Option<String>,
        livery: Option<String>,
        depot: Option<String>,
    ) -> serde_json::Value {
        macro_rules! set_field {
            ($variant_field:expr, $value:expr) => {
                $variant_field = $value;
            };
        }

        match self {
            RollingStock::Locomotive {
                series_code: sc,
                road_number: rn,
                livery: lv,
                depot: dp,
                ..
            } => {
                set_field!(*sc, series_code.clone());
                set_field!(*rn, road_number.clone());
                set_field!(*lv, livery.clone());
                set_field!(*dp, depot.clone());
            }
            RollingStock::ElectricMultipleUnit {
                series_code: sc,
                road_number: rn,
                livery: lv,
                depot: dp,
                ..
            } => {
                set_field!(*sc, series_code.clone());
                set_field!(*rn, road_number.clone());
                set_field!(*lv, livery.clone());
                set_field!(*dp, depot.clone());
            }
            RollingStock::Railcar {
                series_code: sc,
                road_number: rn,
                livery: lv,
                depot: dp,
                ..
            } => {
                set_field!(*sc, series_code.clone());
                set_field!(*rn, road_number.clone());
                set_field!(*lv, livery.clone());
                set_field!(*dp, depot.clone());
            }
            RollingStock::FreightCar {
                series_code: sc,
                road_number: rn,
                livery: lv,
                ..
            } => {
                set_field!(*sc, series_code.clone());
                set_field!(*rn, road_number.clone());
                set_field!(*lv, livery.clone());
            }
            RollingStock::PassengerCar {
                series_code: sc,
                road_number: rn,
                livery: lv,
                ..
            } => {
                set_field!(*sc, series_code.clone());
                set_field!(*rn, road_number.clone());
                set_field!(*lv, livery.clone());
            }
        }

        serde_json::json!({
            "series_code": series_code,
            "road_number": road_number,
            "livery": livery,
            "depot": depot,
        })
    }

    /// Apply a railway company change to this rolling stock and return a JSON patch.
    pub fn apply_railway_company(&mut self, company_id: RailwayCompanyId) -> serde_json::Value {
        let id_str = company_id.to_string();

        match self {
            RollingStock::Locomotive { railway_id, .. } => *railway_id = company_id,
            RollingStock::ElectricMultipleUnit { railway_id, .. } => *railway_id = company_id,
            RollingStock::FreightCar { railway_id, .. } => *railway_id = company_id,
            RollingStock::PassengerCar { railway_id, .. } => *railway_id = company_id,
            RollingStock::Railcar { railway_id, .. } => *railway_id = company_id,
        }

        serde_json::json!({ "railway_company_id": id_str })
    }

    /// Apply a full technical specification patch to this rolling stock and return a JSON patch.
    pub fn apply_specifications(&mut self, spec: RollingStockSpecPatch) -> serde_json::Value {
        let coupling = if spec.coupling_socket.is_some()
            || spec.close_couplers.is_some()
            || spec.digital_shunting.is_some()
        {
            Some(Coupling {
                socket: spec.coupling_socket,
                close_couplers: spec.close_couplers,
                digital_shunting: spec.digital_shunting,
            })
        } else {
            None
        };

        let tech_specs = TechnicalSpecifications {
            minimum_radius: None,
            coupling,
            flywheel_fitted: spec.flywheel_fitted,
            body_shell: spec.body_shell,
            chassis: spec.chassis,
            interior_lights: spec.interior_lights,
            lights: spec.lights,
            sprung_buffers: spec.sprung_buffers,
        };

        match self {
            RollingStock::Locomotive {
                series_code: sc,
                road_number: rn,
                livery: lv,
                depot: dp,
                series: sr,
                friendly_name: fn_,
                technical_specifications: ts,
                dcc_interface: di,
                control: ct,
                is_dummy: id,
                ..
            } => {
                *sc = spec.series_code.clone();
                *rn = spec.road_number.clone();
                *lv = spec.livery.clone();
                *dp = spec.depot.clone();
                *sr = spec.series.clone();
                *fn_ = spec.friendly_name.clone();
                *ts = Some(tech_specs);
                *di = spec.dcc_interface;
                *ct = spec.control;
                if let Some(dummy) = spec.is_dummy {
                    *id = dummy;
                }
            }
            RollingStock::ElectricMultipleUnit {
                series_code: sc,
                road_number: rn,
                livery: lv,
                depot: dp,
                series: sr,
                friendly_name: fn_,
                technical_specifications: ts,
                dcc_interface: di,
                control: ct,
                is_dummy: id,
                ..
            } => {
                *sc = spec.series_code.clone();
                *rn = spec.road_number.clone();
                *lv = spec.livery.clone();
                *dp = spec.depot.clone();
                *sr = spec.series.clone();
                *fn_ = spec.friendly_name.clone();
                *ts = Some(tech_specs);
                *di = spec.dcc_interface;
                *ct = spec.control;
                if let Some(dummy) = spec.is_dummy {
                    *id = dummy;
                }
            }
            RollingStock::Railcar {
                series_code: sc,
                road_number: rn,
                livery: lv,
                depot: dp,
                series: sr,
                friendly_name: fn_,
                technical_specifications: ts,
                dcc_interface: di,
                control: ct,
                is_dummy: id,
                ..
            } => {
                *sc = spec.series_code.clone();
                *rn = spec.road_number.clone();
                *lv = spec.livery.clone();
                *dp = spec.depot.clone();
                *sr = spec.series.clone();
                *fn_ = spec.friendly_name.clone();
                *ts = Some(tech_specs);
                *di = spec.dcc_interface;
                *ct = spec.control;
                if let Some(dummy) = spec.is_dummy {
                    *id = dummy;
                }
            }
            RollingStock::FreightCar {
                series_code: sc,
                road_number: rn,
                livery: lv,
                friendly_name: fn_,
                technical_specifications: ts,
                ..
            } => {
                *sc = spec.series_code.clone();
                *rn = spec.road_number.clone();
                *lv = spec.livery.clone();
                *fn_ = spec.friendly_name.clone();
                *ts = Some(tech_specs);
            }
            RollingStock::PassengerCar {
                series_code: sc,
                road_number: rn,
                livery: lv,
                series: sr,
                friendly_name: fn_,
                technical_specifications: ts,
                ..
            } => {
                *sc = spec.series_code.clone();
                *rn = spec.road_number.clone();
                *lv = spec.livery.clone();
                *sr = spec.series.clone();
                *fn_ = spec.friendly_name.clone();
                *ts = Some(tech_specs);
            }
        }

        serde_json::json!({
            "series_code": spec.series_code,
            "road_number": spec.road_number,
            "livery": spec.livery,
            "depot": spec.depot,
            "series": spec.series,
            "friendly_name": spec.friendly_name,
            "flywheel_fitted": spec.flywheel_fitted.map(|f| f.to_string()),
            "body_shell": spec.body_shell.map(|b| b.to_string()),
            "chassis": spec.chassis.map(|c| c.to_string()),
            "interior_lights": spec.interior_lights.map(|f| f.to_string()),
            "lights": spec.lights.map(|f| f.to_string()),
            "sprung_buffers": spec.sprung_buffers.map(|f| f.to_string()),
            "dcc_interface": spec.dcc_interface.map(|d| d.to_string()),
            "control": spec.control.map(|c| c.to_string()),
            "coupling_socket": spec.coupling_socket.map(|s| s.to_string()),
            "close_couplers": spec.close_couplers.map(|f| f.to_string()),
            "digital_shunting": spec.digital_shunting.map(|f| f.to_string()),
            "is_dummy": spec.is_dummy,
        })
    }

    /// Apply a focused DCC/length patch without touching any other specification fields.
    ///
    /// Only `Locomotive`, `ElectricMultipleUnit`, and `Railcar` variants have DCC fields;
    /// `FreightCar` and `PassengerCar` only update `length_over_buffer`.
    pub fn apply_dcc(&mut self, patch: RollingStockDccPatch) -> serde_json::Value {
        match self {
            RollingStock::Locomotive {
                dcc_interface: di,
                control: ct,
                length_over_buffer: lob,
                ..
            }
            | RollingStock::ElectricMultipleUnit {
                dcc_interface: di,
                control: ct,
                length_over_buffer: lob,
                ..
            }
            | RollingStock::Railcar {
                dcc_interface: di,
                control: ct,
                length_over_buffer: lob,
                ..
            } => {
                *di = patch.dcc_interface;
                *ct = patch.control;
                *lob = patch.length_over_buffers;
            }
            RollingStock::FreightCar {
                length_over_buffer: lob,
                ..
            }
            | RollingStock::PassengerCar {
                length_over_buffer: lob,
                ..
            } => {
                *lob = patch.length_over_buffers;
            }
        }

        let length_mm = patch
            .length_over_buffers
            .as_ref()
            .and_then(|l| l.millimeters())
            .map(|m| m.quantity());

        serde_json::json!({
            "dcc_control": patch.control.map(|c| c.to_string()),
            "dcc_interface": patch.dcc_interface.map(|d| d.to_string()),
            "dcc_length_mm": length_mm,
        })
    }

    /// Change the rolling stock variant to match `new_category`, preserving all shared fields
    /// and applying sensible defaults for type-specific fields.  Returns a JSON patch suitable
    /// for the repository's `update_rolling_stock_from_patch`.
    pub fn apply_category(&mut self, new_category: RollingStockCategory) -> serde_json::Value {
        // Extract shared fields from the current variant.
        let (
            id,
            railway_id,
            livery,
            length_over_buffer,
            technical_specifications,
            friendly_name,
            series_code,
            road_number,
        ) = match self.clone() {
            RollingStock::Locomotive {
                id,
                railway_id,
                livery,
                length_over_buffer,
                technical_specifications,
                friendly_name,
                series_code,
                road_number,
                ..
            }
            | RollingStock::ElectricMultipleUnit {
                id,
                railway_id,
                livery,
                length_over_buffer,
                technical_specifications,
                friendly_name,
                series_code,
                road_number,
                ..
            }
            | RollingStock::FreightCar {
                id,
                railway_id,
                livery,
                length_over_buffer,
                technical_specifications,
                friendly_name,
                series_code,
                road_number,
                ..
            }
            | RollingStock::PassengerCar {
                id,
                railway_id,
                livery,
                length_over_buffer,
                technical_specifications,
                friendly_name,
                series_code,
                road_number,
                ..
            }
            | RollingStock::Railcar {
                id,
                railway_id,
                livery,
                length_over_buffer,
                technical_specifications,
                friendly_name,
                series_code,
                road_number,
                ..
            } => (
                id,
                railway_id,
                livery,
                length_over_buffer,
                technical_specifications,
                friendly_name,
                series_code,
                road_number,
            ),
        };

        // Extract powered-variant-only fields (None when coming from FreightCar/PassengerCar).
        let (series, depot, dcc_interface, control, is_dummy) = match self.clone() {
            RollingStock::Locomotive {
                series,
                depot,
                dcc_interface,
                control,
                is_dummy,
                ..
            }
            | RollingStock::ElectricMultipleUnit {
                series,
                depot,
                dcc_interface,
                control,
                is_dummy,
                ..
            }
            | RollingStock::Railcar {
                series,
                depot,
                dcc_interface,
                control,
                is_dummy,
                ..
            } => (series, depot, dcc_interface, control, is_dummy),
            _ => (None, None, None, None, false),
        };

        let category_str = new_category.to_string();

        *self = match new_category {
            RollingStockCategory::Locomotive => RollingStock::Locomotive {
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
                locomotive_type: LocomotiveType::ElectricLocomotive,
                dcc_interface,
                control,
                is_dummy,
            },
            RollingStockCategory::ElectricMultipleUnit => RollingStock::ElectricMultipleUnit {
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
                electric_multiple_unit_type: ElectricMultipleUnitType::MotorCar,
                dcc_interface,
                control,
                is_dummy,
            },
            RollingStockCategory::FreightCar => RollingStock::FreightCar {
                id,
                railway_id,
                livery,
                length_over_buffer,
                technical_specifications,
                friendly_name,
                series_code,
                road_number,
                freight_car_type: None,
            },
            RollingStockCategory::PassengerCar => RollingStock::PassengerCar {
                id,
                railway_id,
                livery,
                length_over_buffer,
                technical_specifications,
                friendly_name,
                series_code,
                road_number,
                series,
                passenger_car_type: None,
                service_level: None,
            },
            RollingStockCategory::Railcar => RollingStock::Railcar {
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
                railcar_type: RailcarType::PowerCar,
                dcc_interface,
                control,
                is_dummy,
            },
        };

        serde_json::json!({ "rolling_stock_category": category_str })
    }

    /// Change the subcategory (type field) of this rolling stock and return a JSON patch.
    ///
    /// Returns `Err(DomainError::Validation)` when `subcategory` is not a valid variant
    /// for the current category.
    pub fn apply_subcategory(
        &mut self,
        subcategory: String,
    ) -> Result<serde_json::Value, DomainError> {
        match self {
            RollingStock::Locomotive {
                locomotive_type, ..
            } => {
                let t = subcategory.parse::<LocomotiveType>().map_err(|_| {
                    DomainError::Validation(format!("invalid locomotive type: {subcategory}"))
                })?;
                *locomotive_type = t;
                Ok(serde_json::json!({ "locomotive_type": subcategory }))
            }
            RollingStock::FreightCar {
                freight_car_type, ..
            } => {
                let t = subcategory.parse::<FreightCarType>().map_err(|_| {
                    DomainError::Validation(format!("invalid freight car type: {subcategory}"))
                })?;
                *freight_car_type = Some(t);
                Ok(serde_json::json!({ "freight_car_type": subcategory }))
            }
            RollingStock::PassengerCar {
                passenger_car_type, ..
            } => {
                let t = subcategory.parse::<PassengerCarType>().map_err(|_| {
                    DomainError::Validation(format!("invalid passenger car type: {subcategory}"))
                })?;
                *passenger_car_type = Some(t);
                Ok(serde_json::json!({ "passenger_car_type": subcategory }))
            }
            RollingStock::ElectricMultipleUnit {
                electric_multiple_unit_type,
                ..
            } => {
                let t = subcategory
                    .parse::<ElectricMultipleUnitType>()
                    .map_err(|_| {
                        DomainError::Validation(format!(
                            "invalid electric multiple unit type: {subcategory}"
                        ))
                    })?;
                *electric_multiple_unit_type = t;
                Ok(serde_json::json!({ "electric_multiple_unit_type": subcategory }))
            }
            RollingStock::Railcar { railcar_type, .. } => {
                let t = subcategory.parse::<RailcarType>().map_err(|_| {
                    DomainError::Validation(format!("invalid railcar type: {subcategory}"))
                })?;
                *railcar_type = t;
                Ok(serde_json::json!({ "railcar_type": subcategory }))
            }
        }
    }

    /// Change the service level of this rolling stock and return a JSON patch.
    ///
    /// Only `PassengerCar` actually stores a service level; for all other variants
    /// this is a no-op but still returns a valid patch so the call is safe.
    pub fn apply_service_level(
        &mut self,
        service_level: Option<ServiceLevel>,
    ) -> serde_json::Value {
        if let RollingStock::PassengerCar {
            service_level: sl, ..
        } = self
        {
            *sl = service_level;
        }
        serde_json::json!({ "service_level": service_level })
    }

    /// Return true if the rolling stock has a decoder, false otherwise
    pub fn with_decoder(&self) -> bool {
        match self {
            RollingStock::ElectricMultipleUnit {
                control: Some(control),
                ..
            } => control.has_decoder(),
            RollingStock::Locomotive {
                control: Some(control),
                ..
            } => control.has_decoder(),
            RollingStock::Railcar {
                control: Some(control),
                ..
            } => control.has_decoder(),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::domain::railway_model::TechnicalSpecificationsBuilder;
    use crate::core::domain::length::Length;
    use pretty_assertions::assert_eq;
    use rust_decimal_macros::dec;
    use uuid::Uuid;

    fn sample_length_over_buffers() -> LengthOverBuffers {
        LengthOverBuffers::from_millimeters(Length::Millimeters(dec!(42)))
    }

    fn sample_technical_specifications() -> TechnicalSpecifications {
        TechnicalSpecificationsBuilder::default().build()
    }

    #[test]
    fn locomotive_accessors_return_expected_values() {
        let id = RollingStockId::from_uuid(&Uuid::new_v4());
        let railway = RailwayCompanyId::try_from("trn:railway-company:fs").unwrap();

        let loco = RollingStock::Locomotive {
            id: id.clone(),
            railway_id: railway.clone(),
            livery: Some("Blue Livery".to_string()),
            length_over_buffer: None,
            technical_specifications: None,
            friendly_name: Some("Friendly".to_string()),
            series_code: "SC-1".to_string(),
            road_number: Some("42".to_string()),
            series: None,
            depot: None,
            locomotive_type: LocomotiveType::ElectricLocomotive,
            dcc_interface: None,
            control: None,
            is_dummy: false,
        };

        assert_eq!(loco.category(), RollingStockCategory::Locomotive);
        assert_eq!(loco.id_as_ref(), &id);
        assert_eq!(loco.railway_id(), &railway);
        assert_eq!(loco.livery(), Some("Blue Livery"));
        assert_eq!(loco.road_number(), Some("42"));
    }

    #[test]
    fn control_and_dcc_interface_and_decoder_behavior() {
        let id = RollingStockId::from_uuid(&Uuid::new_v4());
        let railway = RailwayCompanyId::try_from("trn:railway-company:fs").unwrap();

        let loco = RollingStock::Locomotive {
            id: id.clone(),
            railway_id: railway,
            livery: None,
            length_over_buffer: None,
            technical_specifications: None,
            friendly_name: None,
            series_code: "SC-2".to_string(),
            road_number: None,
            series: None,
            depot: None,
            locomotive_type: LocomotiveType::DieselLocomotive,
            dcc_interface: Some(DccInterface::Nem651),
            control: Some(Control::DccFitted),
            is_dummy: false,
        };

        assert_eq!(loco.control(), Some(Control::DccFitted));
        assert_eq!(loco.dcc_interface(), Some(DccInterface::Nem651));
        assert!(loco.with_decoder());
    }

    #[test]
    fn freightcar_has_no_decoder_and_no_control() {
        let id = RollingStockId::from_uuid(&Uuid::new_v4());
        let railway = RailwayCompanyId::try_from("trn:railway-company:fs").unwrap();

        let freight = RollingStock::FreightCar {
            id: id.clone(),
            railway_id: railway,
            livery: None,
            length_over_buffer: None,
            technical_specifications: None,
            friendly_name: None,
            series_code: "FC-1".to_string(),
            road_number: None,
            freight_car_type: None,
        };

        assert_eq!(freight.control(), None);
        assert_eq!(freight.dcc_interface(), None);
        assert!(!freight.with_decoder());
    }

    #[test]
    fn electric_multiple_unit_accessors_and_decoder_behavior() {
        let id = RollingStockId::from_uuid(&Uuid::new_v4());
        let railway = RailwayCompanyId::try_from("trn:railway-company:fs").unwrap();

        let emu = RollingStock::ElectricMultipleUnit {
            id: id.clone(),
            railway_id: railway.clone(),
            livery: Some("Green Livery".to_string()),
            length_over_buffer: None,
            technical_specifications: None,
            friendly_name: Some("EMU Friendly".to_string()),
            series_code: "EMU-1".to_string(),
            road_number: Some("EMU-7".to_string()),
            series: Some("Series A".to_string()),
            depot: None,
            electric_multiple_unit_type: ElectricMultipleUnitType::DrivingCar,
            dcc_interface: Some(DccInterface::Nem651),
            control: Some(Control::DccFitted),
            is_dummy: false,
        };

        assert_eq!(emu.category(), RollingStockCategory::ElectricMultipleUnit);
        assert_eq!(emu.id_as_ref(), &id);
        assert_eq!(emu.railway_id(), &railway);
        assert_eq!(emu.livery(), Some("Green Livery"));
        assert_eq!(emu.road_number(), Some("EMU-7"));
        assert_eq!(emu.control(), Some(Control::DccFitted));
        assert_eq!(emu.dcc_interface(), Some(DccInterface::Nem651));
        assert!(emu.with_decoder());
    }
    #[test]
    fn railcar_accessors_and_decoder_behavior() {
        let id = RollingStockId::from_uuid(&Uuid::new_v4());
        let railway = RailwayCompanyId::try_from("trn:railway-company:fs").unwrap();

        let railcar = RollingStock::Railcar {
            id: id.clone(),
            railway_id: railway.clone(),
            livery: Some("Red".to_string()),
            length_over_buffer: None,
            technical_specifications: None,
            friendly_name: Some("Railcar Friendly".to_string()),
            series_code: "RC-1".to_string(),
            road_number: Some("100".to_string()),
            series: None,
            depot: None,
            railcar_type: RailcarType::PowerCar,
            dcc_interface: Some(DccInterface::Plux8),
            control: Some(Control::DccSound),
            is_dummy: false,
        };

        assert_eq!(railcar.category(), RollingStockCategory::Railcar);
        assert_eq!(railcar.id_as_ref(), &id);
        assert_eq!(railcar.railway_id(), &railway);
        assert_eq!(railcar.livery(), Some("Red"));
        assert_eq!(railcar.road_number(), Some("100"));
        assert_eq!(railcar.control(), Some(Control::DccSound));
        assert_eq!(railcar.dcc_interface(), Some(DccInterface::Plux8));
        assert!(railcar.with_decoder());
    }

    #[test]
    fn length_and_technical_specifications_accessors_return_expected_values() {
        let id = RollingStockId::from_uuid(&Uuid::new_v4());
        let railway = RailwayCompanyId::try_from("trn:railway-company:fs").unwrap();
        let length = sample_length_over_buffers();
        let technical_specifications = sample_technical_specifications();

        let locomotives = vec![
            RollingStock::Locomotive {
                id: id.clone(),
                railway_id: railway.clone(),
                livery: None,
                length_over_buffer: Some(length),
                technical_specifications: Some(technical_specifications.clone()),
                friendly_name: None,
                series_code: "L-1".to_string(),
                road_number: None,
                series: None,
                depot: None,
                locomotive_type: LocomotiveType::DieselLocomotive,
                dcc_interface: None,
                control: None,
                is_dummy: false,
            },
            RollingStock::ElectricMultipleUnit {
                id: id.clone(),
                railway_id: railway.clone(),
                livery: None,
                length_over_buffer: Some(length),
                technical_specifications: Some(technical_specifications.clone()),
                friendly_name: None,
                series_code: "E-1".to_string(),
                road_number: None,
                series: None,
                depot: None,
                electric_multiple_unit_type: ElectricMultipleUnitType::PowerCar,
                dcc_interface: None,
                control: None,
                is_dummy: false,
            },
            RollingStock::FreightCar {
                id: id.clone(),
                railway_id: railway.clone(),
                livery: None,
                length_over_buffer: Some(length),
                technical_specifications: Some(technical_specifications.clone()),
                friendly_name: None,
                series_code: "F-1".to_string(),
                road_number: None,
                freight_car_type: None,
            },
            RollingStock::PassengerCar {
                id: id.clone(),
                railway_id: railway.clone(),
                livery: None,
                length_over_buffer: Some(length),
                technical_specifications: Some(technical_specifications.clone()),
                friendly_name: None,
                series_code: "P-1".to_string(),
                road_number: None,
                series: None,
                passenger_car_type: None,
                service_level: None,
            },
            RollingStock::Railcar {
                id,
                railway_id: railway,
                livery: None,
                length_over_buffer: Some(length),
                technical_specifications: Some(technical_specifications.clone()),
                friendly_name: None,
                series_code: "R-1".to_string(),
                road_number: None,
                series: None,
                depot: None,
                railcar_type: RailcarType::PowerCar,
                dcc_interface: None,
                control: None,
                is_dummy: false,
            },
        ];

        for rolling_stock in locomotives {
            assert_eq!(rolling_stock.length_over_buffer(), Some(&length));
            assert_eq!(
                rolling_stock.technical_specifications(),
                Some(&technical_specifications)
            );
        }
    }
}
