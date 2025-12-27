use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// The enumeration of the railway model categories.
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, EnumString, Display, Serialize, Deserialize, specta::Type,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Category {
    /// Independent traction units powered by steam, diesel, or electricity
    /// used to pull unpowered vehicles.
    Locomotives,

    /// Pre-configured groups of permanently or semi-permanently coupled
    /// vehicles, such as high-speed trains.
    TrainSets,

    /// All-in-one entry-level packages typically including a train,
    /// track, and a power controller.
    StarterSets,

    /// Vehicles designed for the transport of physical goods,
    /// raw materials, or equipment.
    FreightCars,

    /// Vehicles designed for the transport of people, typically
    /// including seating, lighting, and climate control.
    PassengerCars,

    /// Self-propelled train sets consisting of multiple carriages
    /// using electricity as their motive power.
    ElectricMultipleUnits,

    /// Lightweight, self-propelled vehicles (usually a single unit)
    /// designed for passenger service on branch lines.
    Railcars,
}

/// High-level classification for different types of railway rolling stock.
///
/// This categorization distinguishes between traction units, hauled vehicles,
/// and self-propelled passenger units.
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, EnumString, Display, Serialize, Deserialize, specta::Type,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RollingStockCategory {
    /// Independent traction units used to haul unpowered vehicles.
    /// Includes steam, diesel, and electric motive power.
    Locomotive,

    /// Vehicles designed specifically for the transport of goods,
    /// raw materials, or heavy equipment.
    FreightCar,

    /// Vehicles designed for passenger transport, usually featuring
    /// interior seating and climate control.
    PassengerCar,

    /// Self-propelled, multi-unit passenger trains that use
    /// electricity as their motive power (EMUs).
    ElectricMultipleUnit,

    /// Lightweight, self-propelled single vehicles designed for
    /// lower-capacity passenger service on branch lines.
    Railcar,
}

/// Represents the various types of freight rolling stock used in rail transport.
///
/// These classifications are based on the physical design and the specific
/// cargo requirements, such as climate control, weather protection, or
/// specialized loading mechanisms.
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, EnumString, Display, Serialize, Deserialize, specta::Type,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FreightCarType {
    /// Specialized wagons for transporting motor vehicles, often multi-deck.
    AutoTransportCars,

    /// A wagon equipped with a handbrake or a cabin for a brakeman,
    /// historically used to assist in braking the train.
    BrakeWagon,

    /// Flat or skeleton-framed wagons designed specifically to carry
    /// standardized shipping containers (ISO containers).
    ContainerCars,

    /// Fully enclosed wagons used for goods that must be protected
    /// from weather and theft.
    CoveredFreightCars,

    /// Wagons with a lowered center section designed to carry tall
    /// loads like intermodal trailers or containers within height clearances.
    DeepWellFlatCars,

    /// Open-top wagons with a mechanism to tilt the body to unload
    /// bulk materials like sand or gravel.
    DumpCars,

    /// Open-topped rail vehicles used for transporting loose bulk
    /// materials such as coal, ore, or scrap metal.
    Gondola,

    /// Heavy-duty flat wagons designed for extremely heavy or
    /// oversized loads, often featuring many axles.
    HeavyGoodsWagons,

    /// Wagons with a roof that is hinged on one side, allowing
    /// for top-loading of weather-sensitive bulk goods.
    HingedCoverWagons,

    /// Wagons with a floor that slopes toward one or more discharge
    /// doors, used for the rapid unloading of bulk materials.
    HopperWagon,

    /// Insulated wagons equipped with cooling systems for
    /// transporting perishable goods.
    RefrigeratorCars,

    /// Specialized wagons for transporting pressurized or
    /// non-pressurized powders and granulated materials in silos.
    SiloContainerCars,

    /// Wagons with a flexible tarpaulin cover that slides open
    /// for easy side-loading of palletized goods.
    SlideTarpaulinWagon,

    /// Boxcars with large sliding doors that make up the entire
    /// side of the wagon, allowing for forklift access.
    SlidingWallBoxcars,

    /// Wagons designed for niche cargo that does not fit
    /// into standard classifications.
    SpecialTransport,

    /// Flat wagons equipped with vertical posts (stakes) along
    /// the sides to secure long loads like timber or pipes.
    StakeWagons,

    /// Wagons with a roof that swings to the side to provide
    /// a wide opening for top-loading bulk cargo.
    SwingRoofWagon,

    /// Enclosed pressurized or non-pressurized vessels for
    /// transporting liquids, gases, or chemicals.
    TankCars,

    /// Wagons with several overlapping "hoods" that slide
    /// over each other, used for protecting steel coils or heavy machinery.
    TelescopeHoodWagons,
}

