//! Raw SQL operations for the tracks inventory feature.
//!
//! Every function accepts a `&mut sqlx::SqliteConnection` (or `&mut`
//! re-borrow of a transaction) and returns `Result<_, sqlx::Error>`.
//! No domain business-logic lives here — only parameterised queries and
//! row-level types from [`super::entities`].

use crate::tracks_inventory::domain::{TrackCode, TrackId, TrackInventoryId, TrackType};
use crate::tracks_inventory::infrastructure::entities::{
    TrackInventoryHeaderViewRow, TrackInventoryItemRow, TrackInventoryItemViewRow,
    TrackInventoryRow, TrackInventorySummaryRow, TrackProductRow, TrackProductViewRow,
    TrackPurchaseRow, TrackPurchaseViewRow,
};

// ---------------------------------------------------------------------------
// Track inventory – header
// ---------------------------------------------------------------------------

/// Fetches the inventory header row for the given `id`.
///
/// Returns `Ok(None)` when no matching row exists.
pub async fn find_track_inventory_by_id(
    executor: &mut sqlx::SqliteConnection,
    id: &TrackInventoryId,
) -> Result<Option<TrackInventoryRow>, sqlx::Error> {
    let sql = r#"
        SELECT id, created_at, updated_at, version, name, description
        FROM track_inventories
        WHERE id = ?1
        LIMIT 1
    "#;
    sqlx::query_as(sql)
        .bind(id)
        .fetch_optional(executor)
        .await
}

/// Inserts a new track inventory header row.
///
/// Uses `INSERT OR REPLACE` so a re-run of the same ID is idempotent.
pub async fn insert_track_inventory(
    executor: &mut sqlx::SqliteConnection,
    id: &TrackInventoryId,
    name: &str,
    description: Option<&str>,
) -> Result<(), sqlx::Error> {
    let sql = r#"
        INSERT OR REPLACE INTO track_inventories
            (id, created_at, updated_at, version, name, description)
        VALUES (?1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 0, ?2, ?3)
    "#;
    sqlx::query(sql)
        .bind(id.to_string())
        .bind(name)
        .bind(description)
        .execute(executor)
        .await?;
    Ok(())
}

/// Renames a track inventory and bumps its `updated_at` timestamp.
///
/// Returns the number of affected rows (0 when the ID does not exist).
pub async fn rename_track_inventory(
    executor: &mut sqlx::SqliteConnection,
    id: &TrackInventoryId,
    name: &str,
) -> Result<u64, sqlx::Error> {
    let sql = r#"
        UPDATE track_inventories
        SET name = ?1, updated_at = CURRENT_TIMESTAMP
        WHERE id = ?2
    "#;
    let res = sqlx::query(sql)
        .bind(name)
        .bind(id.to_string())
        .execute(executor)
        .await?;
    Ok(res.rows_affected())
}

/// Updates the description of a track inventory.
///
/// Returns the number of affected rows (0 when the ID does not exist).
pub async fn update_track_inventory_description(
    executor: &mut sqlx::SqliteConnection,
    id: &TrackInventoryId,
    description: Option<&str>,
) -> Result<u64, sqlx::Error> {
    let sql = r#"
        UPDATE track_inventories
        SET description = ?1, updated_at = CURRENT_TIMESTAMP
        WHERE id = ?2
    "#;
    let res = sqlx::query(sql)
        .bind(description)
        .bind(id.to_string())
        .execute(executor)
        .await?;
    Ok(res.rows_affected())
}

/// Deletes a track inventory by its `id`.
///
/// Cascading deletes on `track_inventory_items` and `track_purchases` are
/// expected to be enforced by database foreign-key constraints.
pub async fn delete_track_inventory(
    executor: &mut sqlx::SqliteConnection,
    id: &TrackInventoryId,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM track_inventories WHERE id = ?1")
        .bind(id)
        .execute(executor)
        .await?;
    Ok(())
}

/// Touches the `updated_at` timestamp of a track inventory without altering
/// any other columns.
pub async fn touch_inventory_updated_at(
    executor: &mut sqlx::SqliteConnection,
    id: &TrackInventoryId,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE track_inventories SET updated_at = CURRENT_TIMESTAMP WHERE id = ?1",
    )
    .bind(id.to_string())
    .execute(executor)
    .await?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Track inventory – items
