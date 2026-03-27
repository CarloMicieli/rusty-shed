/// Integration tests for the data_management export/import pipeline.
///
/// These tests exercise the full roundtrip:
/// 1. Seed a real SQLite in-memory database with known entities
/// 2. Run `export_to_archive` to produce a ZIP archive
/// 3. Run `ValidatePackageUseCase` + `ExecuteImportUseCase` against a fresh DB
/// 4. Assert that the imported data matches what was exported
///
/// Tests live in the infrastructure layer because they require a real `SqlitePool`
/// and real archive I/O — both infrastructure concerns.
#[cfg(test)]
mod roundtrip {
    use crate::data_management::application::execute_export::export_to_archive;
    use crate::data_management::application::{ExecuteImportUseCase, ValidatePackageUseCase};
    use crate::data_management::domain::{ExportEntitySelection, ImportResult, ImportSession};
    use crate::data_management::infrastructure::SqliteImportRepository;
    use std::path::Path;
    use std::sync::Arc;

    // ── Helpers ───────────────────────────────────────────────────────────────

    /// Entity selection used by all roundtrip tests.
    fn test_selection() -> ExportEntitySelection {
        ExportEntitySelection {
            include_railway_models: true,
            include_collection_items: true,
            include_sellers: true,
            include_maintenance_logs: false,
            include_dcc_roster: false,
            include_orphaned_images: false,
            include_track_inventory: false,
        }
    }

