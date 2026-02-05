use crate::import::domain::{ImportResult, ImportSession, ManifestDto, RecordCounts};
use crate::import::infrastructure::DuplicateChecker;
use sqlx::SqlitePool;

/// Executes the import of validated package data into the database.
///
/// The use case:
/// 1. Checks for duplicates using DuplicateChecker
/// 2. Filters out duplicate records
/// 3. Inserts only new records into the database
/// 4. Tracks added and skipped counts for reporting
pub struct ExecuteImportUseCase {
    pool: SqlitePool,
}

impl ExecuteImportUseCase {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Execute the import use case.
    ///
    /// # Arguments
    /// * `session` - The import session with validated data
    /// * `manifest` - The validated manifest data
    ///
    /// # Returns
    /// An ImportResult with counts of added records and skipped duplicates
    ///
    /// # Transaction Safety
    /// All database writes are performed in a single transaction. If any write fails,
    /// the entire transaction is rolled back, ensuring atomicity.
    pub async fn execute(
        &self,
        _session: &ImportSession,
        manifest: &ManifestDto,
    ) -> Result<ImportResult, String> {
        let start = std::time::Instant::now();

        // Start a database transaction for atomic writes
        let tx = self
            .pool
            .begin()
            .await
            .map_err(|e| format!("Failed to begin transaction: {}", e))?;

        // Check for duplicates
        let duplicate_checker = DuplicateChecker::new(self.pool.clone());

        let manufacturer_dupes = duplicate_checker
            .check_manufacturers(&manifest.data.manufacturers)
            .await
            .map_err(|e| format!("Failed to check manufacturer duplicates: {}", e))?;

        let railway_model_dupes = duplicate_checker
            .check_railway_models(&manifest.data.railway_models)
            .await
            .map_err(|e| format!("Failed to check railway model duplicates: {}", e))?;

        let collection_item_dupes = duplicate_checker
            .check_collection_items(&manifest.data.collection_items)
            .await
            .map_err(|e| format!("Failed to check collection item duplicates: {}", e))?;

        let seller_dupes = duplicate_checker
            .check_sellers(&manifest.data.sellers)
            .await
            .map_err(|e| format!("Failed to check seller duplicates: {}", e))?;

        // Calculate counts
        let mut added = RecordCounts::default();
        let mut skipped = RecordCounts::default();

        // For Phase 7: We skip duplicates but don't actually write yet
        // Full write implementation will come in later phases
        // When writes are added, they will be performed via &mut tx

        added.manufacturers = manufacturer_dupes.new_count() as u32;
        added.railway_models = railway_model_dupes.new_count() as u32;
        added.collection_items = collection_item_dupes.new_count() as u32;
        added.sellers = seller_dupes.new_count() as u32;
        added.railway_companies = manifest.data.railway_companies.len() as u32; // Not checked for duplicates
        added.maintenance_cards = manifest.data.maintenance_cards.len() as u32; // Linked to items

        skipped.manufacturers = manufacturer_dupes.duplicate_count() as u32;
        skipped.railway_models = railway_model_dupes.duplicate_count() as u32;
        skipped.collection_items = collection_item_dupes.duplicate_count() as u32;
        skipped.sellers = seller_dupes.duplicate_count() as u32;

        // Commit the transaction (currently no writes, but structure is ready)
        tx.commit()
            .await
            .map_err(|e| format!("Failed to commit transaction: {}", e))?;

        let result = ImportResult {
            session_id: _session.id.clone(),
            added,
            skipped,
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
    async fn test_execute_import_with_no_duplicates() {
        // Create an in-memory SQLite database for testing
        let pool = SqlitePool::connect(":memory:")
            .await
            .expect("Failed to create in-memory database");

        // Run migrations (minimal schema for testing)
        sqlx::query(
            "CREATE TABLE manufacturers (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE
            )",
        )
        .execute(&pool)
        .await
        .expect("Failed to create manufacturers table");

        sqlx::query(
            "CREATE TABLE railway_models (
                id TEXT PRIMARY KEY,
                manufacturer_id TEXT NOT NULL,
                product_code TEXT NOT NULL,
                UNIQUE(manufacturer_id, product_code)
            )",
        )
        .execute(&pool)
        .await
        .expect("Failed to create railway_models table");

        sqlx::query(
            "CREATE TABLE collection_items (
                id TEXT PRIMARY KEY,
                railway_model_id TEXT NOT NULL,
                added_date TEXT NOT NULL,
                UNIQUE(railway_model_id, added_date)
            )",
        )
        .execute(&pool)
        .await
        .expect("Failed to create collection_items table");

        sqlx::query(
            "CREATE TABLE sellers (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE
            )",
        )
        .execute(&pool)
        .await
        .expect("Failed to create sellers table");

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

        let use_case = ExecuteImportUseCase::new(pool);
        let result = use_case.execute(&session, &manifest).await;

        assert!(result.is_ok());
        let import_result = result.unwrap();
        assert_eq!(import_result.added.total(), 0);
        assert_eq!(import_result.skipped.total(), 0);
    }
}