// ---------------------------------------------------------------------------

/// Fetches all inventory item rows for the given `inventory_id`.
pub async fn find_track_inventory_items(
    executor: &mut sqlx::SqliteConnection,
    inventory_id: &TrackInventoryId,
) -> Result<Vec<TrackInventoryItemRow>, sqlx::Error> {
    let sql = r#"
        SELECT track_id, quantity, required
        FROM track_inventory_items
        WHERE inventory_id = ?1
    "#;
    sqlx::query_as(sql)
        .bind(inventory_id)
        .fetch_all(executor)
        .await
}

/// Removes the inventory item row for `(inventory_id, track_id)`.
pub async fn delete_track_inventory_item(
    executor: &mut sqlx::SqliteConnection,
    inventory_id: &TrackInventoryId,
    track_id: &TrackId,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "DELETE FROM track_inventory_items WHERE inventory_id = ?1 AND track_id = ?2",
    )
    .bind(inventory_id.to_string())
    .bind(track_id.to_string())
    .execute(executor)
    .await?;
    Ok(())
}

/// Upserts an inventory item with an explicit `quantity`.
///
/// Uses `INSERT OR REPLACE` – note that this resets the `required` column to
/// its default value; use this only when you intend to overwrite the full row.
pub async fn upsert_track_inventory_item(
    executor: &mut sqlx::SqliteConnection,
    inventory_id: &TrackInventoryId,
    track_id: &TrackId,
    quantity: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT OR REPLACE INTO track_inventory_items (inventory_id, track_id, quantity) VALUES (?1, ?2, ?3)",
    )
    .bind(inventory_id.to_string())
    .bind(track_id.to_string())
    .bind(quantity)
    .execute(executor)
    .await?;
    Ok(())
}

/// Increments the inventory item quantity by `delta`.
///
/// Inserts a new row if none exists, or adds `delta` to the existing
/// `quantity` on conflict, preserving the `required` column.
pub async fn increment_inventory_item_quantity(
    executor: &mut sqlx::SqliteConnection,
    inventory_id: &TrackInventoryId,
    track_id: &TrackId,
    delta: i64,
) -> Result<(), sqlx::Error> {
    let sql = r#"
        INSERT INTO track_inventory_items (inventory_id, track_id, quantity)
        VALUES (?1, ?2, ?3)
        ON CONFLICT (inventory_id, track_id)
        DO UPDATE SET quantity = quantity + excluded.quantity
    "#;
    sqlx::query(sql)
        .bind(inventory_id.to_string())
        .bind(track_id.to_string())
        .bind(delta)
        .execute(executor)
        .await?;
    Ok(())
}

/// Updates the `required` quantity for a specific `(inventory_id, track_id)` item.
///
/// Returns the number of rows updated (0 if the item does not exist).
pub async fn set_inventory_item_required(
    executor: &mut sqlx::SqliteConnection,
    inventory_id: &TrackInventoryId,
    track_id: &TrackId,
    required: i64,
) -> Result<u64, sqlx::Error> {
    let sql = r#"
        UPDATE track_inventory_items
        SET required = ?1
        WHERE inventory_id = ?2 AND track_id = ?3
    "#;
    let res = sqlx::query(sql)
        .bind(required)
        .bind(inventory_id)
        .bind(track_id)
        .execute(executor)
        .await?;
    Ok(res.rows_affected())
}

// ---------------------------------------------------------------------------
// Track purchases
// ---------------------------------------------------------------------------

/// Fetches all purchase rows for the given `inventory_id`, ordered by date.
pub async fn find_track_purchases(
    executor: &mut sqlx::SqliteConnection,
    inventory_id: &TrackInventoryId,
) -> Result<Vec<TrackPurchaseRow>, sqlx::Error> {
    let sql = r#"
        SELECT id, track_id, quantity, price_amount, price_currency, seller_id, purchase_date
        FROM track_purchases
        WHERE inventory_id = ?1
        ORDER BY purchase_date ASC
    "#;
    sqlx::query_as(sql)
        .bind(inventory_id)
        .fetch_all(executor)
        .await
}