    /// Insert the minimum set of entities needed for a meaningful roundtrip test.
    ///
    /// Entities:
    /// - 1 manufacturer (`mfr-test-001`)
    /// - 1 railway company (`rc-test-001`)
    /// - 1 railway model (`rm-test-001`) + English translation + 1 rolling stock
    /// - 1 default collection (required FK) + 1 collection item (`ci-test-001`)
    /// - 1 seller (`seller-test-001`)
    async fn seed_pool(pool: &sqlx::SqlitePool) {
        sqlx::query("INSERT INTO manufacturers (id, name, status) VALUES (?, ?, ?)")
            .bind("mfr-test-001")
            .bind("Test Manufacturer")
            .bind("ACTIVE")
            .execute(pool)
            .await
            .expect("seed manufacturer");

        sqlx::query("INSERT INTO railway_companies (id, name, status) VALUES (?, ?, ?)")
            .bind("rc-test-001")
            .bind("Test Railway")
            .bind("ACTIVE")
            .execute(pool)
            .await
            .expect("seed railway company");

        sqlx::query(
            "INSERT INTO railway_models \
             (id, manufacturer_id, product_code, power_method, scale, epoch, category) \
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind("rm-test-001")
        .bind("mfr-test-001")
        .bind("TEST-001")
        .bind("DC")
        .bind("H0")
        .bind("VI")
        .bind("LOCOMOTIVES")
        .execute(pool)
        .await
        .expect("seed railway model");

        sqlx::query(
            "INSERT INTO railway_model_translations \
             (railway_model_id, language_code, description) VALUES (?, ?, ?)",
        )
        .bind("rm-test-001")
        .bind("en")
        .bind("Test Locomotive")
        .execute(pool)
        .await
        .expect("seed translation");

        sqlx::query(
            "INSERT INTO rolling_stocks \
             (id, railway_model_id, category, railway_company_id, series_code, is_dummy) \
             VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind("rs-test-001")
        .bind("rm-test-001")
        .bind("LOCOMOTIVES")
        .bind("rc-test-001")
        .bind("BR 101")
        .bind(0_i64)
        .execute(pool)
        .await
        .expect("seed rolling stock");

        sqlx::query("INSERT INTO collections (id, name) VALUES (?, ?)")
            .bind("trn:collection:1")
            .bind("My Collection")
            .execute(pool)
            .await
            .expect("seed collection");

        sqlx::query(
            "INSERT INTO collection_items \
             (id, collection_id, railway_model_id, added_date) VALUES (?, ?, ?, ?)",
        )
        .bind("ci-test-001")
        .bind("trn:collection:1")
        .bind("rm-test-001")
        .bind("2024-01-15")
        .execute(pool)
        .await
        .expect("seed collection item");

        // A purchase record is required for collection-item deduplication: the
        // duplicate checker keys on `railway_model_id + purchase_date`, and only
        // considers items with a purchase as potentially duplicate.
        sqlx::query(
            "INSERT INTO purchase_infos \
             (id, collection_item_id, purchase_type, purchase_date) VALUES (?, ?, ?, ?)",
        )
        .bind("pi-test-001")
        .bind("ci-test-001")
        .bind("PURCHASED")
        .bind("2024-01-15")
        .execute(pool)
        .await
        .expect("seed purchase info");

        sqlx::query("INSERT INTO sellers (id, name, type) VALUES (?, ?, ?)")
            .bind("seller-test-001")
            .bind("Test Shop")
            .bind("SHOP")
            .execute(pool)
            .await
            .expect("seed seller");
    }

    /// Export the seeded pool to a new archive and return the archive path.
    ///
    /// The archive is written to `test_exports/` (gitignored) with a UUID filename
    /// so concurrent test runs never collide.
    async fn build_archive(pool: &sqlx::SqlitePool, media_dir: &Path) -> std::path::PathBuf {
        let test_exports = Path::new("test_exports");
        tokio::fs::create_dir_all(test_exports)
            .await
            .expect("create test_exports dir");

        let archive_path = test_exports.join(format!("{}.zip", uuid::Uuid::new_v4()));
        export_to_archive(pool, &archive_path, media_dir, &test_selection())
            .await
            .expect("export_to_archive should succeed");
        archive_path
    }

    /// Run a full import pipeline against the given pool and return the result.
    async fn run_import(
        import_pool: &sqlx::SqlitePool,
        archive_path: &Path,
        import_media_dir: &Path,
    ) -> ImportResult {
        let (format, manifest, _counts) = ValidatePackageUseCase::execute(archive_path)
            .await
            .expect("ValidatePackageUseCase should succeed");

        let repo = SqliteImportRepository::new(import_pool.clone());
        let use_case = ExecuteImportUseCase::new(Arc::new(repo));
        let session = ImportSession::new(archive_path.to_path_buf(), format);

        use_case
            .execute(&session, &manifest, archive_path, import_media_dir)
            .await
            .expect("ExecuteImportUseCase should succeed")
    }

    /// Create a fresh in-memory SQLite pool with all migrations applied.
    async fn fresh_import_pool() -> sqlx::SqlitePool {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .expect("create in-memory import pool");
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("apply migrations to import pool");
        pool
    }

    // ── Tests ─────────────────────────────────────────────────────────────────

    /// Full export → import roundtrip: all seeded entities must appear in the
    /// import database with the same IDs.
    ///
    /// The media directory contains one fake PNG whose name follows the
    /// `probe_model_image` convention (`{model_id}.png`, colons replaced by `_`).
    /// Since `rm-test-001` has no colons the filename is simply `rm-test-001.png`.
    #[sqlx::test(migrations = "./migrations")]
    async fn roundtrip_preserves_all_exported_entities(pool: sqlx::SqlitePool) {
        seed_pool(&pool).await;

        // Create a fake image so the archive is non-trivial
        let media_dir = tempfile::tempdir().expect("media tempdir");
        tokio::fs::write(media_dir.path().join("rm-test-001.png"), b"FAKE_PNG_DATA")
            .await
            .expect("write fake image");

        let archive_path = build_archive(&pool, media_dir.path()).await;
        assert!(
            archive_path.exists(),
            "archive file must exist after export"
        );
        assert!(
            tokio::fs::metadata(&archive_path).await.unwrap().len() > 0,
            "archive must not be empty"
        );

        let import_pool = fresh_import_pool().await;
        let import_media_dir = tempfile::tempdir().expect("import media tempdir");

        let result = run_import(&import_pool, &archive_path, import_media_dir.path()).await;

        // --- ImportResult counters ---
        assert_eq!(result.added.manufacturers, 1, "one manufacturer added");
        assert_eq!(result.added.railway_models, 1, "one railway model added");
        assert_eq!(
            result.added.collection_items, 1,
            "one collection item added"
        );
        assert_eq!(result.added.sellers, 1, "one seller added");
        assert_eq!(
            result.skipped.manufacturers, 0,
            "no duplicate manufacturers"
        );
        assert_eq!(
            result.skipped.railway_models, 0,
            "no duplicate railway models"
        );
        assert_eq!(
            result.skipped.collection_items, 0,
            "no duplicate collection items"
        );
        assert_eq!(result.skipped.sellers, 0, "no duplicate sellers");
        assert!(result.images_failed.is_empty(), "no image failures");

        // --- Database state ---
        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM manufacturers WHERE id = 'mfr-test-001'")
                .fetch_one(&import_pool)
                .await
                .unwrap();
        assert_eq!(count, 1, "manufacturer must exist in import DB");

        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM railway_models WHERE id = 'rm-test-001'")
                .fetch_one(&import_pool)
                .await
                .unwrap();
        assert_eq!(count, 1, "railway model must exist in import DB");

        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM collection_items WHERE id = 'ci-test-001'")
                .fetch_one(&import_pool)
                .await
                .unwrap();
        assert_eq!(count, 1, "collection item must exist in import DB");

        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM sellers WHERE id = 'seller-test-001'")
                .fetch_one(&import_pool)
                .await
                .unwrap();
        assert_eq!(count, 1, "seller must exist in import DB");
    }

    /// Importing the same archive twice must not create duplicates.
    ///
    /// On the second import, all entities with explicit deduplication
    /// (manufacturers, railway_models, collection_items, sellers) must be
    /// counted as skipped, not added.
    ///
    /// Note: `railway_companies` uses `INSERT OR IGNORE` without a dedicated
    /// deduplication phase, so its counter is excluded from the skipped assertions.
    #[sqlx::test(migrations = "./migrations")]
    async fn second_import_skips_duplicates(pool: sqlx::SqlitePool) {
        seed_pool(&pool).await;

        let media_dir = tempfile::tempdir().expect("media tempdir");
        tokio::fs::write(media_dir.path().join("rm-test-001.png"), b"FAKE_PNG_DATA")
            .await
            .expect("write fake image");

        let archive_path = build_archive(&pool, media_dir.path()).await;

        let import_pool = fresh_import_pool().await;
        let import_media_dir = tempfile::tempdir().expect("import media tempdir");

        // First import — baseline
        let first = run_import(&import_pool, &archive_path, import_media_dir.path()).await;
        assert_eq!(first.added.manufacturers, 1);
        assert_eq!(first.added.railway_models, 1);
        assert_eq!(first.added.collection_items, 1);
        assert_eq!(first.added.sellers, 1);

        // Second import of the same archive into the same pool
        let second = run_import(&import_pool, &archive_path, import_media_dir.path()).await;

        assert_eq!(
            second.added.manufacturers, 0,
            "no new manufacturers on re-import"
        );
        assert_eq!(
            second.skipped.manufacturers, 1,
            "duplicate manufacturer must be skipped"
        );
        assert_eq!(
            second.added.railway_models, 0,
            "no new railway models on re-import"
        );
        assert_eq!(
            second.skipped.railway_models, 1,
            "duplicate railway model must be skipped"
        );
        assert_eq!(
            second.added.collection_items, 0,
            "no new collection items on re-import"
        );
        assert_eq!(
            second.skipped.collection_items, 1,
            "duplicate collection item must be skipped"
        );
        assert_eq!(second.added.sellers, 0, "no new sellers on re-import");
        assert_eq!(
            second.skipped.sellers, 1,
            "duplicate seller must be skipped"
        );

        // Row counts must not grow
        let mfr_count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM manufacturers WHERE id = 'mfr-test-001'")
                .fetch_one(&import_pool)
                .await
                .unwrap();
        assert_eq!(mfr_count, 1, "manufacturer must not be duplicated");

        let model_count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM railway_models WHERE id = 'rm-test-001'")
                .fetch_one(&import_pool)
                .await
                .unwrap();
        assert_eq!(model_count, 1, "railway model must not be duplicated");

        let item_count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM collection_items WHERE id = 'ci-test-001'")
                .fetch_one(&import_pool)
                .await
                .unwrap();
        assert_eq!(item_count, 1, "collection item must not be duplicated");
    }
}
