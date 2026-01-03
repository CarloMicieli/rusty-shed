use crate::wishlist::domain::wishlist_id::WishlistId;
use crate::wishlist::infrastructure::entities::{
    WishlistItemRow, WishlistPreviewProjection, WishlistRow,
};
use anyhow::{Context, Result};

pub async fn find_wishlist_by_id(
    executor: &mut sqlx::SqliteConnection,
    id: &WishlistId,
) -> Result<Option<WishlistRow>> {
    let sql = r#"
            SELECT id, name, notes, is_default, created_at, updated_at
            FROM wishlists
            WHERE id = ?
        "#;

    let id_str = id.to_string();
    let res = sqlx::query_as::<_, WishlistRow>(sql)
        .bind(&id_str)
        .fetch_optional(executor)
        .await
        .with_context(|| format!("querying wishlist id={}", id_str))?;

    Ok(res)
}

pub async fn find_wishlist_items_by_id(
    executor: &mut sqlx::SqliteConnection,
    wishlist_id: &WishlistId,
) -> Result<Vec<WishlistItemRow>> {
    let sql = r#"
            SELECT
                id,
                wishlist_id,
                railway_model_id,
                priority,
                status,
                desired_price_amount,
                desired_price_currency,
                added_date,
                removed_date,
                notes,
                purchased_at,
                purchased_price_amount,
                purchased_price_currency
            FROM wishlist_items
            WHERE wishlist_id = ?
            ORDER BY added_date ASC
        "#;

    let id_str = wishlist_id.to_string();
    let rows = sqlx::query_as::<_, WishlistItemRow>(sql)
        .bind(&id_str)
        .fetch_all(executor)
        .await
        .with_context(|| format!("querying wishlist items with id={}", wishlist_id))?;

    Ok(rows)
}

pub async fn find_wishlist_previews(
    executor: &mut sqlx::SqliteConnection,
) -> Result<Vec<WishlistPreviewProjection>> {
    let sql = r#"
        SELECT
            w.id as wishlist_id,
            w.name,
            w.notes,
            w.is_default,
            w.updated_at,
            wi.desired_price_currency as currency,
            SUM(wi.desired_price_amount) as total_amount,
            COUNT(wi.id) as item_count
        FROM wishlists w
        LEFT JOIN wishlist_items wi ON wi.wishlist_id = w.id
        GROUP BY w.id, wi.desired_price_currency
        ORDER BY w.updated_at DESC
    "#;

    let rows = sqlx::query_as::<_, WishlistPreviewProjection>(sql)
        .fetch_all(executor)
        .await
        .with_context(|| "querying wishlist previews")?;

    Ok(rows)
}

pub async fn insert_wishlist(
    executor: &mut sqlx::SqliteConnection,
    row: WishlistRow,
) -> Result<()> {
    let sql = r#"
        INSERT INTO wishlists (id, name, notes, is_default, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?)
    "#;

    sqlx::query(sql)
        .bind(row.id)
        .bind(row.name)
        .bind(row.notes)
        .bind(row.is_default)
        .bind(row.created_at)
        .bind(row.updated_at)
        .execute(executor)
        .await
        .with_context(|| "inserting wishlist")?;

    Ok(())
}

pub async fn update_wishlist_name(
    executor: &mut sqlx::SqliteConnection,
    id: &WishlistId,
    name: &str,
) -> Result<u64> {
    let sql = r#"
        UPDATE wishlists
        SET name = ?, updated_at = CURRENT_TIMESTAMP
        WHERE id = ?
    "#;

    let res = sqlx::query(sql)
        .bind(name)
        .bind(id.to_string())
        .execute(executor)
        .await
        .with_context(|| "renaming wishlist")?;

    Ok(res.rows_affected())
}

pub async fn delete_wishlist(
    executor: &mut sqlx::SqliteConnection,
    id: &WishlistId,
) -> Result<u64> {
    let sql = r#"
        DELETE FROM wishlists
        WHERE id = ?
    "#;

    let res = sqlx::query(sql)
        .bind(id.to_string())
        .execute(executor)
        .await
        .with_context(|| "deleting wishlist")?;

    Ok(res.rows_affected())
}

pub async fn set_default_wishlist(
    executor: &mut sqlx::SqliteConnection,
    id: &WishlistId,
) -> Result<()> {
    // Clear existing defaults, then set the target as default within the same transaction.
    sqlx::query("UPDATE wishlists SET is_default = 0")
        .execute(&mut *executor)
        .await
        .with_context(|| "clearing default wishlists")?;

    sqlx::query("UPDATE wishlists SET is_default = 1, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
        .bind(id.to_string())
        .execute(executor)
        .await
        .with_context(|| "setting default wishlist")?;

    Ok(())
}

pub async fn insert_wishlist_item(
    executor: &mut sqlx::SqliteConnection,
    row: WishlistItemRow,
) -> Result<()> {
    let sql = r#"
        INSERT INTO wishlist_items (
            id,
            wishlist_id,
            railway_model_id,
            priority,
            status,
            desired_price_amount,
            desired_price_currency,
            added_date,
            removed_date,
            notes,
            purchased_at,
            purchased_price_amount,
            purchased_price_currency
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
    "#;

    sqlx::query(sql)
        .bind(row.id)
        .bind(row.wishlist_id)
        .bind(row.railway_model_id)
        .bind(row.priority)
        .bind(row.status)
        .bind(row.desired_price_amount)
        .bind(row.desired_price_currency)
        .bind(row.added_date)
        .bind(row.removed_date)
        .bind(row.notes)
        .bind(row.purchased_at)
        .bind(row.purchased_price_amount)
        .bind(row.purchased_price_currency)
        .execute(executor)
        .await
        .with_context(|| "inserting wishlist item")?;

    Ok(())
}

pub async fn delete_wishlist_item(
    executor: &mut sqlx::SqliteConnection,
    id: &crate::wishlist::domain::wishlist_item_id::WishlistItemId,
) -> Result<u64> {
    let sql = "DELETE FROM wishlist_items WHERE id = ?";

    let res = sqlx::query(sql)
        .bind(id.to_string())
        .execute(executor)
        .await
        .with_context(|| "deleting wishlist item")?;

    Ok(res.rows_affected())
}

pub async fn move_wishlist_item(
    executor: &mut sqlx::SqliteConnection,
    id: &crate::wishlist::domain::wishlist_item_id::WishlistItemId,
    destination: &WishlistId,
) -> Result<u64> {
    let sql = r#"
        UPDATE wishlist_items
        SET wishlist_id = ?
        WHERE id = ?
    "#;

    let res = sqlx::query(sql)
        .bind(destination.to_string())
        .bind(id.to_string())
        .execute(executor)
        .await
        .with_context(|| "moving wishlist item")?;

    Ok(res.rows_affected())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    #[sqlx::test(migrations = "./migrations")]
    async fn list_wishlist_previews_returns_empty(conn: SqlitePool) -> Result<()> {
        let mut conn = conn.acquire().await?;

        let id = WishlistId::default();
        let wishlist_row = WishlistRow {
            id: id.to_string(),
            name: "Test Wishlist".to_string(),
            notes: Some("Some notes".to_string()),
            is_default: 1,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        };

        let insert_res = insert_wishlist(&mut conn, wishlist_row.clone()).await;
        assert!(insert_res.is_ok());

        let query_res = find_wishlist_by_id(&mut conn, &id).await?;
        assert!(query_res.is_some());

        let fetched = query_res.unwrap();
        assert_eq!(fetched, wishlist_row);

        Ok(())
    }
}
