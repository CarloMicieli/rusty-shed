//! Database row representations for the `collecting` feature.
//!
//! These structs mirror the columns defined in the `0002_create_collection_schema` migration
//! and are intended only as a thin database representation (FromRow). Conversion to rich
//! domain types should happen in the repository layer.

use chrono::{NaiveDate, NaiveDateTime};

/// Row mapping for the `collections` table.
#[derive(Debug, sqlx::FromRow)]
pub struct CollectionRow {
    pub id: String,
    pub name: String,
    pub locomotives_count: i64,
    pub passenger_cars_count: i64,
    pub freight_cars_count: i64,
    pub train_sets_count: i64,
    pub railcars_count: i64,
    pub electric_multiple_units_count: i64,
    pub total_value_amount: i64,
    pub total_value_currency: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Row mapping for the `collection_items` table.
#[derive(Debug, sqlx::FromRow)]
pub struct CollectionItemRow {
    pub id: String,
    pub collection_id: String,
    pub railway_model_id: String,
    pub conditions: Option<String>,
    pub notes: Option<String>,
}

/// Row mapping for the `owned_rolling_stocks` table.
#[derive(Debug, sqlx::FromRow)]
pub struct OwnedRollingStockRow {
    pub id: String,
    pub collection_item_id: String,
    pub rolling_stock_id: Option<String>,
    pub notes: Option<String>,
}

/// Row mapping for the `purchase_infos` table.
#[derive(Debug, sqlx::FromRow)]
pub struct PurchaseInfoRow {
    pub purchase_id: String,
    pub collection_item_id: String,
    pub purchase_type: Option<String>,
    pub purchase_date: NaiveDate,
    pub seller_id: Option<String>,
    pub buyer_id: Option<String>,
    pub sale_date: Option<NaiveDate>,
    pub purchased_price_amount: Option<i64>,
    pub purchased_price_currency: Option<String>,
    pub sale_price_amount: Option<i64>,
    pub sale_price_currency: Option<String>,
    pub deposit_amount: Option<i64>,
    pub deposit_currency: Option<String>,
    pub preorder_total_amount: Option<i64>,
    pub preorder_total_currency: Option<String>,
    pub expected_date: Option<NaiveDate>,
}
