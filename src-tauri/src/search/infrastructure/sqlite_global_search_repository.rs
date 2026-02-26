use async_trait::async_trait;
use sqlx::SqliteConnection;

use crate::catalog::domain::railway_model::RailwayModelId;
use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::search::domain::global_search_result::{GlobalSearchResult, SearchSource};
use crate::search::domain::repository::{GlobalSearchRepository, GlobalSearchUowExt};

/// SQLite implementation of [`GlobalSearchRepository`].
///
/// Executes FTS5 queries against `railway_model_search_idx` and joins the
/// results with `collection_items` and `wishlist_items` to attach source context.
pub struct SqliteGlobalSearchRepository<'conn> {
    executor: &'conn mut SqliteConnection,
}

impl<'conn> SqliteGlobalSearchRepository<'conn> {
    /// Create a new repository bound to the given database connection/executor.
    pub fn new(executor: &'conn mut SqliteConnection) -> Self {
        Self { executor }
    }
}

/// Rebuild the FTS5 search index for a single railway model.
///
/// Runs within the **same transaction** as the caller — no new transaction is started.
/// Deletes all existing FTS5 rows for `model_id`, then re-inserts one row per
/// language translation joined with the manufacturer and rolling stock data.
///
/// # Arguments
/// * `model_id` - The string ID of the railway model to rebuild (e.g. a ULID/UUID string).
/// * `executor` - The active database connection / transaction executor.
pub async fn rebuild_search_index(
    model_id: &str,
    executor: &mut SqliteConnection,
) -> Result<(), DomainError> {
    // Step 1: Remove all existing FTS5 rows for this model.
    sqlx::query("DELETE FROM railway_model_search_idx WHERE railway_model_id = ?1")
        .bind(model_id)
        .execute(&mut *executor)
        .await
        .map_err(DomainError::from)?;

    // Step 2: Re-insert one row per language translation.
    sqlx::query(
        r#"
        INSERT INTO railway_model_search_idx (
            railway_model_id,
            language_code,
            description,
            details,
            manufacturer_name,
            rolling_stocks_text
        )
        SELECT
            rmt.railway_model_id,
            rmt.language_code,
            COALESCE(rmt.description, ''),
            COALESCE(rmt.details, ''),
            COALESCE(m.name, ''),
            COALESCE(
                (SELECT group_concat(
                    COALESCE(rs.road_number, '') || ' ' ||
                    COALESCE(rs.series_code, '') || ' ' ||
                    COALESCE(rs.livery, '') || ' ' ||
                    COALESCE(rs.depot, ''),
                    ' '
                 )
                 FROM rolling_stocks rs
                 WHERE rs.railway_model_id = ?1),
                ''
            )
        FROM railway_model_translations rmt
        JOIN railway_models rm ON rm.id  = rmt.railway_model_id
        JOIN manufacturers m   ON m.id   = rm.manufacturer_id
        WHERE rmt.railway_model_id = ?1
        "#,
    )
    .bind(model_id)
    .execute(&mut *executor)
    .await
    .map_err(DomainError::from)?;

    Ok(())
}

/// Row type used internally to map the FTS5 search query results.
#[derive(Debug, sqlx::FromRow)]
struct SearchResultRow {
    railway_model_id: String,
    collection_item_id: Option<String>,
    wishlist_item_id: Option<String>,
    wishlist_id: Option<String>,
    display_name: Option<String>,
    manufacturer_name: String,
}

