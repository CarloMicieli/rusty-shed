//! Rolling-stock specification variants for the `Prototype` catalog entity.
//!
//! A [`Specification`] captures the discriminated set of technical attributes
//! that differ across rolling-stock categories. The variants map one-to-one
//! with the five categories recognised by the catalog domain.

use crate::catalog::domain::railway_model::{
    ElectricMultipleUnitType, FreightCarType, LocomotiveType, PassengerCarType, RailcarType,
    ServiceLevel,
};
use serde::{Deserialize, Serialize};

/// Discriminated specification for a [`super::prototype::Prototype`].
///
/// Each variant carries the type-specific attributes for that rolling-stock
/// category. The outer `Prototype` holds the fields that are common across all
/// categories (series code, traction flags, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(tag = "kind", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Specification {
    /// An independent traction unit (steam, diesel, or electric).
    Locomotive(LocomotiveSpecs),
    /// A hauled vehicle designed for passenger transport.
    PassengerCar(PassengerCarSpecs),
    /// A hauled vehicle designed for freight transport.
    FreightCar(FreightCarSpecs),
    /// A lightweight self-propelled passenger vehicle.
    Railcar(RailcarSpecs),
    /// A self-propelled multi-unit electric train.
    ElectricMultipleUnit(ElectricMultipleUnitSpecs),
}

/// Technical attributes for a locomotive prototype.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct LocomotiveSpecs {
    /// Primary motive power (steam, diesel, or electric).
    pub locomotive_type: LocomotiveType,
    /// Optional production-series label (e.g. `"I Serie"`, `"prototype"`).
    pub series: Option<String>,
}

/// Technical attributes for a passenger-car prototype.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct PassengerCarSpecs {
    /// Service class(es) provided by this car.
    pub service_level: ServiceLevel,
    /// Interior layout / function of the car.
    pub passenger_car_type: PassengerCarType,
}

/// Technical attributes for a freight-car prototype.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct FreightCarSpecs {
    /// Physical design and cargo category.
    pub freight_car_type: FreightCarType,
}

/// Technical attributes for a railcar prototype.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct RailcarSpecs {
    /// Whether this car carries traction equipment or is an unpowered trailer.
    pub railcar_type: RailcarType,
}

/// Technical attributes for an Electric Multiple Unit prototype.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct ElectricMultipleUnitSpecs {
    /// Function of this car within the EMU set.
    pub electric_multiple_unit_type: ElectricMultipleUnitType,
    /// Number of individual vehicles in the set (e.g. `3` for a 3-car EMU).
    pub elements_count: u8,
    /// `true` when the unit cannot be split into individual cars.
    pub is_permanently_coupled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_roundtrip_locomotive_spec() {
        let spec = Specification::Locomotive(LocomotiveSpecs {
            locomotive_type: LocomotiveType::ElectricLocomotive,
            series: Some("I Serie".to_string()),
        });
        let json = serde_json::to_string(&spec).expect("serialize");
        let de: Specification = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(spec, de);
    }

    #[test]
    fn it_should_roundtrip_passenger_car_spec() {
        let spec = Specification::PassengerCar(PassengerCarSpecs {
            service_level: ServiceLevel::First,
            passenger_car_type: PassengerCarType::CompartmentCoach,
        });
        let json = serde_json::to_string(&spec).expect("serialize");
        let de: Specification = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(spec, de);
    }

    #[test]
    fn it_should_roundtrip_emu_spec() {
        let spec = Specification::ElectricMultipleUnit(ElectricMultipleUnitSpecs {
            electric_multiple_unit_type: ElectricMultipleUnitType::HighSpeedTrain,
            elements_count: 4,
            is_permanently_coupled: true,
        });
        let json = serde_json::to_string(&spec).expect("serialize");
        let de: Specification = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(spec, de);
    }
}
