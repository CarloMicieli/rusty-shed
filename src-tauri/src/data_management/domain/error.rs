/// Internal error type for data management operations.
/// Never returned directly to Tauri — handlers map to CommandError.
#[derive(Debug, thiserror::Error)]
pub enum DataManagementError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Archive error: {0}")]
    ArchiveError(String),
    #[error("Schema violation: {0}")]
    SchemaViolation(String),
    #[error("IO error: {0}")]
    IoError(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}
