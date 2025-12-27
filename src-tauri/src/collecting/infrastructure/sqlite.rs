//! SQLite helper functions (crate-internal) used to read collecting-related rows.
//!
//! These helpers return typed row representations defined in
//! `crate::collecting::infrastructure::entities` and intentionally keep SQL and
//! mapping logic separate from domain conversion. All queries use parameter
//! binding via `sqlx::query_as(...).bind(...)` to avoid string interpolation.

use anyhow::{Context, Result};
use sqlx::SqlitePool;

use crate::collecting::infrastructure::entities::{
    CollectionItemRow, CollectionRow, OwnedRollingStockRow, PurchaseInfoRow,
};

use crate::collecting::domain::collection_id::CollectionId;
use crate::collecting::domain::collection_item_id::CollectionItemId;

/// Fetch a single collection row by id.
///
/// Parameters:
/// - `pool`: SQLite connection pool (moved/cloned by caller).
/// - `collection_id`: domain newtype identifying the collection. The function
///   binds the string form of the id to the SQL query.
///
/// Returns `Ok(Some(CollectionRow))` if found, `Ok(None)` if not found, or an
/// `Err` on database errors.
pub async fn get_collection(
    pool: &SqlitePool,
    collection_id: CollectionId,
) -> Result<Option<CollectionRow>> {
    let sql = "SELECT id, name, locomotives_count, passenger_cars_count, freight_cars_count, train_sets_count, railcars_count, electric_multiple_units_count, total_value_amount, total_value_currency, created_at, updated_at FROM collections WHERE id = ?1 LIMIT 1";

    let row = sqlx::query_as::<_, CollectionRow>(sql)
        .bind(collection_id.to_string())
        .fetch_optional(pool)
        .await
        .with_context(|| format!("querying collection id={}", collection_id))?;

    Ok(row)
}

/// Fetch a single collection item row by its id.
///
/// Binds the `collection_item_id` string form to the query and returns the
/// corresponding `CollectionItemRow` if present.
pub async fn get_collection_item(
    pool: &SqlitePool,
    collection_item_id: CollectionItemId,
) -> Result<Option<CollectionItemRow>> {
    let sql = "SELECT id, collection_id, railway_model_id, conditions, notes FROM collection_items WHERE id = ?1 LIMIT 1";

    let row = sqlx::query_as::<_, CollectionItemRow>(sql)
        .bind(collection_item_id.to_string())
        .fetch_optional(pool)
        .await
        .with_context(|| format!("querying collection_item id={}", collection_item_id))?;

    Ok(row)
}

/// Fetch all collection items belonging to a collection.
///
/// Returns a vector of `CollectionItemRow`. The `collection_id` is bound as a
/// parameter to the query to avoid string concatenation.
pub async fn get_collection_items(
    pool: &SqlitePool,
    collection_id: &CollectionId,
) -> Result<Vec<CollectionItemRow>> {
    let sql = "SELECT id, collection_id, railway_model_id, conditions, notes FROM collection_items WHERE collection_id = ?1";

    let rows = sqlx::query_as::<_, CollectionItemRow>(sql)
        .bind(collection_id.to_string())
        .fetch_all(pool)
        .await
        .with_context(|| {
            format!(
                "querying collection_items for collection_id={}",
                collection_id
            )
        })?;

    Ok(rows)
}

/// Fetch a single owned rolling stock row by id.
///
/// The function accepts the raw owned rolling stock id string and returns the
/// matching `OwnedRollingStockRow` if present.
pub async fn get_owned_rolling_stock(
    pool: &SqlitePool,
    owned_rolling_stock_id: String,
) -> Result<Option<OwnedRollingStockRow>> {
    let sql = "SELECT id, collection_item_id, rolling_stock_id, notes FROM owned_rolling_stocks WHERE id = ?1 LIMIT 1";

    let row = sqlx::query_as::<_, OwnedRollingStockRow>(sql)
        .bind(owned_rolling_stock_id)
        .fetch_optional(pool)
        .await
        .context("querying owned_rolling_stock by id")?;

    Ok(row)
}

/// Fetch all owned rolling stocks that belong to a collection.
///
/// This performs a join from `owned_rolling_stocks` to `collection_items` and
/// filters by `collection_items.collection_id = ?` using parameter binding.
pub async fn get_owned_rolling_stocks(
    pool: &SqlitePool,
    collection_id: &CollectionId,
) -> Result<Vec<OwnedRollingStockRow>> {
    let sql = "SELECT ors.id, ors.collection_item_id, ors.rolling_stock_id, ors.notes FROM owned_rolling_stocks AS ors JOIN collection_items AS ci ON ci.id = ors.collection_item_id WHERE ci.collection_id = ?1";

    let rows = sqlx::query_as::<_, OwnedRollingStockRow>(sql)
        .bind(collection_id.to_string())
        .fetch_all(pool)
        .await
        .with_context(|| {
            format!(
                "querying owned_rolling_stocks for collection_id={}",
                collection_id
            )
        })?;

    Ok(rows)
}

