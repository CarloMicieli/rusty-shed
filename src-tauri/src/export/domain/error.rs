/// Error types for the export feature
use thiserror::Error;

/// Export-specific errors
#[derive(Error, Debug)]
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
