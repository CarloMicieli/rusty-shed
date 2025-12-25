use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Category {
    ElectricMultipleUnit,
    FreightCar,
    Locomotive,
    PassengerCar,
    Train,
    TrainSet,
}
