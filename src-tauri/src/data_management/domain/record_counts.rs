use serde::{Deserialize, Serialize};
use specta::Type;

/// Counts of records by entity type.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct RecordCounts {
    pub manufacturers: u32,
    pub railway_companies: u32,
    pub railway_models: u32,
    pub collection_items: u32,
    pub sellers: u32,
    pub maintenance_cards: u32,
    pub track_products: u32,
    pub track_inventories: u32,
    pub prototypes: u32,
    pub formation_categories: u32,
    pub train_formations: u32,
    pub wishlists: u32,
}

impl RecordCounts {
    /// Create a new empty `RecordCounts`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the total count of all records.
    pub fn total(&self) -> u32 {
        self.manufacturers
            + self.railway_companies
            + self.railway_models
            + self.collection_items
            + self.sellers
            + self.maintenance_cards
            + self.track_products
            + self.track_inventories
            + self.prototypes
            + self.formation_categories
            + self.train_formations
            + self.wishlists
    }

    /// Add counts from another `RecordCounts` instance.
    pub fn add(&mut self, other: &RecordCounts) {
        self.manufacturers += other.manufacturers;
        self.railway_companies += other.railway_companies;
        self.railway_models += other.railway_models;
        self.collection_items += other.collection_items;
        self.sellers += other.sellers;
        self.maintenance_cards += other.maintenance_cards;
        self.track_products += other.track_products;
        self.track_inventories += other.track_inventories;
        self.prototypes += other.prototypes;
        self.formation_categories += other.formation_categories;
        self.train_formations += other.train_formations;
        self.wishlists += other.wishlists;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_counts_total() {
        let counts = RecordCounts {
            manufacturers: 1,
            railway_companies: 2,
            railway_models: 3,
            collection_items: 4,
            sellers: 5,
            maintenance_cards: 6,
            track_products: 7,
            track_inventories: 8,
            prototypes: 0,
            formation_categories: 0,
            train_formations: 0,
            wishlists: 0,
        };
        assert_eq!(counts.total(), 36);
    }

    #[test]
    fn test_record_counts_add() {
        let mut counts1 = RecordCounts {
            manufacturers: 1,
            ..Default::default()
        };
        let counts2 = RecordCounts {
            manufacturers: 2,
            railway_companies: 3,
            ..Default::default()
        };
        counts1.add(&counts2);
        assert_eq!(counts1.manufacturers, 3);
        assert_eq!(counts1.railway_companies, 3);
    }
}
