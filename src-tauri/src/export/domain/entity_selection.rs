/// Entity selection for export.
///
/// Specifies which entity types to include in the export
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExportEntitySelection {
    /// Include railway models
    pub include_railway_models: bool,
    /// Include collection items
    pub include_collection_items: bool,
    /// Include sellers
    pub include_sellers: bool,
    /// Include maintenance logs
    pub include_maintenance_logs: bool,
    /// Include DCC roster
    pub include_dcc_roster: bool,
    /// Include orphaned images
    pub include_orphaned_images: bool,
    /// Include track inventory (products, inventories, purchases)
    pub include_track_inventory: bool,
}

impl ExportEntitySelection {
    /// Check if at least one entity type is selected
    pub fn is_valid(&self) -> bool {
        self.include_railway_models
            || self.include_collection_items
            || self.include_sellers
            || self.include_maintenance_logs
            || self.include_dcc_roster
            || self.include_track_inventory
    }

    /// Get count of entity types selected
    pub fn get_entity_count(&self) -> u32 {
        let mut count = 0;
        if self.include_railway_models {
            count += 1;
        }
        if self.include_collection_items {
            count += 1;
        }
        if self.include_sellers {
            count += 1;
        }
        if self.include_maintenance_logs {
            count += 1;
        }
        if self.include_dcc_roster {
            count += 1;
        }
        if self.include_track_inventory {
            count += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_empty() {
        let selection = ExportEntitySelection {
            include_railway_models: false,
            include_collection_items: false,
            include_sellers: false,
            include_maintenance_logs: false,
            include_dcc_roster: false,
            include_orphaned_images: false,
            include_track_inventory: false,
        };
        assert!(!selection.is_valid());
    }

    #[test]
    fn test_validation_with_selection() {
        let selection = ExportEntitySelection {
            include_railway_models: true,
            include_collection_items: false,
            include_sellers: false,
            include_maintenance_logs: false,
            include_dcc_roster: false,
            include_orphaned_images: false,
            include_track_inventory: false,
        };
        assert!(selection.is_valid());
    }
}
