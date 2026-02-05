use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use specta::Type;
use std::path::PathBuf;
use uuid::Uuid;

/// Represents the state of an import session.
#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ImportState {
    /// Initial state after file selection
    Pending,
    /// Manifest extracted and parsed
    Analyzed,
    /// Schema validation complete
    Validated,
    /// Preview generated with duplicate detection
    Previewed,
    /// Import in progress
    Importing,
    /// Import completed successfully
    Completed,
    /// Import failed or aborted
    Failed { reason: String },
}

/// Archive format detection.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ArchiveFormat {
    Zip,
    TarGz,
}

/// An active import session.
#[derive(Debug, Clone)]
pub struct ImportSession {
    /// Unique session identifier
    pub id: String,
    /// Path to the original archive file
    pub source_path: PathBuf,
    /// Archive format detected
    pub format: ArchiveFormat,
    /// Current state of the session
    pub state: ImportState,
    /// Session timestamps
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl ImportSession {
    /// Create a new import session.
    pub fn new(source_path: PathBuf, format: ArchiveFormat) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            source_path,
            format,
            state: ImportState::Pending,
            created_at: Utc::now(),
            completed_at: None,
        }
    }

    /// Transition to the next state.
    pub fn transition(&mut self, new_state: ImportState) {
        self.state = new_state;
        if matches!(
            self.state,
            ImportState::Completed | ImportState::Failed { .. }
        ) {
            self.completed_at = Some(Utc::now());
        }
    }

    /// Check if the session is in a terminal state.
    pub fn is_complete(&self) -> bool {
        matches!(
            self.state,
            ImportState::Completed | ImportState::Failed { .. }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_session_creation() {
        let path = PathBuf::from("/tmp/test.zip");
        let session = ImportSession::new(path.clone(), ArchiveFormat::Zip);
        assert_eq!(session.source_path, path);
        assert_eq!(session.format, ArchiveFormat::Zip);
        assert_eq!(session.state, ImportState::Pending);
        assert!(session.completed_at.is_none());
    }

    #[test]
    fn test_import_session_transition() {
        let mut session = ImportSession::new(PathBuf::from("/tmp/test.zip"), ArchiveFormat::Zip);
        session.transition(ImportState::Analyzed);
        assert_eq!(session.state, ImportState::Analyzed);
        assert!(session.completed_at.is_none());
    }

    #[test]
    fn test_import_session_completion() {
        let mut session = ImportSession::new(PathBuf::from("/tmp/test.zip"), ArchiveFormat::Zip);
        session.transition(ImportState::Completed);
        assert_eq!(session.state, ImportState::Completed);
        assert!(session.completed_at.is_some());
        assert!(session.is_complete());
    }

    #[test]
    fn test_import_session_failure() {
        let mut session = ImportSession::new(PathBuf::from("/tmp/test.zip"), ArchiveFormat::Zip);
        session.transition(ImportState::Failed {
            reason: "Test error".to_string(),
        });
        assert!(session.is_complete());
    }
}