/// Inserts a purchase record.
///
/// Uses `INSERT OR REPLACE` to make the operation idempotent on repeated
/// runs with the same `purchase_id`.
#[allow(clippy::too_many_arguments)]
pub async fn insert_track_purchase(
    executor: &mut sqlx::SqliteConnection,
    inventory_id: &TrackInventoryId,
    purchase_id: &str,
    track_id: &TrackId,
    quantity: i64,
    price_amount: i64,
    price_currency: &str,
    seller_id: Option<String>,
    purchase_date: &str,
) -> Result<(), sqlx::Error> {
    let sql = r#"
        INSERT OR REPLACE INTO track_purchases (
            id, inventory_id, track_id, quantity, price_amount,
            price_currency, seller_id, purchase_date, created_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, CURRENT_TIMESTAMP)
    "#;
    sqlx::query(sql)
        .bind(purchase_id)
        .bind(inventory_id.to_string())
        .bind(track_id.to_string())
        .bind(quantity)
        .bind(price_amount)
        .bind(price_currency)
        .bind(seller_id)
        .bind(purchase_date)
        .execute(executor)
        .await?;
    Ok(())
}

// ---------------------------------------------------------------------------
// View / read-model queries
// ---------------------------------------------------------------------------

/// Fetches summary rows for all inventories (item count + total quantity).
pub async fn find_all_inventory_summaries(
    executor: &mut sqlx::SqliteConnection,
) -> Result<Vec<TrackInventorySummaryRow>, sqlx::Error> {
    let sql = r#"
        SELECT
            ti.id,
            ti.name,
            ti.description,
            COUNT(DISTINCT tii.track_id) as total_items,
            COALESCE(SUM(tii.quantity), 0) as total_quantity
        FROM track_inventories ti
        LEFT JOIN track_inventory_items tii ON ti.id = tii.inventory_id
        GROUP BY ti.id, ti.name, ti.description
        ORDER BY ti.created_at DESC
    "#;
    sqlx::query_as(sql).fetch_all(executor).await
}

/// Fetches the lightweight header view row for the given `id`.
///
/// Returns `Ok(None)` when no matching row exists.
pub async fn find_inventory_header_view(
    executor: &mut sqlx::SqliteConnection,
    id: &TrackInventoryId,
) -> Result<Option<TrackInventoryHeaderViewRow>, sqlx::Error> {
    let sql = r#"
        SELECT id, name, description
        FROM track_inventories
        WHERE id = ?1
    "#;
    sqlx::query_as(sql)
        .bind(id)
        .fetch_optional(executor)
        .await
}

/// Fetches item detail rows for the view of a single inventory.
///
/// Each row is joined with the `track_products` and `manufacturers` tables to
/// provide the display fields expected by [`TrackInventoryItemViewRow`].
pub async fn find_inventory_item_views(
    executor: &mut sqlx::SqliteConnection,
    inventory_id: &TrackInventoryId,
) -> Result<Vec<TrackInventoryItemViewRow>, sqlx::Error> {
    let sql = r#"
        SELECT
            tii.track_id,
            tii.quantity,
            tii.required,
            tp.product_code,
            tp.description,
            tp.track_type,
            tp.track_code,
            tp.with_roadbed,
            tp.length_mm,
            tp.radius_mm,
            m.name as manufacturer_name
        FROM track_inventory_items tii
        INNER JOIN track_products tp ON tii.track_id = tp.track_id
        INNER JOIN manufacturers m ON tp.manufacturer_id = m.id
        WHERE tii.inventory_id = ?1
        ORDER BY tp.product_code
    "#;
    sqlx::query_as(sql)
        .bind(inventory_id)
        .fetch_all(executor)
        .await
}

