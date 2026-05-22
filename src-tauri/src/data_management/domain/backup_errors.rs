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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::infrastructure::error::CommandError;

    #[test]
    fn invalid_path_maps_to_destination_path_validation_error() {
        let msg = "missing folder".to_string();
        let err: CommandError = DatabaseBackupError::InvalidPath(msg.clone()).into();

        match err {
            CommandError::ValidationError(fields) => {
                let errors = fields
                    .get("destination_path")
                    .expect("destination_path field should exist");
                assert_eq!(errors.len(), 1);
                assert_eq!(errors[0].message.as_deref(), Some(msg.as_str()));
            }
            other => panic!("expected ValidationError, got {other:?}"),
        }
    }

    #[test]
    fn invalid_database_maps_to_source_path_validation_error() {
        let msg = "not sqlite".to_string();
        let err: CommandError = DatabaseBackupError::InvalidDatabase(msg.clone()).into();

        match err {
            CommandError::ValidationError(fields) => {
                let errors = fields
                    .get("source_path")
                    .expect("source_path field should exist");
                assert_eq!(errors.len(), 1);
                assert_eq!(errors[0].message.as_deref(), Some(msg.as_str()));
            }
            other => panic!("expected ValidationError, got {other:?}"),
        }
    }

    #[test]
    fn incompatible_schema_maps_to_source_path_validation_error() {
        let msg = "schema mismatch".to_string();
        let err: CommandError = DatabaseBackupError::IncompatibleSchema(msg.clone()).into();

        match err {
            CommandError::ValidationError(fields) => {
                let errors = fields
                    .get("source_path")
                    .expect("source_path field should exist");
                assert_eq!(errors.len(), 1);
                assert_eq!(errors[0].message.as_deref(), Some(msg.as_str()));
            }
            other => panic!("expected ValidationError, got {other:?}"),
        }
    }

    #[test]
    fn confirmation_failed_maps_to_confirmation_validation_error() {
        let msg = "must type CONFIRM".to_string();
        let err: CommandError = DatabaseBackupError::ConfirmationFailed(msg.clone()).into();

        match err {
            CommandError::ValidationError(fields) => {
                let errors = fields
                    .get("confirmation")
                    .expect("confirmation field should exist");
                assert_eq!(errors.len(), 1);
                assert_eq!(errors[0].message.as_deref(), Some(msg.as_str()));
            }
            other => panic!("expected ValidationError, got {other:?}"),
        }
    }

    #[test]
    fn permission_denied_maps_to_permission_denied() {
        let msg = "access denied".to_string();
        let err: CommandError = DatabaseBackupError::PermissionDenied(msg.clone()).into();

        match err {
            CommandError::PermissionDenied(actual) => assert_eq!(actual, msg),
            other => panic!("expected PermissionDenied, got {other:?}"),
        }
    }

    #[test]
    fn fallback_variants_map_to_unknown() {
        let variants = vec![
            DatabaseBackupError::DatabaseError("db failure".to_string()),
            DatabaseBackupError::FileSystemError("io failure".to_string()),
            DatabaseBackupError::OperationInProgress,
        ];

        for variant in variants {
            let err: CommandError = variant.into();
            match err {
                CommandError::Unknown { message, .. } => assert!(!message.is_empty()),
                other => panic!("expected Unknown, got {other:?}"),
            }
        }
    }
}
