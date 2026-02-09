/// Export progress value object
use serde::{Deserialize, Serialize};

/// Represents a phase in the export process
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExportPhase {
    Collecting,
    Compressing,
    Finalizing,
}

/// Tracks progress of an export operation
#[derive(Debug, Clone, Serialize, Deserialize)]
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