/// Specifies the primary motive power source for a locomotive.
///
/// This classification determines the operational requirements, such as
/// fueling infrastructure or overhead electrification.
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, EnumString, Display, Serialize, Deserialize, specta::Type,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LocomotiveType {
    /// Locomotives powered by an external combustion engine, typically
    /// using a boiler to produce steam from coal, wood, or oil.
    SteamLocomotive,

    /// Locomotives powered by an internal combustion engine, usually
    /// driving an electric generator or a hydraulic transmission.
    DieselLocomotive,

    /// Locomotives that draw power from external sources, such as
    /// overhead catenary wires or a third rail.
    ElectricLocomotive,
}

/// The types for passenger car rolling stocks
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, EnumString, Display, Serialize, Deserialize, specta::Type,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PassengerCarType {
    /// A car usually placed between the locomotive and the rest of the train,
    /// featuring a wide-open interior for carrying checked baggage.
    BaggageCar,

    /// A car providing limited food service (snacks/drinks) and a counter,
    /// smaller in scale than a full Dining Car.
    BuffetCar,

    /// A hybrid car containing separate sections for both passengers and
    /// freight or baggage.
    CombineCar,

    /// A coach featuring a side corridor connecting individual private
    /// compartments, each with face-to-face seating rows.
    CompartmentCoach,

    /// A car dedicated to full-service meal preparation and seating for passengers.
    DiningCar,

    /// A car with two levels of passenger seating to increase capacity
    /// without increasing train length.
    DoubleDecker,

    /// A car with a glass-roofed section raised above the normal roofline,
    /// allowing 360-degree views of the scenery.
    DomeCar,

    /// A control car equipped with a driver's cab, allowing the locomotive
    /// to be operated from the opposite end in a push-pull configuration.
    DrivingTrailer,

    /// A car featuring a bar and informal public seating, often used
    /// as a social space.
    Lounge,

    /// The final car of a train, often featuring large windows or an open
    /// rear platform for scenic viewing.
    Observation,

    /// A coach with a central aisle and rows of seats similar to an
    /// airliner's cabin layout.
    OpenCoach,

    /// A specialized car used for sorting mail while in transit to
    /// expedite delivery.
    RailwayPostOffice,

    /// A car with berths or private rooms for overnight travel.
    /// Also known as a "Sleeper" or "Pullman car."
    SleepingCar,

    /// A "couchette" or "Sleeperette" car, providing reclining seats or
    /// basic fold-down bunks for a more economical overnight option.
    Sleeperette,
}

/// The cars that form a complete EMU set can usually be separated by function into four types:
/// power car, motor car, driving car, and trailer car.
///
/// Each car can have more than one function, such as a motor-driving car or power-driving car.
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, EnumString, Display, Serialize, Deserialize, specta::Type,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ElectricMultipleUnitType {
    /// Driving cars are similar to a cab car, containing a driver's cab for controlling the train.
    /// An EMU will usually have two driving cars at its outer ends.
    DrivingCar,

    /// High-speed rail is a type of rail system that runs significantly faster than traditional
    /// rail, using an integrated system of specialised rolling stock and dedicated tracks.
    HighSpeedTrain,

    /// Motor cars carry the traction motors to move the train, and are often combined with the
    /// power car to avoid high-voltage inter-car connections.
    MotorCar,

    /// A power car carries the necessary equipment to draw power from the electrified
    /// infrastructure, such as pickup shoes for third rail systems and pantographs for
    /// overhead systems, and transformers.
    PowerCar,

    /// Trailer cars are any cars (sometimes semi-permanently coupled) that carry little or no
    /// traction or power related equipment, and are similar to passenger cars in a
    /// locomotive-hauled train.
    TrailerCar,

    /// A trainset is working as whole unit
    TrainSet,
}

