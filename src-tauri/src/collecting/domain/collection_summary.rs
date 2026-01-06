use crate::catalog::domain::railway_model::Category;
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

    pub starter_sets_count: u16,

    /// The number of self-propelled, multi-unit electric passenger formations.
    pub electric_multiple_units_count: u16,
}

impl CollectionSummary {
    /// Updates the count for a specific category of rolling stock.
    pub fn update_count(&mut self, category: Category, count: u16) {
        match category {
            Category::Locomotives => self.locomotives_count += count,
            Category::PassengerCars => self.passenger_cars_count += count,
            Category::FreightCars => self.freight_cars_count += count,
            Category::TrainSets => self.train_sets_count += count,
            Category::Railcars => self.railcars_count += count,
            Category::ElectricMultipleUnits => self.electric_multiple_units_count += count,
            Category::StarterSets => self.starter_sets_count += count,
        }
    }

    /// Calculates the total number of rolling stock items in the collection.
    ///
    /// This sums up all categories of rolling stock to provide an overall count.
    pub fn total_items(&self) -> u16 {
        self.locomotives_count
            + self.passenger_cars_count
            + self.freight_cars_count
            + self.train_sets_count
            + self.railcars_count
            + self.electric_multiple_units_count
            + self.starter_sets_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_count() {
        let mut summary = CollectionSummary::default();
        summary.update_count(Category::Locomotives, 5);
        summary.update_count(Category::PassengerCars, 10);
        assert_eq!(summary.locomotives_count, 5);
        assert_eq!(summary.passenger_cars_count, 10);
    }

    #[test]
    fn test_total_items() {
        let mut summary = CollectionSummary::default();
        summary.update_count(Category::Locomotives, 3);
        summary.update_count(Category::FreightCars, 7);
        summary.update_count(Category::TrainSets, 2);
        assert_eq!(summary.total_items(), 12);
    }
}
