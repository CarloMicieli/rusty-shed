use serde::{Deserialize, Serialize};

/// Summary counters for a `Collection` domain object.
#[derive(Debug, Clone, Serialize, Deserialize, Default, specta::Type)]
pub struct CollectionSummary {
    /// Number of locomotives in the collection.
    pub locomotives_count: u16,

    /// Number of passenger cars in the collection.
    pub passenger_cars_count: u16,

    /// Number of freight cars in the collection.
    pub freight_cars_count: u16,

    /// Number of train sets in the collection.
    pub train_sets_count: u16,

    /// Number of railcars in the collection.
    pub railcars_count: u16,

    /// Number of electric multiple units (EMUs) in the collection.
    pub electric_multiple_units_count: u16,
}
