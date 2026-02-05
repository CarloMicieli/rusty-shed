use crate::import::domain::{ImportResult, ImportSession, ManifestDto, RecordCounts};

/// Executes the import of validated package data into the database.
///
/// **Note**: This is a placeholder implementation for Phase 3 MVP.
/// Full database write operations will be implemented in Phase 3.
///
/// The use case:
/// 1. Uses existing repositories to insert records (manufacturers, models, etc.)
/// 2. Uses MediaStorage to copy images
/// 3. Tracks success/failure for reporting
pub struct ExecuteImportUseCase;

impl ExecuteImportUseCase {
    /// Execute the import use case.
    ///
    /// # Arguments
    /// * `session` - The import session with validated data
    /// * `manifest` - The validated manifest data
    ///
    /// # Returns
    /// An ImportResult with counts of added records and any failures
    pub async fn execute(
        _session: &ImportSession,
        _manifest: &ManifestDto,
    ) -> Result<ImportResult, String> {
        let start = std::time::Instant::now();

        // Placeholder: will be implemented with database writes in Phase 3
        // For now, return a minimal success result
        let result = ImportResult {
            session_id: _session.id.clone(),
            added: RecordCounts::default(),
            skipped: RecordCounts::default(),
            images_imported: 0,
            images_failed: vec![],
            duration_ms: start.elapsed().as_millis() as u64,
            warnings: vec![],
        };

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_import_placeholder() {
        let session = ImportSession::new(
            std::path::PathBuf::from("/tmp/test.zip"),
            crate::import::domain::ArchiveFormat::Zip,
        );
        let manifest = ManifestDto {
            schema: None,
            version: "1.0".to_string(),
            exported_at: None,
            source: None,
            data: crate::import::domain::DataContainerDto::default(),
        };

        let result = ExecuteImportUseCase::execute(&session, &manifest).await;
        assert!(result.is_ok());
        let import_result = result.unwrap();
        assert_eq!(import_result.added.total(), 0);
    }
}
