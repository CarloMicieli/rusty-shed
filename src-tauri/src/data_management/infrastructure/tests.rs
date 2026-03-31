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
            include_maintenance_logs: true,
            include_dcc_roster: false,
            include_orphaned_images: false,
            include_track_inventory: false,
            include_train_formations: true,
            include_wishlists: true,
        }
    }

    /// Insert the minimum set of entities needed for a meaningful roundtrip test.
    ///
    /// Entities:
    /// - 1 manufacturer (`trn:manufacturer:test-manufacturer`)
    /// - 1 railway company (`trn:railway-company:test-railway`)
    /// - 1 railway model (`trn:railway-model:test-manufacturer:test-001`) + translations + 1 rolling stock
    /// - 1 default collection (required FK) + 1 collection item (`trn:collection-item:...`)
    /// - 1 seller (`trn:seller:test-shop`)
    async fn seed_pool(pool: &sqlx::SqlitePool) {
        sqlx::query(
            "INSERT INTO manufacturers \
             (id, name, status, street_address, city, postal_code) \
             VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind("trn:manufacturer:test-manufacturer")
        .bind("Test Manufacturer")
        .bind("ACTIVE")
        .bind("Main Street 1")
        .bind("Torino")
        .bind("10121")
        .execute(pool)
        .await
        .expect("seed manufacturer");

        sqlx::query(
            "INSERT INTO railway_companies \
             (id, name, status, operating_since) VALUES (?, ?, ?, ?)",
        )
        .bind("trn:railway-company:test-railway")
        .bind("Test Railway")
        .bind("ACTIVE")
        .bind("1994-01-01")
        .execute(pool)
        .await
        .expect("seed railway company");

        sqlx::query(
            "INSERT INTO railway_models \
             (id, manufacturer_id, product_code, power_method, scale, epoch, category) \
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind("trn:railway-model:test-manufacturer:test-001")
        .bind("trn:manufacturer:test-manufacturer")
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
        .bind("trn:railway-model:test-manufacturer:test-001")
        .bind("en")
        .bind("Test Locomotive")
        .execute(pool)
        .await
        .expect("seed translation");

        sqlx::query(
            "INSERT INTO railway_model_translations \
             (railway_model_id, language_code, description, details) VALUES (?, ?, ?, ?)",
        )
        .bind("trn:railway-model:test-manufacturer:test-001")
        .bind("it")
        .bind("Locomotiva di test")
        .bind("Dettagli in italiano")
        .execute(pool)
        .await
        .expect("seed translation it");

        sqlx::query(
            "INSERT INTO rolling_stocks \
               (id, railway_model_id, category, railway_company_id, series_code, \
                service_level, length_inches, length_millimeters, technical_minimum_radius_mm, \
                technical_coupling_socket, control, is_dummy) \
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind("rs-test-001")
        .bind("trn:railway-model:test-manufacturer:test-001")
        .bind("LOCOMOTIVES")
        .bind("trn:railway-company:test-railway")
        .bind("BR 101")
        .bind("FIRST_SECOND")
        .bind("8.39")
        .bind("213")
        .bind("358")
        .bind("NEM_362")
        .bind("DCC_READY")
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
               (id, collection_id, railway_model_id, added_date, purchase_condition, model_condition, box_condition) \
               VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind("trn:collection-item:00000000-0000-0000-0000-000000000001")
        .bind("trn:collection:1")
        .bind("trn:railway-model:test-manufacturer:test-001")
        .bind("2024-01-15")
           .bind("NEW")
           .bind("MINT")
           .bind("ORIGINAL_MINT")
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
        .bind("trn:collection-item:00000000-0000-0000-0000-000000000001")
        .bind("PURCHASED")
        .bind("2024-01-15")
        .execute(pool)
        .await
        .expect("seed purchase info");

        sqlx::query("INSERT INTO sellers (id, name, type) VALUES (?, ?, ?)")
            .bind("trn:seller:test-shop")
            .bind("Test Shop")
            .bind("SHOP")
            .execute(pool)
            .await
            .expect("seed seller");

        sqlx::query("INSERT INTO owned_rolling_stocks (id, collection_item_id, rolling_stock_id) VALUES (?, ?, ?)")
            .bind("ors-test-001")
            .bind("trn:collection-item:00000000-0000-0000-0000-000000000001")
            .bind("rs-test-001")
            .execute(pool)
            .await
            .expect("seed owned rolling stock");

        sqlx::query(
            "INSERT INTO maintenance_cards (id, owned_rolling_stock_id, last_maintenance_date) VALUES (?, ?, ?)",
        )
        .bind("trn:maintenance-card:00000000-0000-0000-0000-000000000101")
        .bind("ors-test-001")
        .bind("2024-07-01")
        .execute(pool)
        .await
        .expect("seed maintenance card");

        sqlx::query(
            "INSERT INTO maintenance_events (id, maintenance_card_id, date_performed, maintenance_type, notes) VALUES (?, ?, ?, ?, ?)",
        )
        .bind("trn:maintenance-event:00000000-0000-0000-0000-000000000201")
        .bind("trn:maintenance-card:00000000-0000-0000-0000-000000000101")
        .bind("2024-07-01")
        .bind("GENERAL_INSPECTION")
        .bind("Inspection complete")
        .execute(pool)
        .await
        .expect("seed maintenance event");

        sqlx::query("INSERT INTO formation_categories (id, name, is_custom) VALUES (?, ?, ?)")
            .bind("fc-test-001")
            .bind("Test Category")
            .bind(1_i64)
            .execute(pool)
            .await
            .expect("seed formation category");

        sqlx::query(
            "INSERT INTO prototypes \
             (id, railway_company_id, series_code, car_type, category, is_motorized, is_custom) \
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind("proto-test-001")
        .bind("trn:railway-company:test-railway")
        .bind("BR 101")
        .bind("Locomotive")
        .bind("LOCOMOTIVES")
        .bind(1_i64)
        .bind(1_i64)
        .execute(pool)
        .await
        .expect("seed prototype");

        sqlx::query(
            "INSERT INTO train_formations \
             (id, name, category_id, epoch, notes) \
             VALUES (?, ?, ?, ?, ?)",
        )
        .bind("tf-test-001")
        .bind("Test Formation")
        .bind("fc-test-001")
        .bind("VI")
        .bind("Roundtrip test formation")
        .execute(pool)
        .await
        .expect("seed train formation");

        sqlx::query(
            "INSERT INTO formation_elements \
             (id, formation_id, prototype_id, position_order, traction_override) \
             VALUES (?, ?, ?, ?, ?)",
        )
        .bind("fe-test-001")
        .bind("tf-test-001")
        .bind("proto-test-001")
        .bind(0_i64)
        .bind(0_i64)
        .execute(pool)
        .await
        .expect("seed formation element");

        sqlx::query(
            "INSERT INTO wishlists (id, name, notes, is_default, version) VALUES (?, ?, ?, ?, ?)",
        )
        .bind("wl-test-001")
        .bind("Test Wishlist")
        .bind("My wishlist notes")
        .bind(0_i64)
        .bind(0_i64)
        .execute(pool)
        .await
        .expect("seed wishlist");

        sqlx::query(
            "INSERT INTO wishlist_items \
             (id, wishlist_id, railway_model_id, priority, status, added_date, desired_price_amount, desired_price_currency) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind("wi-test-001")
        .bind("wl-test-001")
        .bind("trn:railway-model:test-manufacturer:test-001")
        .bind("NORMAL")
        .bind("WANTED")
        .bind("2024-02-01")
        .bind(12000_i64)
        .bind("EUR")
        .execute(pool)
        .await
        .expect("seed wishlist item");
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
    /// The helper replaces `:` with `_`, so the image file name must mirror the model id.
    #[sqlx::test(migrations = "./migrations")]
    async fn roundtrip_preserves_all_exported_entities(pool: sqlx::SqlitePool) {
        seed_pool(&pool).await;

        // Create a fake image so the archive is non-trivial
        let media_dir = tempfile::tempdir().expect("media tempdir");
        tokio::fs::write(
            media_dir
                .path()
                .join("trn_railway-model_test-manufacturer_test-001.png"),
            b"FAKE_PNG_DATA",
        )
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
            result.added.maintenance_cards, 1,
            "one maintenance card added"
        );
        assert_eq!(
            result.added.train_formations, 1,
            "one train formation added"
        );
        assert_eq!(result.added.wishlists, 1, "one wishlist added");
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
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM manufacturers WHERE id = 'trn:manufacturer:test-manufacturer'",
        )
        .fetch_one(&import_pool)
        .await
        .unwrap();
        assert_eq!(count, 1, "manufacturer must exist in import DB");

        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM railway_models WHERE id = 'trn:railway-model:test-manufacturer:test-001'")
                .fetch_one(&import_pool)
                .await
                .unwrap();
        assert_eq!(count, 1, "railway model must exist in import DB");

        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM collection_items WHERE id = 'trn:collection-item:00000000-0000-0000-0000-000000000001'")
                .fetch_one(&import_pool)
                .await
                .unwrap();
        assert_eq!(count, 1, "collection item must exist in import DB");

        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM sellers WHERE id = 'trn:seller:test-shop'")
                .fetch_one(&import_pool)
                .await
                .unwrap();
        assert_eq!(count, 1, "seller must exist in import DB");

        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM train_formations WHERE id = 'tf-test-001'")
                .fetch_one(&import_pool)
                .await
                .unwrap();
        assert_eq!(count, 1, "train formation must exist in import DB");

        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM formation_elements WHERE formation_id = 'tf-test-001'",
        )
        .fetch_one(&import_pool)
        .await
        .unwrap();
        assert_eq!(count, 1, "formation element must exist in import DB");

        let it_translation_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM railway_model_translations \
             WHERE railway_model_id = 'trn:railway-model:test-manufacturer:test-001' AND language_code = 'it' \
               AND description = 'Locomotiva di test'",
        )
        .fetch_one(&import_pool)
        .await
        .unwrap();
        assert_eq!(
            it_translation_count, 1,
            "italian translation must roundtrip"
        );

        let rs_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM rolling_stocks \
             WHERE id = 'rs-test-001' AND railway_model_id = 'trn:railway-model:test-manufacturer:test-001' \
               AND service_level = 'FIRST_SECOND' AND length_millimeters = '213' \
               AND technical_coupling_socket = 'NEM_362' AND control = 'DCC_READY'",
        )
        .fetch_one(&import_pool)
        .await
        .unwrap();
        assert_eq!(rs_count, 1, "rolling stock extended fields must roundtrip");

        let maintenance_event_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM maintenance_events \
             WHERE id = 'trn:maintenance-event:00000000-0000-0000-0000-000000000201' AND maintenance_type = 'GENERAL_INSPECTION'",
        )
        .fetch_one(&import_pool)
        .await
        .unwrap();
        assert_eq!(
            maintenance_event_count, 1,
            "maintenance events must roundtrip"
        );

        let wishlist_count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM wishlists WHERE id = 'wl-test-001'")
                .fetch_one(&import_pool)
                .await
                .unwrap();
        assert_eq!(wishlist_count, 1, "wishlist must exist in import DB");

        let wishlist_item_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM wishlist_items WHERE id = 'wi-test-001' \
             AND wishlist_id = 'wl-test-001' AND status = 'WANTED' \
             AND desired_price_amount = 12000 AND desired_price_currency = 'EUR'",
        )
        .fetch_one(&import_pool)
        .await
        .unwrap();
        assert_eq!(wishlist_item_count, 1, "wishlist item must roundtrip");
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
        tokio::fs::write(
            media_dir
                .path()
                .join("trn_railway-model_test-manufacturer_test-001.png"),
            b"FAKE_PNG_DATA",
        )
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
        assert_eq!(first.added.maintenance_cards, 1);
        assert_eq!(first.added.train_formations, 1);
        assert_eq!(first.added.wishlists, 1);

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
        assert_eq!(
            second.added.maintenance_cards, 0,
            "no new maintenance cards on re-import"
        );
        assert_eq!(
            second.skipped.maintenance_cards, 1,
            "duplicate maintenance card must be skipped"
        );
        assert_eq!(
            second.added.train_formations, 0,
            "no new train formations on re-import"
        );
        assert_eq!(
            second.skipped.train_formations, 1,
            "duplicate train formation must be skipped"
        );
        assert_eq!(second.added.wishlists, 0, "no new wishlists on re-import");
        assert_eq!(
            second.skipped.wishlists, 1,
            "duplicate wishlist must be skipped"
        );

        // Row counts must not grow
        let mfr_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM manufacturers WHERE id = 'trn:manufacturer:test-manufacturer'",
        )
        .fetch_one(&import_pool)
        .await
        .unwrap();
        assert_eq!(mfr_count, 1, "manufacturer must not be duplicated");

        let model_count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM railway_models WHERE id = 'trn:railway-model:test-manufacturer:test-001'")
                .fetch_one(&import_pool)
                .await
                .unwrap();
        assert_eq!(model_count, 1, "railway model must not be duplicated");

        let item_count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM collection_items WHERE id = 'trn:collection-item:00000000-0000-0000-0000-000000000001'")
                .fetch_one(&import_pool)
                .await
                .unwrap();
        assert_eq!(item_count, 1, "collection item must not be duplicated");
    }
}
