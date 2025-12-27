use serde::{Deserialize, Serialize};

/// A statistical summary of a model railway collection.
///
/// This struct provides a high-level overview of the total quantities
/// of different types of rolling stock within a specific inventory or sub-collection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize, Serialize, specta::Type)]
pub struct CollectionSummary {
    /// The total number of independent traction units (Steam, Diesel, Electric).
    pub locomotives_count: u16,

    /// The total number of individual passenger-carrying vehicles.
    pub passenger_cars_count: u16,

    /// The total number of individual goods-transporting vehicles.
    pub freight_cars_count: u16,

    /// The number of complete train sets (e.g., starter sets or fixed formations).
    ///
    /// Note: Depending on implementation, the individual cars within these sets
    /// may or may not be included in the other specific counts.
    pub train_sets_count: u16,

    /// The number of self-propelled, typically single-unit passenger vehicles.
    pub railcars_count: u16,

    /// The number of self-propelled, multi-unit electric passenger formations.
    pub electric_multiple_units_count: u16,
}
