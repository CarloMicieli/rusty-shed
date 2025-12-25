use serde::{Deserialize, Serialize};

/// The enumeration of the model categories.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Category {
    /// The steam locomotives category
    Locomotives,

    /// The train sets category
    TrainSets,

    /// The train sets category
    StarterSets,

    /// The freight cars category
    FreightCars,

    /// The passenger cars category
    PassengerCars,

    /// The electric multiple units category
    ElectricMultipleUnits,

    /// The railcars category
    Railcars,
}

/// The enumeration of the rolling stock categories.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RollingStockCategory {
    /// The steam locomotives category
    Locomotive,

    /// The freight cars category
    FreightCar,

    /// The passenger cars category
    PassengerCar,

    /// The electric multiple units category
    ElectricMultipleUnit,

    /// The railcars category
    Railcar,
}

/// The different kind of freight cars
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FreightCarType {
    AutoTransportCars,
    BrakeWagon,
    ContainerCars,
    CoveredFreightCars,
    DeepWellFlatCars,
    DumpCars,
    Gondola,
    HeavyGoodsWagons,
    HingedCoverWagons,
    HopperWagon,
    RefrigeratorCars,
    SiloContainerCars,
    SlideTarpaulinWagon,
    SlidingWallBoxcars,
    SpecialTransport,
    StakeWagons,
    SwingRoofWagon,
    TankCars,
    TelescopeHoodWagons,
}

/// The different kinds of locomotives
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocomotiveType {
    /// The steam locomotives category
    SteamLocomotive,

    /// The diesel locomotives category
    DieselLocomotive,

    /// The electric locomotives category
    ElectricLocomotive,
}

/// The types for passenger car rolling stocks
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PassengerCarType {
    /// The baggage car is a car that was normally placed between the train's motive power and the
    /// remainder of the passenger train. The car's interior is normally wide open and is used to
    /// carry passengers' checked baggage.
    BaggageCar,

    /// A combine car is a type of railroad car which combines sections for both passengers and freight
    CombineCar,

    /// "closed" coaches or "compartment" cars have a side corridor to connect individual compartments
    /// along the body of the train, each with two rows of seats facing each other.
    CompartmentCoach,

    /// A dining car (or diner) is used to serve meals to the passengers.
    DiningCar,

    /// A double-decker coach, or bilevel car, is a type of rail car that has two levels of passenger
    /// accommodation, as opposed to one, increasing passenger capacity
    DoubleDecker,

    /// A driving trailer is a purpose-built control car railway vehicle that allows the driver
    /// to operate with a locomotive in push-pull formation from the opposite end of a train
    DrivingTrailer,

    /// Lounge cars carry a bar and public seating.
    Lounge,

    /// The observation car almost always operated as the last car in a passenger train, in US
    /// practice. Its interior could include features of a coach, lounge, diner, or sleeper. The
    /// main spotting feature was at the tail end of the car.
    Observation,

    /// An "open coach" has a central aisle; the car's interior is often filled with row upon row of
    /// seats as in a passenger airliner.
    OpenCoach,

    /// A railway post office is a railroad car that was normally operated in passenger service
    /// as a means to sort mail en route, in order to speed delivery.
    RailwayPostOffice,

    ///Often called "sleepers" or "Pullman cars", these cars provide sleeping arrangements for
    ///passengers travelling at night. Early models were divided into sections, where coach
    /// seating converted at night into semi-private berths.
    SleepingCar,
}

/// The cars that form a complete EMU set can usually be separated by function into four types:
/// power car, motor car, driving car, and trailer car.
///
/// Each car can have more than one function, such as a motor-driving car or power-driving car.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
/// # Description
///
/// A railcar is a self-propelled railway vehicle designed to transport passengers.
/// The term _"railcar"_ is usually used in reference to a train consisting of a single coach
/// (carriage, car), with a driver's cab at one or both ends.
///
/// In its simplest form, a "railcar" may also be little more than a motorized railway handcar
/// or draisine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RailcarType {
    /// A self-propelled passenger vehicles also capable of hauling a train.
    PowerCar,

    /// Trailer cars are any cars (sometimes semi-permanently coupled) that carry little or no
    /// traction or power related equipment, and are similar to passenger cars in a
    /// locomotive-hauled train.
    TrailerCar,
}
