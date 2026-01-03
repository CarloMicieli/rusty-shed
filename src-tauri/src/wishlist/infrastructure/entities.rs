use chrono::{NaiveDate, NaiveDateTime};

/// Database row representation for the `wishlists` table.
///
/// This struct is used with `sqlx::FromRow` to map query results to a typed
/// representation. Fields correspond to the `wishlists` table columns.
#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct WishlistRow {
    pub id: String,
    pub name: String,
    pub notes: Option<String>,
    pub is_default: i64, // SQLite INTEGER 0/1
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Database row representation for the `wishlist_items` table.
///
/// Represents a single wishlist item returned from the database. Monetary
/// fields are stored as separate amount/currency columns and are mapped to
/// domain `MonetaryAmount` when converting rows to domain types.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct WishlistItemRow {
    pub id: String,
    pub wishlist_id: String,
    pub railway_model_id: String,
    pub priority: String,
    pub status: String,
    pub desired_price_amount: Option<i64>,
    pub desired_price_currency: Option<String>,
    pub added_date: NaiveDate,
    pub removed_date: Option<NaiveDate>,
    pub notes: Option<String>,
    pub purchased_at: Option<NaiveDate>,
    pub purchased_price_amount: Option<i64>,
    pub purchased_price_currency: Option<String>,
}

/// Flat row used to build wishlist previews grouped by wishlist and currency.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct WishlistPreviewProjection {
    pub wishlist_id: String,
    pub name: String,
    pub notes: Option<String>,
    pub is_default: i64,
    pub updated_at: NaiveDateTime,
    pub currency: Option<String>,
    pub total_amount: Option<i64>,
    pub item_count: i64,
}
