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

/// Export-specific errors
#[derive(thiserror::Error, Debug)]
pub enum ExportError {
    #[error("No data to export")]
    NoDataToExport,

    #[error("Insufficient disk space")]
    DiskSpaceError,

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Archive creation failed: {0}")]
    ArchiveError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("ZIP error: {0}")]
    ZipError(String),
}

impl From<zip::result::ZipError> for ExportError {
    fn from(err: zip::result::ZipError) -> Self {
        ExportError::ZipError(err.to_string())
    }
}

/// Represents the state of an export session
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ExportSessionState {
    Selecting,
    Previewing,
    Exporting,
    Completed,
}

/// Represents an export session aggregate
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExportSession {
    pub state: ExportSessionState,
    pub entity_selection: Option<ExportEntitySelection>,
    pub destination_path: Option<String>,
    pub estimated_size: Option<u64>,
}

impl Default for ExportSession {
    fn default() -> Self {
        ExportSession {
            state: ExportSessionState::Selecting,
            entity_selection: None,
            destination_path: None,
            estimated_size: None,
        }
    }
}

impl ExportSession {
    /// Create a new export session
    pub fn new() -> Self {
        Self::default()
    }

    /// Transition to previewing state
    pub fn to_previewing(&mut self, selection: ExportEntitySelection) {
        self.entity_selection = Some(selection);
        self.state = ExportSessionState::Previewing;
    }

    /// Transition to exporting state
    pub fn to_exporting(&mut self, destination_path: String, estimated_size: u64) {
        self.destination_path = Some(destination_path);
        self.estimated_size = Some(estimated_size);
        self.state = ExportSessionState::Exporting;
    }

    /// Transition to completed state
    pub fn to_completed(&mut self) {
        self.state = ExportSessionState::Completed;
    }
}

/// Configuration for an export operation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExportConfig {
    /// Destination path for the archive
    pub destination_path: String,
    /// Custom filename (optional)
    pub custom_filename: Option<String>,
    /// Include orphaned images in export
    pub include_orphaned_images: bool,
}

impl ExportConfig {
    /// Create a new export configuration
    pub fn new(destination_path: String) -> Self {
        ExportConfig {
            destination_path,
            custom_filename: None,
            include_orphaned_images: false,
        }
    }

    /// Set custom filename
    pub fn with_filename(mut self, filename: String) -> Self {
        self.custom_filename = Some(filename);
        self
    }

    /// Set whether to include orphaned images
    pub fn with_orphaned_images(mut self, include: bool) -> Self {
        self.include_orphaned_images = include;
        self
    }
}

/// Represents a phase in the export process
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ExportPhase {
    Collecting,
    Compressing,
    Finalizing,
}

/// Tracks progress of an export operation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExportProgress {
    /// Current phase
    pub phase: ExportPhase,
    /// Progress percentage (0-100)
    pub percentage: u32,
    /// Current item being processed
    pub current_item: Option<String>,
    /// Estimated seconds remaining
    pub estimated_seconds_remaining: u32,
}

impl ExportProgress {
    /// Create a new export progress tracker
    pub fn new(phase: ExportPhase) -> Self {
        ExportProgress {
            phase,
            percentage: 0,
            current_item: None,
            estimated_seconds_remaining: 0,
        }
    }

    /// Update progress
    pub fn update(&mut self, percentage: u32, current_item: Option<String>, eta_seconds: u32) {
        self.percentage = percentage.min(100);
        self.current_item = current_item;
        self.estimated_seconds_remaining = eta_seconds;
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