/// Fetch a single purchase_info by id.
///
/// Accepts the raw purchase info id string and returns the typed
/// `PurchaseInfoRow` if present.
pub async fn get_purchase_info(
    pool: &SqlitePool,
    purchase_info_id: String,
) -> Result<Option<PurchaseInfoRow>> {
    let sql = "SELECT purchase_id, collection_item_id, purchase_type, purchase_date, seller_id, buyer_id, sale_date, purchased_price_amount, purchased_price_currency, sale_price_amount, sale_price_currency, deposit_amount, deposit_currency, preorder_total_amount, preorder_total_currency, expected_date FROM purchase_infos WHERE purchase_id = ?1 LIMIT 1";

    let row = sqlx::query_as::<_, PurchaseInfoRow>(sql)
        .bind(purchase_info_id)
        .fetch_optional(pool)
        .await
        .context("querying purchase_info by id")?;

    Ok(row)
}

/// Fetch all purchase infos associated to a collection (via collection_items).
///
/// Joins `purchase_infos` to `collection_items` and binds the collection id
/// parameter to prevent SQL injection.
pub async fn get_purchase_infos(
    pool: &SqlitePool,
    collection_id: &CollectionId,
) -> Result<Vec<PurchaseInfoRow>> {
    let sql = "SELECT pi.purchase_id, pi.collection_item_id, pi.purchase_type, pi.purchase_date, pi.seller_id, pi.buyer_id, pi.sale_date, pi.purchased_price_amount, pi.purchased_price_currency, pi.sale_price_amount, pi.sale_price_currency, pi.deposit_amount, pi.deposit_currency, pi.preorder_total_amount, pi.preorder_total_currency, pi.expected_date FROM purchase_infos pi JOIN collection_items ci ON ci.id = pi.collection_item_id WHERE ci.collection_id = ?1";

    let rows = sqlx::query_as::<_, PurchaseInfoRow>(sql)
        .bind(collection_id.to_string())
        .fetch_all(pool)
        .await
        .with_context(|| {
            format!(
                "querying purchase_infos for collection_id={}",
                collection_id
            )
        })?;

    Ok(rows)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use sqlx::SqlitePool;

    use crate::catalog::infrastructure::testing::CatalogTestDb;
    use crate::collecting::infrastructure::sqlite::*;
    use crate::collecting::infrastructure::testing::CollectingTestDb;

    #[sqlx::test(migrations = "./migrations")]
    async fn get_collection_returns_row(pool: SqlitePool) -> Result<()> {
        let catalog_db = CatalogTestDb::new(pool.clone());
        let catalog_test_data = catalog_db.setup_railway_model().await?;

        let collecting_db = CollectingTestDb::new(pool.clone());
        let data = collecting_db
            .setup_minimal_collection(
                &catalog_test_data.railway_model_id,
                catalog_test_data
                    .rolling_stock_ids
                    .iter()
                    .map(|s| s.as_str())
                    .collect(),
            )
            .await?;

        let collection_id = CollectionId::try_from(data.collection_id.as_str())?;

        let collection = get_collection(&pool, collection_id.clone()).await?;
        assert!(collection.is_some());
        let collection = collection.unwrap();
        assert_eq!(collection.id, collection_id.to_string());

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn get_collection_item_and_items_and_owned_rs_and_purchase_info(
        pool: SqlitePool,
    ) -> Result<()> {
        let catalog_db = CatalogTestDb::new(pool.clone());
        let catalog_test_data = catalog_db.setup_railway_model().await?;

        let collecting_db = CollectingTestDb::new(pool.clone());
        let data = collecting_db
            .setup_minimal_collection(
                &catalog_test_data.railway_model_id,
                catalog_test_data
                    .rolling_stock_ids
                    .iter()
                    .map(|s| s.as_str())
                    .collect(),
            )
            .await?;

        let collection_id = CollectionId::try_from(data.collection_id.as_str())?;
        let collection_item_id = CollectionItemId::try_from(data.collection_item_id.as_str())?;

        // collection items
        let items = get_collection_items(&pool, &collection_id).await?;
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, collection_item_id.to_string());

        let item = get_collection_item(&pool, collection_item_id.clone()).await?;
        assert!(item.is_some());
        let item = item.unwrap();
        assert_eq!(item.collection_id, collection_id.to_string());

        // owned rolling stocks
        let ors = get_owned_rolling_stocks(&pool, &collection_id).await?;
        assert_eq!(ors.len(), data.owned_rolling_stock_ids.len());

        let first_owned_id = data.owned_rolling_stock_ids.get(0).unwrap().clone();
        let ors_row = get_owned_rolling_stock(&pool, first_owned_id.clone()).await?;
        assert!(ors_row.is_some());
        let ors_row = ors_row.unwrap();
        assert_eq!(ors_row.id, first_owned_id);

        // purchase infos
        let pis = get_purchase_infos(&pool, &collection_id).await?;
        assert_eq!(pis.len(), 1);
        assert_eq!(pis[0].collection_item_id, collection_item_id.to_string());

        let pi = get_purchase_info(&pool, data.purchase_info_id.clone()).await?;
        assert!(pi.is_some());
        let pi = pi.unwrap();
        assert_eq!(pi.purchase_id, data.purchase_info_id);

        Ok(())
    }
}