/// The types for railcar rolling stocks
///
/// A railcar is a self-propelled railway vehicle designed to transport passengers.
/// The term _"railcar"_ is usually used in reference to a train consisting of a single coach
/// (carriage, car), with a driver's cab at one or both ends.
///
/// In its simplest form, a "railcar" may also be little more than a motorized railway handcar
/// or draisine.
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, EnumString, Display, Serialize, Deserialize, specta::Type,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RailcarType {
    /// A self-propelled passenger vehicles also capable of hauling a train.
    PowerCar,

    /// Trailer cars are any cars (sometimes semi-permanently coupled) that carry little or no
    /// traction or power related equipment, and are similar to passenger cars in a
    /// locomotive-hauled train.
    TrailerCar,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use strum::ParseError;

    mod category_tests {
        use super::*;
        use pretty_assertions::assert_eq;

        #[rstest]
        #[case("LOCOMOTIVES", Ok(Category::Locomotives))]
        #[case("TRAIN_SETS", Ok(Category::TrainSets))]
        #[case("STARTER_SETS", Ok(Category::StarterSets))]
        #[case("FREIGHT_CARS", Ok(Category::FreightCars))]
        #[case("PASSENGER_CARS", Ok(Category::PassengerCars))]
        #[case("ELECTRIC_MULTIPLE_UNITS", Ok(Category::ElectricMultipleUnits))]
        #[case("RAILCARS", Ok(Category::Railcars))]
        fn parse_category(#[case] input: &str, #[case] expected: Result<Category, ParseError>) {
            let result = input.parse::<Category>();
            assert_eq!(expected, result);
        }

        #[test]
        fn parse_category_lowercase() {
            let result = "locomotives".parse::<Category>();
            assert_eq!(Ok(Category::Locomotives), result);
        }

        #[rstest]
        #[case(Category::Locomotives, "LOCOMOTIVES")]
        #[case(Category::TrainSets, "TRAIN_SETS")]
        #[case(Category::StarterSets, "STARTER_SETS")]
        #[case(Category::FreightCars, "FREIGHT_CARS")]
        #[case(Category::PassengerCars, "PASSENGER_CARS")]
        #[case(Category::ElectricMultipleUnits, "ELECTRIC_MULTIPLE_UNITS")]
        #[case(Category::Railcars, "RAILCARS")]
        fn display_category(#[case] input: Category, #[case] expected: &str) {
            assert_eq!(expected, input.to_string());
        }

        #[test]
        fn parse_category_invalid() {
            let result = "NOT_A_VALID_CATEGORY".parse::<Category>();
            assert_eq!(Err(ParseError::VariantNotFound), result);
        }
    }

    mod rolling_stock_category_tests {
        use super::*;
        use pretty_assertions::assert_eq;

        #[rstest]
        #[case("LOCOMOTIVE", Ok(RollingStockCategory::Locomotive))]
        #[case("FREIGHT_CAR", Ok(RollingStockCategory::FreightCar))]
        #[case("PASSENGER_CAR", Ok(RollingStockCategory::PassengerCar))]
        #[case(
            "ELECTRIC_MULTIPLE_UNIT",
            Ok(RollingStockCategory::ElectricMultipleUnit)
        )]
        #[case("RAILCAR", Ok(RollingStockCategory::Railcar))]
        fn parse_rolling_stock_category(
            #[case] input: &str,
            #[case] expected: Result<RollingStockCategory, ParseError>,
        ) {
            let result = input.parse::<RollingStockCategory>();
            assert_eq!(expected, result);
        }

        #[test]
        fn parse_rolling_stock_category_lowercase() {
            let result = "locomotive".parse::<RollingStockCategory>();
            assert_eq!(Ok(RollingStockCategory::Locomotive), result);
        }

        #[rstest]
        #[case(RollingStockCategory::Locomotive, "LOCOMOTIVE")]
        #[case(RollingStockCategory::FreightCar, "FREIGHT_CAR")]
        #[case(RollingStockCategory::PassengerCar, "PASSENGER_CAR")]
        #[case(RollingStockCategory::ElectricMultipleUnit, "ELECTRIC_MULTIPLE_UNIT")]
        #[case(RollingStockCategory::Railcar, "RAILCAR")]
        fn display_rolling_stock_category(
            #[case] input: RollingStockCategory,
            #[case] expected: &str,
        ) {
            assert_eq!(expected, input.to_string());
        }

        #[test]
        fn parse_rolling_stock_category_invalid() {
            let result = "NO_SUCH_ROLLING_STOCK_CATEGORY".parse::<RollingStockCategory>();
            assert_eq!(Err(ParseError::VariantNotFound), result);
        }
    }

    mod freight_car_type_tests {
        use super::*;
        use pretty_assertions::assert_eq;

        #[rstest]
        #[case("AUTO_TRANSPORT_CARS", Ok(FreightCarType::AutoTransportCars))]
        #[case("BRAKE_WAGON", Ok(FreightCarType::BrakeWagon))]
        #[case("CONTAINER_CARS", Ok(FreightCarType::ContainerCars))]
        #[case("COVERED_FREIGHT_CARS", Ok(FreightCarType::CoveredFreightCars))]
        #[case("DEEP_WELL_FLAT_CARS", Ok(FreightCarType::DeepWellFlatCars))]
        #[case("DUMP_CARS", Ok(FreightCarType::DumpCars))]
        #[case("GONDOLA", Ok(FreightCarType::Gondola))]
        #[case("HEAVY_GOODS_WAGONS", Ok(FreightCarType::HeavyGoodsWagons))]
        #[case("HINGED_COVER_WAGONS", Ok(FreightCarType::HingedCoverWagons))]
        #[case("HOPPER_WAGON", Ok(FreightCarType::HopperWagon))]
        #[case("REFRIGERATOR_CARS", Ok(FreightCarType::RefrigeratorCars))]
        #[case("SILO_CONTAINER_CARS", Ok(FreightCarType::SiloContainerCars))]
        #[case("SLIDE_TARPAULIN_WAGON", Ok(FreightCarType::SlideTarpaulinWagon))]
        #[case("SLIDING_WALL_BOXCARS", Ok(FreightCarType::SlidingWallBoxcars))]
        #[case("SPECIAL_TRANSPORT", Ok(FreightCarType::SpecialTransport))]
        #[case("STAKE_WAGONS", Ok(FreightCarType::StakeWagons))]
        #[case("SWING_ROOF_WAGON", Ok(FreightCarType::SwingRoofWagon))]
        #[case("TANK_CARS", Ok(FreightCarType::TankCars))]
        #[case("TELESCOPE_HOOD_WAGONS", Ok(FreightCarType::TelescopeHoodWagons))]
        fn parse_freight_car_type(
            #[case] input: &str,
            #[case] expected: Result<FreightCarType, ParseError>,
        ) {
            let result = input.parse::<FreightCarType>();
            assert_eq!(expected, result);
        }

        #[test]
        fn parse_freight_car_type_lowercase() {
            let result = "gondola".parse::<FreightCarType>();
            assert_eq!(Ok(FreightCarType::Gondola), result);
        }

        #[rstest]
        #[case(FreightCarType::AutoTransportCars, "AUTO_TRANSPORT_CARS")]
        #[case(FreightCarType::BrakeWagon, "BRAKE_WAGON")]
        #[case(FreightCarType::ContainerCars, "CONTAINER_CARS")]
        #[case(FreightCarType::CoveredFreightCars, "COVERED_FREIGHT_CARS")]
        #[case(FreightCarType::DeepWellFlatCars, "DEEP_WELL_FLAT_CARS")]
        #[case(FreightCarType::DumpCars, "DUMP_CARS")]
        #[case(FreightCarType::Gondola, "GONDOLA")]
        #[case(FreightCarType::HeavyGoodsWagons, "HEAVY_GOODS_WAGONS")]
        #[case(FreightCarType::HingedCoverWagons, "HINGED_COVER_WAGONS")]
        #[case(FreightCarType::HopperWagon, "HOPPER_WAGON")]
        #[case(FreightCarType::RefrigeratorCars, "REFRIGERATOR_CARS")]
        #[case(FreightCarType::SiloContainerCars, "SILO_CONTAINER_CARS")]
        #[case(FreightCarType::SlideTarpaulinWagon, "SLIDE_TARPAULIN_WAGON")]
        #[case(FreightCarType::SlidingWallBoxcars, "SLIDING_WALL_BOXCARS")]
        #[case(FreightCarType::SpecialTransport, "SPECIAL_TRANSPORT")]
        #[case(FreightCarType::StakeWagons, "STAKE_WAGONS")]
        #[case(FreightCarType::SwingRoofWagon, "SWING_ROOF_WAGON")]
        #[case(FreightCarType::TankCars, "TANK_CARS")]
        #[case(FreightCarType::TelescopeHoodWagons, "TELESCOPE_HOOD_WAGONS")]
        fn display_freight_car_type(#[case] input: FreightCarType, #[case] expected: &str) {
            assert_eq!(expected, input.to_string());
        }

        #[test]
        fn parse_freight_car_type_invalid() {
            let result = "UNKNOWN_FREIGHT_CAR_TYPE".parse::<FreightCarType>();
            assert_eq!(Err(ParseError::VariantNotFound), result);
        }
    }

    mod locomotive_type_tests {
        use super::*;
        use pretty_assertions::assert_eq;

        #[rstest]
        #[case("STEAM_LOCOMOTIVE", Ok(LocomotiveType::SteamLocomotive))]
        #[case("DIESEL_LOCOMOTIVE", Ok(LocomotiveType::DieselLocomotive))]
        #[case("ELECTRIC_LOCOMOTIVE", Ok(LocomotiveType::ElectricLocomotive))]
        fn parse_locomotive_type(
            #[case] input: &str,
            #[case] expected: Result<LocomotiveType, ParseError>,
        ) {
            let result = input.parse::<LocomotiveType>();
            assert_eq!(expected, result);
        }

        #[test]
        fn parse_locomotive_type_lowercase() {
            let result = "steam_locomotive".parse::<LocomotiveType>();
            assert_eq!(Ok(LocomotiveType::SteamLocomotive), result);
        }

        #[rstest]
        #[case(LocomotiveType::SteamLocomotive, "STEAM_LOCOMOTIVE")]
        #[case(LocomotiveType::DieselLocomotive, "DIESEL_LOCOMOTIVE")]
        #[case(LocomotiveType::ElectricLocomotive, "ELECTRIC_LOCOMOTIVE")]
        fn display_locomotive_type(#[case] input: LocomotiveType, #[case] expected: &str) {
            assert_eq!(expected, input.to_string());
        }

        #[test]
        fn parse_locomotive_type_invalid() {
            let result = "NOT_A_LOCOMOTIVE_TYPE".parse::<LocomotiveType>();
            assert_eq!(Err(ParseError::VariantNotFound), result);
        }
    }

    mod passenger_car_type_tests {
        use super::*;
        use pretty_assertions::assert_eq;

        #[rstest]
        #[case("BAGGAGE_CAR", Ok(PassengerCarType::BaggageCar))]
        #[case("BUFFET_CAR", Ok(PassengerCarType::BuffetCar))]
        #[case("COMBINE_CAR", Ok(PassengerCarType::CombineCar))]
        #[case("COMPARTMENT_COACH", Ok(PassengerCarType::CompartmentCoach))]
        #[case("DINING_CAR", Ok(PassengerCarType::DiningCar))]
        #[case("DOME_CAR", Ok(PassengerCarType::DomeCar))]
        #[case("DOUBLE_DECKER", Ok(PassengerCarType::DoubleDecker))]
        #[case("DRIVING_TRAILER", Ok(PassengerCarType::DrivingTrailer))]
        #[case("LOUNGE", Ok(PassengerCarType::Lounge))]
        #[case("OBSERVATION", Ok(PassengerCarType::Observation))]
        #[case("OPEN_COACH", Ok(PassengerCarType::OpenCoach))]
        #[case("RAILWAY_POST_OFFICE", Ok(PassengerCarType::RailwayPostOffice))]
        #[case("SLEEPING_CAR", Ok(PassengerCarType::SleepingCar))]
        #[case("SLEEPERETTE", Ok(PassengerCarType::Sleeperette))]
        fn parse_passenger_car_type(
            #[case] input: &str,
            #[case] expected: Result<PassengerCarType, ParseError>,
        ) {
            let result = input.parse::<PassengerCarType>();
            assert_eq!(expected, result);
        }

        #[test]
        fn parse_passenger_car_type_lowercase() {
            let result = "combine_car".parse::<PassengerCarType>();
            assert_eq!(Ok(PassengerCarType::CombineCar), result);
        }

        #[rstest]
        #[case(PassengerCarType::BaggageCar, "BAGGAGE_CAR")]
        #[case(PassengerCarType::CombineCar, "COMBINE_CAR")]
        #[case(PassengerCarType::CompartmentCoach, "COMPARTMENT_COACH")]
        #[case(PassengerCarType::DiningCar, "DINING_CAR")]
        #[case(PassengerCarType::DoubleDecker, "DOUBLE_DECKER")]
        #[case(PassengerCarType::DrivingTrailer, "DRIVING_TRAILER")]
        #[case(PassengerCarType::Lounge, "LOUNGE")]
        #[case(PassengerCarType::Observation, "OBSERVATION")]
        #[case(PassengerCarType::OpenCoach, "OPEN_COACH")]
        #[case(PassengerCarType::RailwayPostOffice, "RAILWAY_POST_OFFICE")]
        #[case(PassengerCarType::SleepingCar, "SLEEPING_CAR")]
        fn display_passenger_car_type(#[case] input: PassengerCarType, #[case] expected: &str) {
            assert_eq!(expected, input.to_string());
        }

        #[test]
        fn parse_passenger_car_type_invalid() {
            let result = "NOT_VALID_PASSENGER_CAR_TYPE".parse::<PassengerCarType>();
            assert_eq!(Err(ParseError::VariantNotFound), result);
        }
    }

    mod emu_type_tests {
        use super::*;
        use pretty_assertions::assert_eq;

        #[rstest]
        #[case("DRIVING_CAR", Ok(ElectricMultipleUnitType::DrivingCar))]
        #[case("HIGH_SPEED_TRAIN", Ok(ElectricMultipleUnitType::HighSpeedTrain))]
        #[case("MOTOR_CAR", Ok(ElectricMultipleUnitType::MotorCar))]
        #[case("POWER_CAR", Ok(ElectricMultipleUnitType::PowerCar))]
        #[case("TRAILER_CAR", Ok(ElectricMultipleUnitType::TrailerCar))]
        #[case("TRAIN_SET", Ok(ElectricMultipleUnitType::TrainSet))]
        fn parse_electric_multiple_unit_type(
            #[case] input: &str,
            #[case] expected: Result<ElectricMultipleUnitType, ParseError>,
        ) {
            let result = input.parse::<ElectricMultipleUnitType>();
            assert_eq!(expected, result);
        }

        #[test]
        fn parse_electric_multiple_unit_type_lowercase() {
            let result = "driving_car".parse::<ElectricMultipleUnitType>();
            assert_eq!(Ok(ElectricMultipleUnitType::DrivingCar), result);
        }

        #[rstest]
        #[case(ElectricMultipleUnitType::DrivingCar, "DRIVING_CAR")]
        #[case(ElectricMultipleUnitType::HighSpeedTrain, "HIGH_SPEED_TRAIN")]
        #[case(ElectricMultipleUnitType::MotorCar, "MOTOR_CAR")]
        #[case(ElectricMultipleUnitType::PowerCar, "POWER_CAR")]
        #[case(ElectricMultipleUnitType::TrailerCar, "TRAILER_CAR")]
        #[case(ElectricMultipleUnitType::TrainSet, "TRAIN_SET")]
        fn display_electric_multiple_unit_type(
            #[case] input: ElectricMultipleUnitType,
            #[case] expected: &str,
        ) {
            assert_eq!(expected, input.to_string());
        }

        #[test]
        fn parse_electric_multiple_unit_type_invalid() {
            let result = "NO_EMU_TYPE".parse::<ElectricMultipleUnitType>();
            assert_eq!(Err(ParseError::VariantNotFound), result);
        }
    }

    mod railcar_type_tests {
        use super::*;
        use pretty_assertions::assert_eq;

        #[rstest]
        #[case("POWER_CAR", Ok(RailcarType::PowerCar))]
        #[case("TRAILER_CAR", Ok(RailcarType::TrailerCar))]
        fn parse_railcar_type(
            #[case] input: &str,
            #[case] expected: Result<RailcarType, ParseError>,
        ) {
            let result = input.parse::<RailcarType>();
            assert_eq!(expected, result);
        }

        #[test]
        fn parse_railcar_type_lowercase() {
            let result = "power_car".parse::<RailcarType>();
            assert_eq!(Ok(RailcarType::PowerCar), result);
        }

        #[rstest]
        #[case(RailcarType::PowerCar, "POWER_CAR")]
        #[case(RailcarType::TrailerCar, "TRAILER_CAR")]
        fn display_railcar_type(#[case] input: RailcarType, #[case] expected: &str) {
            assert_eq!(expected, input.to_string());
        }

        #[test]
        fn parse_railcar_type_invalid() {
            let result = "UNKNOWN_RAILCAR_TYPE".parse::<RailcarType>();
            assert_eq!(Err(ParseError::VariantNotFound), result);
        }
    }
}
