use crate::export::domain::entity_selection::ExportEntitySelection;
/// Export session aggregate
/// Manages the state and lifecycle of an export operation
use serde::{Deserialize, Serialize};

/// Represents the state of an export session
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExportSessionState {
    Selecting,
    Previewing,
    Exporting,
    Completed,
}

/// Represents an export session aggregate
#[derive(Debug, Clone, Serialize, Deserialize)]
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
