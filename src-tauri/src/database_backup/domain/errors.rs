use crate::core::infrastructure::error::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum DatabaseBackupError {
    #[error("Invalid file path: {0}")]
    InvalidPath(String),

    #[error("Invalid SQLite database: {0}")]
    InvalidDatabase(String),

    #[error("Incompatible database schema: {0}")]
    IncompatibleSchema(String),

    #[error("Confirmation failed: {0}")]
    ConfirmationFailed(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("File system error: {0}")]
    FileSystemError(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Operation in progress")]
    OperationInProgress,
}

impl From<DatabaseBackupError> for CommandError {
    fn from(err: DatabaseBackupError) -> Self {
        match err {
            DatabaseBackupError::InvalidPath(msg) => {
                CommandError::validation_field("destination_path", msg)
            }
            DatabaseBackupError::InvalidDatabase(msg) => {
                CommandError::validation_field("source_path", msg)
            }
            DatabaseBackupError::IncompatibleSchema(msg) => {
                CommandError::validation_field("source_path", msg)
            }
            DatabaseBackupError::ConfirmationFailed(msg) => {
                CommandError::validation_field("confirmation", msg)
            }
            DatabaseBackupError::PermissionDenied(msg) => CommandError::PermissionDenied(msg),
            _ => CommandError::unknown(err.to_string()),
        }
    }
}
