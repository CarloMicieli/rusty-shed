use crate::data_management::domain::{
    DataContainerDto, DataManagementError, ImageFailure, RecordCounts,
};
use crate::data_management::infrastructure::DuplicateCheckResult;
use async_trait::async_trait;
use std::path::Path;

/// All duplicate-check results grouped for a single import run.
pub struct AllDuplicates {
    pub manufacturer_dupes: DuplicateCheckResult,
    pub railway_model_dupes: DuplicateCheckResult,
    pub collection_item_dupes: DuplicateCheckResult,
    pub seller_dupes: DuplicateCheckResult,
    pub track_product_dupes: DuplicateCheckResult,
    pub track_inventory_dupes: DuplicateCheckResult,
    pub formation_category_dupes: DuplicateCheckResult,
    pub train_formation_dupes: DuplicateCheckResult,
    pub prototype_dupes: DuplicateCheckResult,
    pub wishlist_dupes: DuplicateCheckResult,
    pub decoder_dupes: DuplicateCheckResult,
    pub digital_roster_dupes: DuplicateCheckResult,
}

/// Statistics returned after all records have been persisted.
pub struct PersistResult {
    pub added: RecordCounts,
    pub skipped: RecordCounts,
    pub images_imported: u32,
    pub images_failed: Vec<ImageFailure>,
}

/// Port that abstracts database access for the import use cases.
///
/// Implementations live in the infrastructure layer (e.g., `SqliteImportRepository`).
/// A fake/stub implementation can be provided in tests so use cases run without a real DB.
#[async_trait]
pub trait ImportRepository: Send + Sync {
    /// Run all six duplicate checks and return the combined results.
    async fn check_duplicates(
        &self,
        data: &DataContainerDto,
    ) -> Result<AllDuplicates, DataManagementError>;

    /// Persist all new records (using `duplicates` to skip existing ones),
    /// then copy images from the archive.
    ///
    /// The database writes are performed in a single atomic transaction that is committed
    /// before any file I/O begins.
    async fn persist(
        &self,
        data: &DataContainerDto,
        duplicates: &AllDuplicates,
        archive_path: &Path,
        media_dir: &Path,
    ) -> Result<PersistResult, DataManagementError>;
}
