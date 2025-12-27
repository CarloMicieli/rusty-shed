use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct WishlistRow {
    pub id: String,
    pub name: String,
    pub notes: Option<String>,
    pub is_default: i64, // SQLite INTEGER 0/1
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

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
