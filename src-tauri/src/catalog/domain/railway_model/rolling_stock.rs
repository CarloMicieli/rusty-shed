use crate::catalog::domain::railway_model::ServiceLevel;
use crate::catalog::domain::railway_model::category::{
    ElectricMultipleUnitType, FreightCarType, LocomotiveType, PassengerCarType, RailcarType,
    RollingStockCategory,
};
use crate::catalog::domain::railway_model::control::Control;
use crate::catalog::domain::railway_model::dcc_interface::DccInterface;
use crate::catalog::domain::railway_model::length_over_buffers::LengthOverBuffers;
use crate::catalog::domain::railway_model::rolling_stock_id::RollingStockId;
use crate::catalog::domain::railway_model::rolling_stock_railway::RollingStockRailway;
use crate::catalog::domain::railway_model::technical_specifications::TechnicalSpecifications;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, specta::Type)]
#[serde(tag = "category")]
#[specta(tag = "category", content = "data")]
pub enum RollingStock {
    /// an electric multiple unit rolling stock
    ElectricMultipleUnit {
        /// the unique identifier for this rolling stock
        id: RollingStockId,
        /// the railway for this rolling stock
        railway: RollingStockRailway,
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
        /// the railway for this rolling stock
        railway: RollingStockRailway,
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
        /// the railway for this rolling stock
        railway: RollingStockRailway,
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
        /// the railway for this rolling stock
        railway: RollingStockRailway,
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
        /// the railway for this rolling stock
        railway: RollingStockRailway,
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

    /// The railway company for this rolling stock
    pub fn railway(&self) -> &RollingStockRailway {
        match self {
            RollingStock::ElectricMultipleUnit { railway, .. } => railway,
            RollingStock::Locomotive { railway, .. } => railway,
            RollingStock::FreightCar { railway, .. } => railway,
            RollingStock::PassengerCar { railway, .. } => railway,
            RollingStock::Railcar { railway, .. } => railway,
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
