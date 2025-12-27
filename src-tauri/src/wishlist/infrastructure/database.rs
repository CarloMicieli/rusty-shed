use crate::wishlist::infrastructure::entities::{WishlistItemRow, WishlistRow};
use anyhow::{Context, Result};

pub async fn find_wishlist_by_id(
    executor: &mut sqlx::SqliteConnection,
    id: &str,
) -> Result<Option<WishlistRow>> {
    let sql = r#"
            SELECT id, name, notes, is_default, created_at, updated_at
            FROM wishlists
            WHERE id = ?
        "#;

    let res = sqlx::query_as::<_, WishlistRow>(sql)
        .bind(id)
        .fetch_optional(executor)
        .await
        .with_context(|| format!("querying wishlist id={}", id))?;

    Ok(res)
}

pub async fn find_wishlist_items_by_id(
    executor: &mut sqlx::SqliteConnection,
    wishlist_id: &str,
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
                purchased_price_currency,
            FROM wishlist_items
            WHERE wishlist_id = ?
            ORDER BY added_date ASC
        "#;

    let rows = sqlx::query_as::<_, WishlistItemRow>(sql)
        .bind(wishlist_id)
        .fetch_all(executor)
        .await
        .with_context(|| format!("querying wishlist items with id={}", wishlist_id))?;

    Ok(rows)
}
