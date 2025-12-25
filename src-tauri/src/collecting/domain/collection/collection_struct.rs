use crate::collecting::domain::collection::CollectionItem;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub id: String,
    pub name: String,

    // Summary counts
    pub locomotives_count: i32,
    pub passenger_cars_count: i32,
    pub freight_cars_count: i32,
    pub train_sets_count: i32,
    pub railcars_count: i32,
    pub electric_multiple_units_count: i32,

    // Total Value
    pub total_value_amount: i64, // in cents
    pub total_value_currency: String,

    pub items: Vec<CollectionItem>,
}
