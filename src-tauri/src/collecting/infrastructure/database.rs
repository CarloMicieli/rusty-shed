//! SQLite helper functions (crate-internal) used to read collecting-related rows.
//!
//! These helpers return typed row representations defined in
//! `crate::collecting::infrastructure::entities` and intentionally keep SQL and
//! mapping logic separate from domain conversion. All queries use parameter
//! binding via `sqlx::query_as(...).bind(...)` to avoid string interpolation.

use anyhow::{Context, Result};

use crate::collecting::infrastructure::entities::{
    CollectionItemRow, CollectionRow, OwnedRollingStockRow, PurchaseInfoRow,
};

use crate::collecting::domain::collection_id::CollectionId;

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
    executor: &mut sqlx::SqliteConnection,
    collection_id: &CollectionId,
) -> Result<Option<CollectionRow>> {
    let sql = "SELECT id, name, locomotives_count, passenger_cars_count, freight_cars_count, train_sets_count, railcars_count, electric_multiple_units_count, total_value_amount, total_value_currency, created_at, updated_at FROM collections WHERE id = ?1 LIMIT 1";

    let row = sqlx::query_as::<_, CollectionRow>(sql)
        .bind(collection_id.to_string())
        .fetch_optional(executor)
        .await
        .with_context(|| format!("querying collection id={}", collection_id))?;

    Ok(row)
}

/// Fetch all collection items belonging to a collection.
///
/// Returns a vector of `CollectionItemRow`. The `collection_id` is bound as a
/// parameter to the query to avoid string concatenation.
pub async fn get_collection_items(
    executor: &mut sqlx::SqliteConnection,
    collection_id: &CollectionId,
) -> Result<Vec<CollectionItemRow>> {
    let sql = "SELECT id, collection_id, railway_model_id, conditions, notes FROM collection_items WHERE collection_id = ?1";

    let rows = sqlx::query_as::<_, CollectionItemRow>(sql)
        .bind(collection_id.to_string())
        .fetch_all(executor)
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
    executor: &mut sqlx::SqliteConnection,
    owned_rolling_stock_id: String,
) -> Result<Option<OwnedRollingStockRow>> {
    let sql = "SELECT id, collection_item_id, rolling_stock_id, notes FROM owned_rolling_stocks WHERE id = ?1 LIMIT 1";

    let row = sqlx::query_as::<_, OwnedRollingStockRow>(sql)
        .bind(owned_rolling_stock_id)
        .fetch_optional(executor)
        .await
        .context("querying owned_rolling_stock by id")?;

    Ok(row)
}

/// Fetch all owned rolling stocks that belong to a collection.
///
/// This performs a join from `owned_rolling_stocks` to `collection_items` and
/// filters by `collection_items.collection_id = ?` using parameter binding.
pub async fn get_owned_rolling_stocks(
    executor: &mut sqlx::SqliteConnection,
    collection_id: &CollectionId,
) -> Result<Vec<OwnedRollingStockRow>> {
    let sql = "SELECT ors.id, ors.collection_item_id, ors.rolling_stock_id, ors.notes FROM owned_rolling_stocks AS ors JOIN collection_items AS ci ON ci.id = ors.collection_item_id WHERE ci.collection_id = ?1";

    let rows = sqlx::query_as::<_, OwnedRollingStockRow>(sql)
        .bind(collection_id.to_string())
        .fetch_all(executor)
        .await
        .with_context(|| {
            format!(
                "querying owned_rolling_stocks for collection_id={}",
                collection_id
            )
        })?;

    Ok(rows)
}

