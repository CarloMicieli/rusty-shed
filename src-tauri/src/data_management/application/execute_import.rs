use crate::data_management::application::ports::ImportRepository;
use crate::data_management::domain::{
    DataManagementError, ImportResult, ImportSession, ManifestDto,
};
use std::path::Path;
use std::sync::Arc;

/// Executes the import of validated package data into the database.
///
/// Delegates all duplicate checking and persistence to the injected `ImportRepository`.
/// This use case contains only orchestration logic — no raw SQL.
pub struct ExecuteImportUseCase {
    repo: Arc<dyn ImportRepository>,
}

impl ExecuteImportUseCase {
    pub fn new(repo: Arc<dyn ImportRepository>) -> Self {
        Self { repo }
    }

    /// Execute the import use case.
    ///
    /// # Returns
    /// An `ImportResult` with counts of added records, skipped duplicates, and image statistics.
    pub async fn execute(
        &self,
        session: &ImportSession,
        manifest: &ManifestDto,
        archive_path: &Path,
        media_dir: &Path,
    ) -> Result<ImportResult, DataManagementError> {
        let start = std::time::Instant::now();

        let duplicates = self.repo.check_duplicates(&manifest.data).await?;
        let stats = self
            .repo
            .persist(&manifest.data, &duplicates, archive_path, media_dir)
            .await?;

        Ok(ImportResult {
            session_id: session.id.clone(),
            added: stats.added,
            skipped: stats.skipped,
            images_imported: stats.images_imported,
            images_failed: stats.images_failed,
            duration_ms: start.elapsed().as_millis() as u64,
            warnings: vec![],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_management::application::ports::{AllDuplicates, PersistResult};
    use crate::data_management::domain::{ArchiveFormat, DataContainerDto, RecordCounts};
    use crate::data_management::infrastructure::DuplicateCheckResult;
    use async_trait::async_trait;

    struct FakeImportRepository;

    #[async_trait]
    impl ImportRepository for FakeImportRepository {
        async fn check_duplicates(
            &self,
            _data: &DataContainerDto,
        ) -> Result<AllDuplicates, DataManagementError> {
            Ok(AllDuplicates {
                manufacturer_dupes: DuplicateCheckResult::default(),
                railway_model_dupes: DuplicateCheckResult::default(),
                collection_item_dupes: DuplicateCheckResult::default(),
                seller_dupes: DuplicateCheckResult::default(),
                track_product_dupes: DuplicateCheckResult::default(),
                track_inventory_dupes: DuplicateCheckResult::default(),
                formation_category_dupes: DuplicateCheckResult::default(),
                train_formation_dupes: DuplicateCheckResult::default(),
                prototype_dupes: DuplicateCheckResult::default(),
            })
        }

        async fn persist(
            &self,
            _data: &DataContainerDto,
            _duplicates: &AllDuplicates,
            _archive_path: &Path,
            _media_dir: &Path,
        ) -> Result<PersistResult, DataManagementError> {
            Ok(PersistResult {
                added: RecordCounts::default(),
                skipped: RecordCounts::default(),
                images_imported: 0,
                images_failed: vec![],
            })
        }
    }

    #[tokio::test]
    async fn test_execute_import_delegates_to_repository() {
        let repo = Arc::new(FakeImportRepository);
        let use_case = ExecuteImportUseCase::new(repo);

        let session = ImportSession::new(
            std::path::PathBuf::from("/tmp/test.zip"),
            ArchiveFormat::Zip,
        );
        let manifest = ManifestDto {
            schema: None,
            version: "1.0".to_string(),
            exported_at: None,
            source: None,
            data: DataContainerDto::default(),
        };

        let result = use_case
            .execute(
                &session,
                &manifest,
                std::path::Path::new("/tmp/test.zip"),
                std::path::Path::new("/tmp"),
            )
            .await;

        assert!(result.is_ok());
        let import_result = result.unwrap();
        assert_eq!(import_result.added.total(), 0);
        assert_eq!(import_result.skipped.total(), 0);
        assert_eq!(import_result.images_imported, 0);
    }
}
