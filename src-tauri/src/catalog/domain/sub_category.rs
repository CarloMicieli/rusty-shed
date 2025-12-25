use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubCategory {
    ClosedCargoVehicle,
    DieselLocomotive,
    DiningCar,
    ElectricLocomotive,
    PowerCars,
    Railcars,
    RailwayPostOffice,
    RefrigeratorCars,
    SteamLocomotive,
    TrailerCar,
}
