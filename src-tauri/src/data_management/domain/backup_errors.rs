use crate::core::infrastructure::error::CommandError;

/// Errors that can occur during database backup and restore operations.
#[derive(Debug, thiserror::Error)]
pub enum DatabaseBackupError {
    /// The provided file path is invalid or the directory does not exist.
    #[error("Invalid file path: {0}")]
    InvalidPath(String),

    /// The file is not a valid SQLite database.
    #[error("Invalid SQLite database: {0}")]
    InvalidDatabase(String),

    /// The database schema is not compatible with the current application schema.
    #[error("Incompatible database schema: {0}")]
    IncompatibleSchema(String),

    /// The user-provided confirmation string did not match the required value.
    #[error("Confirmation failed: {0}")]
    ConfirmationFailed(String),

    /// A SQLite or sqlx database error occurred.
    #[error("Database error: {0}")]
    DatabaseError(String),

    /// A file system I/O error occurred.
    #[error("File system error: {0}")]
    FileSystemError(String),

    /// The process does not have permission to access the file or directory.
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// A backup or restore operation is already in progress.
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
