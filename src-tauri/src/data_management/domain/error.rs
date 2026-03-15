/// Internal error type for data management operations.
/// Never returned directly to Tauri — handlers map to CommandError.
#[derive(Debug)]
pub enum DataManagementError {
    DatabaseError(String),
    ArchiveError(String),
    SchemaViolation(String),
    IoError(String),
    NotFound(String),
    InvalidInput(String),
    Unknown(String),
}

impl std::fmt::Display for DataManagementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            Self::ArchiveError(msg) => write!(f, "Archive error: {}", msg),
            Self::SchemaViolation(msg) => write!(f, "Schema violation: {}", msg),
            Self::IoError(msg) => write!(f, "IO error: {}", msg),
            Self::NotFound(msg) => write!(f, "Not found: {}", msg),
            Self::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            Self::Unknown(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl std::error::Error for DataManagementError {}
