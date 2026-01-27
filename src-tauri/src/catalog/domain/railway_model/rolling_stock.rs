use crate::catalog::domain::railway_company::RailwayCompanyId;
use crate::catalog::domain::railway_model::ServiceLevel;
use crate::catalog::domain::railway_model::category::{
    ElectricMultipleUnitType, FreightCarType, LocomotiveType, PassengerCarType, RailcarType,
    RollingStockCategory,
};
use crate::catalog::domain::railway_model::control::Control;
use crate::catalog::domain::railway_model::dcc_interface::DccInterface;
use crate::catalog::domain::railway_model::length_over_buffers::LengthOverBuffers;
use crate::catalog::domain::railway_model::rolling_stock_id::RollingStockId;
use crate::catalog::domain::railway_model::technical_specifications::TechnicalSpecifications;

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
    use pretty_assertions::assert_eq;
    use uuid::Uuid;

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
}