/// Fetches purchase history rows for the view of a single inventory.
///
/// Each row is joined with `track_products`, `manufacturers`, and optionally
/// `sellers` to provide the display fields in [`TrackPurchaseViewRow`].
pub async fn find_inventory_purchase_views(
    executor: &mut sqlx::SqliteConnection,
    inventory_id: &TrackInventoryId,
) -> Result<Vec<TrackPurchaseViewRow>, sqlx::Error> {
    let sql = r#"
        SELECT
            tp_hist.id,
            tp_hist.track_id,
            tp_hist.quantity,
            tp_hist.price_amount,
            tp_hist.price_currency,
            tp_hist.purchase_date,
            s.name as seller_name,
            tp.product_code,
            tp.description,
            tp.track_type,
            tp.track_code,
            tp.with_roadbed,
            tp.length_mm,
            tp.radius_mm,
            m.name as manufacturer_name
        FROM track_purchases tp_hist
        INNER JOIN track_products tp ON tp_hist.track_id = tp.track_id
        INNER JOIN manufacturers m ON tp.manufacturer_id = m.id
        LEFT JOIN sellers s ON tp_hist.seller_id = s.id
        WHERE tp_hist.inventory_id = ?1
        ORDER BY tp_hist.purchase_date DESC
    "#;
    sqlx::query_as(sql)
        .bind(inventory_id)
        .fetch_all(executor)
        .await
}

// ---------------------------------------------------------------------------
// Track products
// ---------------------------------------------------------------------------

/// Fetches a track product row by its canonical `TrackId`.
///
/// Returns `Ok(None)` when no matching row exists.
pub async fn find_track_product_by_id(
    executor: &mut sqlx::SqliteConnection,
    id: &TrackId,
) -> Result<Option<TrackProductRow>, sqlx::Error> {
    let sql = r#"
        SELECT track_id, product_code, manufacturer_id, with_roadbed,
               length_mm, radius_mm, track_code, track_type, description
        FROM track_products
        WHERE track_id = ?1
        LIMIT 1
    "#;
    sqlx::query_as(sql)
        .bind(id)
        .fetch_optional(executor)
        .await
}

/// Fetches a track product row by manufacturer ID and product code.
///
/// Returns `Ok(None)` when no matching row exists.
pub async fn find_track_product_by_code(
    executor: &mut sqlx::SqliteConnection,
    manufacturer_id: &str,
    product_code: &str,
) -> Result<Option<TrackProductRow>, sqlx::Error> {
    let sql = r#"
        SELECT track_id, product_code, manufacturer_id, with_roadbed,
               length_mm, radius_mm, track_code, track_type, description
        FROM track_products
        WHERE manufacturer_id = ?1 AND product_code = ?2
        LIMIT 1
    "#;
    sqlx::query_as(sql)
        .bind(manufacturer_id)
        .bind(product_code)
        .fetch_optional(executor)
        .await
}

/// Fetches all track product view rows joined with the manufacturer name.
pub async fn find_all_product_views(
    executor: &mut sqlx::SqliteConnection,
) -> Result<Vec<TrackProductViewRow>, sqlx::Error> {
    let sql = r#"
        SELECT
            tp.track_id,
            tp.product_code,
            tp.description,
            tp.track_type,
            tp.track_code,
            tp.with_roadbed,
            tp.length_mm,
            tp.radius_mm,
            m.name as manufacturer_name
        FROM track_products tp
        INNER JOIN manufacturers m ON tp.manufacturer_id = m.id
        ORDER BY m.name, tp.product_code
    "#;
    sqlx::query_as(sql).fetch_all(executor).await
}

/// Upserts a track product record.
///
/// The `id` column is set to the same value as `track_id`; `created_at`,
/// `updated_at`, and `version` are managed by the database.
#[allow(clippy::too_many_arguments)]
pub async fn upsert_track_product(
    executor: &mut sqlx::SqliteConnection,
    track_id: &TrackId,
    manufacturer_id: &str,
    product_code: &str,
    with_roadbed: i64,
    length_mm: Option<i32>,
    radius_mm: Option<i32>,
    track_code: TrackCode,
    track_type: TrackType,
) -> Result<(), sqlx::Error> {
    let sql = r#"
        INSERT OR REPLACE INTO track_products (
            id, track_id, manufacturer_id, product_code, with_roadbed,
            length_mm, radius_mm, track_code, track_type,
            created_at, updated_at, version)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 0)
    "#;
    sqlx::query(sql)
        .bind(track_id)
        .bind(track_id)
        .bind(manufacturer_id)
        .bind(product_code)
        .bind(with_roadbed)
        .bind(length_mm)
        .bind(radius_mm)
        .bind(track_code)
        .bind(track_type)
        .execute(executor)
        .await?;
    Ok(())
}