/// Fetch all purchase infos associated to a collection (via collection_items).
///
/// Joins `purchase_infos` to `collection_items` and binds the collection id
/// parameter to prevent SQL injection.
pub async fn get_purchase_infos(
    executor: &mut sqlx::SqliteConnection,
    collection_id: &CollectionId,
) -> Result<Vec<PurchaseInfoRow>> {
    let sql = "SELECT pi.purchase_id, pi.collection_item_id, pi.purchase_type, pi.purchase_date, pi.seller_id, pi.buyer_id, pi.sale_date, pi.purchased_price_amount, pi.purchased_price_currency, pi.sale_price_amount, pi.sale_price_currency, pi.deposit_amount, pi.deposit_currency, pi.preorder_total_amount, pi.preorder_total_currency, pi.expected_date FROM purchase_infos pi JOIN collection_items ci ON ci.id = pi.collection_item_id WHERE ci.collection_id = ?1";

    let rows = sqlx::query_as::<_, PurchaseInfoRow>(sql)
        .bind(collection_id.to_string())
        .fetch_all(executor)
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
    use crate::collecting::domain::collection_id::CollectionId;
    use pretty_assertions::assert_eq;
    use sqlx::Sqlite;
    use sqlx::pool::PoolConnection;

    #[sqlx::test(migrations = "./migrations")]
    async fn get_collection_returns_none(mut conn: PoolConnection<Sqlite>) -> Result<()> {
        let result = super::get_collection(&mut conn, &CollectionId::default()).await?;

        assert!(result.is_none());

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations", fixtures("test_collection"))]
    async fn get_collection_returns_some(mut conn: PoolConnection<Sqlite>) -> Result<()> {
        let result = super::get_collection(&mut conn, &CollectionId::default()).await?;

        assert!(result.is_some());

        let collection = result.unwrap();
        assert_eq!(collection.id, CollectionId::default().to_string());
        // Fixture sets the collection name to 'Test Collection'
        assert_eq!(collection.name, "Test Collection");

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations", fixtures("test_collection"))]
    async fn get_collection_items(mut conn: PoolConnection<Sqlite>) -> Result<()> {
        let result = super::get_collection_items(&mut conn, &CollectionId::default()).await?;

        assert_eq!(result.len(), 1);

        let item = &result[0];
        assert_eq!(item.id, "d20a1a95-1ae4-4970-9e87-b4c84676e730");
        assert_eq!(item.collection_id, CollectionId::default().to_string());
        assert_eq!(item.railway_model_id, "trn:railway-model:acme:60100");
        // `conditions` and `notes` are Option<String> in the row mapping
        assert_eq!(item.conditions, Some("new".to_string()));
        assert_eq!(item.notes, Some("My notes go here".to_string()));

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations", fixtures("test_collection"))]
    async fn get_owned_rolling_stocks(mut conn: PoolConnection<Sqlite>) -> Result<()> {
        let result = super::get_owned_rolling_stocks(&mut conn, &CollectionId::default()).await?;

        assert_eq!(result.len(), 1);

        let ors = &result[0];
        assert_eq!(ors.id, "d3606635-4c4e-462b-ae9f-02c7ce47bc770");
        assert_eq!(ors.collection_item_id, "d20a1a95-1ae4-4970-9e87-b4c84676e730");
        // rolling_stock_id and notes are optional in the entity mapping
        assert_eq!(ors.rolling_stock_id, Some("rs-001".to_string()));
        assert_eq!(ors.notes, Some("My rolling stock notes go here".to_string()));

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations", fixtures("test_collection"))]
    async fn get_purchase_infos(mut conn: PoolConnection<Sqlite>) -> Result<()> {
        let result = super::get_purchase_infos(&mut conn, &CollectionId::default()).await?;

        assert_eq!(result.len(), 1);

        let pi = &result[0];
        assert_eq!(pi.purchase_id, "59adc26d-0274-4d6b-8c14-61e598d3fe0e");
        assert_eq!(pi.collection_item_id, "d20a1a95-1ae4-4970-9e87-b4c84676e730");
        assert_eq!(pi.purchase_type, Some("purchased".to_string()));
        // purchase_date is a NaiveDate; compare its string form to the fixture date
        assert_eq!(pi.purchase_date.to_string(), "2025-12-26");
        assert_eq!(pi.purchased_price_amount, Some(17500));
        assert_eq!(pi.purchased_price_currency, Some("EUR".to_string()));

        Ok(())
    }
}