#[async_trait]
impl<'conn> GlobalSearchRepository for SqliteGlobalSearchRepository<'conn> {
    async fn search(
        &mut self,
        query: &str,
        lang: &str,
    ) -> Result<Vec<GlobalSearchResult>, DomainError> {
        let sql = r#"
            SELECT
                si.railway_model_id,
                ci.id           AS collection_item_id,
                wi.id           AS wishlist_item_id,
                wi.wishlist_id  AS wishlist_id,
                COALESCE(
                    (SELECT description
                     FROM railway_model_translations
                     WHERE railway_model_id = si.railway_model_id
                       AND language_code = ?2
                     LIMIT 1),
                    (SELECT description
                     FROM railway_model_translations
                     WHERE railway_model_id = si.railway_model_id
                       AND language_code = 'en'
                     LIMIT 1),
                    ''
                )               AS display_name,
                m.name          AS manufacturer_name,
                bm25(railway_model_search_idx) AS rank
            FROM railway_model_search_idx si
            JOIN railway_models rm ON rm.id = si.railway_model_id
            JOIN manufacturers m   ON m.id  = rm.manufacturer_id
            LEFT JOIN collection_items ci
                ON ci.railway_model_id = si.railway_model_id
               AND ci.removed_date IS NULL
            LEFT JOIN wishlist_items wi
                ON wi.railway_model_id = si.railway_model_id
               AND wi.removed_date IS NULL
            WHERE railway_model_search_idx MATCH ?1
              AND si.language_code = ?2
              AND (ci.id IS NOT NULL OR wi.id IS NOT NULL)
            ORDER BY rank
            LIMIT 50
        "#;

        let rows = sqlx::query_as::<_, SearchResultRow>(sql)
            .bind(query)
            .bind(lang)
            .fetch_all(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;

        let mut results = Vec::with_capacity(rows.len() * 2);

        for row in rows {
            let railway_model_id = RailwayModelId::try_from(row.railway_model_id.as_str())
                .map_err(|e| DomainError::Validation(format!("invalid railway model id: {e}")))?;
            let display_name = row.display_name.unwrap_or_default();

            // Emit a result for each source this model appears in.
            if let Some(item_id) = row.collection_item_id {
                results.push(GlobalSearchResult {
                    railway_model_id: railway_model_id.clone(),
                    source: SearchSource::Collection,
                    item_id,
                    parent_id: None,
                    display_name: display_name.clone(),
                    manufacturer_name: row.manufacturer_name.clone(),
                });
            }

            if let Some(item_id) = row.wishlist_item_id {
                results.push(GlobalSearchResult {
                    railway_model_id,
                    source: SearchSource::Wishlist,
                    item_id,
                    parent_id: row.wishlist_id,
                    display_name,
                    manufacturer_name: row.manufacturer_name,
                });
            }
        }

        Ok(results)
    }
}

impl<'conn> GlobalSearchUowExt for SqliteUnitOfWork<'conn> {
    fn global_search_repo(&mut self) -> Box<dyn GlobalSearchRepository + '_> {
        Box::new(SqliteGlobalSearchRepository::new(&mut self.tx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
    use crate::search::domain::repository::GlobalSearchUowExt;

    // Helper to insert a manufacturer, railway_model, translation, collection and wishlist
    async fn seed_test_data(pool: &SqlitePool, model_id: &str, manufacturer_id: &str) {
        let mut conn = pool.acquire().await.unwrap();

        // Insert manufacturer
        sqlx::query(
            "INSERT INTO manufacturers (id, name, status, created_at, updated_at, version) \
             VALUES (?1, ?2, 'ACTIVE', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1)",
        )
        .bind(manufacturer_id)
        .bind("Test Brand")
        .execute(&mut *conn)
        .await
        .unwrap();

        // Insert railway model
        sqlx::query(
            "INSERT INTO railway_models (id, manufacturer_id, product_code, power_method, scale, \
             epoch, category, created_at, updated_at, version) \
             VALUES (?1, ?2, 'TEST-001', 'DC', 'H0', 'VI', 'LOCOMOTIVES', \
             CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1)",
        )
        .bind(model_id)
        .bind(manufacturer_id)
        .execute(&mut *conn)
        .await
        .unwrap();

        // Insert translation
        sqlx::query(
            "INSERT INTO railway_model_translations (railway_model_id, language_code, description, details) \
             VALUES (?1, 'en', 'Test locomotive description', 'Some details')",
        )
        .bind(model_id)
        .execute(&mut *conn)
        .await
        .unwrap();
    }

    async fn seed_rolling_stock(pool: &SqlitePool, rs_id: &str, model_id: &str, road_number: &str) {
        let mut conn = pool.acquire().await.unwrap();

        // Ensure railway company exists
        sqlx::query(
            "INSERT OR IGNORE INTO railway_companies (id, name, status, created_at, updated_at) \
             VALUES ('trn:railway-company:trenitalia', 'Trenitalia', 'ACTIVE', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
        )
        .execute(&mut *conn)
        .await
        .unwrap();

        sqlx::query(
            "INSERT INTO rolling_stocks (id, railway_model_id, category, railway_company_id, \
             road_number, series_code) \
             VALUES (?1, ?2, 'LOCOMOTIVE', 'trn:railway-company:trenitalia', ?3, 'E636')",
        )
        .bind(rs_id)
        .bind(model_id)
        .bind(road_number)
        .execute(&mut *conn)
        .await
        .unwrap();
    }

    async fn seed_collection(pool: &SqlitePool, item_id: &str, model_id: &str) {
        let collection_id = "trn:collection:1";
        let mut conn = pool.acquire().await.unwrap();

        // Ensure collection exists
        sqlx::query("INSERT OR IGNORE INTO collections (id, name) VALUES (?1, 'My Collection')")
            .bind(collection_id)
            .execute(&mut *conn)
            .await
            .unwrap();

        sqlx::query(
            "INSERT INTO collection_items (id, collection_id, railway_model_id, added_date) \
             VALUES (?1, ?2, ?3, '2024-01-01')",
        )
        .bind(item_id)
        .bind(collection_id)
        .bind(model_id)
        .execute(&mut *conn)
        .await
        .unwrap();
    }

    async fn seed_wishlist_and_item(
        pool: &SqlitePool,
        wishlist_id: &str,
        item_id: &str,
        model_id: &str,
    ) {
        let mut conn = pool.acquire().await.unwrap();
        sqlx::query(
            "INSERT INTO wishlists (id, name, is_default, version, created_at, updated_at) \
             VALUES (?1, 'My Wishlist', 0, 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
        )
        .bind(wishlist_id)
        .execute(&mut *conn)
        .await
        .unwrap();

        sqlx::query(
            "INSERT INTO wishlist_items (id, wishlist_id, railway_model_id, priority, status, \
             added_date) \
             VALUES (?1, ?2, ?3, 'MEDIUM', 'PENDING', '2024-01-01')",
        )
        .bind(item_id)
        .bind(wishlist_id)
        .bind(model_id)
        .execute(&mut *conn)
        .await
        .unwrap();
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_match_by_description(pool: SqlitePool) {
        let model_id = "trn:railway-model:testbrand:desc-001";
        let manufacturer_id = "trn:manufacturer:testbrand";

        seed_test_data(&pool, model_id, manufacturer_id).await;
        seed_collection(&pool, "col-item-desc-001", model_id).await;

        // Rebuild FTS5 index
        let mut conn = pool.acquire().await.unwrap();
        rebuild_search_index(model_id, &mut conn).await.unwrap();
        drop(conn);

        let mut uow = SqliteUnitOfWork::new(&pool).await.unwrap();
        let mut repo = uow.global_search_repo();
        let results = repo.search("\"Test locomotive\"*", "en").await.unwrap();

        assert!(!results.is_empty(), "should find at least one result");
        let found = results.iter().any(|r| r.source == SearchSource::Collection);
        assert!(found, "should find a collection result");
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_match_by_manufacturer_name(pool: SqlitePool) {
        let model_id = "trn:railway-model:searchbrand:mfr-001";
        let manufacturer_id = "trn:manufacturer:searchbrand";

        seed_test_data(&pool, model_id, manufacturer_id).await;
        // Override manufacturer name
        let mut conn = pool.acquire().await.unwrap();
        sqlx::query("UPDATE manufacturers SET name = 'SearchBrand GmbH' WHERE id = ?1")
            .bind(manufacturer_id)
            .execute(&mut *conn)
            .await
            .unwrap();
        rebuild_search_index(model_id, &mut conn).await.unwrap();
        drop(conn);

        seed_collection(&pool, "col-item-mfr-001", model_id).await;

        let mut uow = SqliteUnitOfWork::new(&pool).await.unwrap();
        let mut repo = uow.global_search_repo();
        let results = repo.search("\"SearchBrand\"*", "en").await.unwrap();

        assert!(
            !results.is_empty(),
            "should find result by manufacturer name"
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_return_empty_when_no_collection_or_wishlist_item(pool: SqlitePool) {
        let model_id = "trn:railway-model:noitem:nocol-001";
        let manufacturer_id = "trn:manufacturer:noitem";

        seed_test_data(&pool, model_id, manufacturer_id).await;
        let mut conn = pool.acquire().await.unwrap();
        rebuild_search_index(model_id, &mut conn).await.unwrap();
        drop(conn);

        let mut uow = SqliteUnitOfWork::new(&pool).await.unwrap();
        let mut repo = uow.global_search_repo();
        // Model exists in FTS5 but has no collection/wishlist entry
        let results = repo.search("\"Test locomotive\"*", "en").await.unwrap();

        let matches_this_model = results
            .iter()
            .any(|r| r.railway_model_id.to_string() == model_id);
        assert!(
            !matches_this_model,
            "model without collection/wishlist should not appear"
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_exclude_removed_collection_items(pool: SqlitePool) {
        let model_id = "trn:railway-model:removedbrand:rmv-001";
        let manufacturer_id = "trn:manufacturer:removedbrand";

        seed_test_data(&pool, model_id, manufacturer_id).await;
        seed_collection(&pool, "col-item-rmv-001", model_id).await;

        // Mark item as removed
        let mut conn = pool.acquire().await.unwrap();
        sqlx::query("UPDATE collection_items SET removed_date = '2024-12-01' WHERE id = ?1")
            .bind("col-item-rmv-001")
            .execute(&mut *conn)
            .await
            .unwrap();
        rebuild_search_index(model_id, &mut conn).await.unwrap();
        drop(conn);

        let mut uow = SqliteUnitOfWork::new(&pool).await.unwrap();
        let mut repo = uow.global_search_repo();
        let results = repo.search("\"Test locomotive\"*", "en").await.unwrap();

        let found_removed = results.iter().any(|r| r.item_id == "col-item-rmv-001");
        assert!(
            !found_removed,
            "removed collection items should not appear in results"
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_emit_two_results_when_in_both_collection_and_wishlist(pool: SqlitePool) {
        let model_id = "trn:railway-model:bothbrand:both-001";
        let manufacturer_id = "trn:manufacturer:bothbrand";

        seed_test_data(&pool, model_id, manufacturer_id).await;
        seed_collection(&pool, "col-item-both-001", model_id).await;
        seed_wishlist_and_item(&pool, "wishlist-both-001", "wi-item-both-001", model_id).await;

        let mut conn = pool.acquire().await.unwrap();
        rebuild_search_index(model_id, &mut conn).await.unwrap();
        drop(conn);

        let mut uow = SqliteUnitOfWork::new(&pool).await.unwrap();
        let mut repo = uow.global_search_repo();
        let results = repo.search("\"Test locomotive\"*", "en").await.unwrap();

        let col = results.iter().any(|r| r.source == SearchSource::Collection);
        let wl = results.iter().any(|r| r.source == SearchSource::Wishlist);
        assert!(col, "should have a Collection result");
        assert!(wl, "should have a Wishlist result");
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_match_by_road_number(pool: SqlitePool) {
        let model_id = "trn:railway-model:trenibrand:road-001";
        let manufacturer_id = "trn:manufacturer:trenibrand";

        seed_test_data(&pool, model_id, manufacturer_id).await;
        seed_rolling_stock(&pool, "rs-road-001", model_id, "E636.005").await;
        seed_collection(&pool, "col-item-road-001", model_id).await;

        let mut conn = pool.acquire().await.unwrap();
        rebuild_search_index(model_id, &mut conn).await.unwrap();
        drop(conn);

        let mut uow = SqliteUnitOfWork::new(&pool).await.unwrap();
        let mut repo = uow.global_search_repo();
        let results = repo.search("\"E636\"*", "en").await.unwrap();

        let found = results.iter().any(|r| r.item_id == "col-item-road-001");
        assert!(
            found,
            "should find item by road number in rolling_stocks_text"
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn it_should_remove_fts5_rows_when_model_is_deleted(pool: SqlitePool) {
        let model_id = "trn:railway-model:delbrand:del-001";
        let manufacturer_id = "trn:manufacturer:delbrand";

        seed_test_data(&pool, model_id, manufacturer_id).await;
        seed_collection(&pool, "col-item-del-001", model_id).await;

        let mut conn = pool.acquire().await.unwrap();
        rebuild_search_index(model_id, &mut conn).await.unwrap();

        // Simulate deletion: remove FTS5 rows before deleting the model
        sqlx::query("DELETE FROM railway_model_search_idx WHERE railway_model_id = ?1")
            .bind(model_id)
            .execute(&mut *conn)
            .await
            .unwrap();
        drop(conn);

        let mut uow = SqliteUnitOfWork::new(&pool).await.unwrap();
        let mut repo = uow.global_search_repo();
        let results = repo.search("\"Test locomotive\"*", "en").await.unwrap();

        let found = results
            .iter()
            .any(|r| r.railway_model_id.to_string() == model_id);
        assert!(!found, "deleted model should not appear in search results");
    }
}
