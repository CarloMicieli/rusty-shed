use crate::catalog::domain::manufacturer::ManufacturerId;
use crate::catalog::domain::railway_model::{DccInterface, RailwayModelId, RollingStockId};
use crate::collecting::domain::CollectionItemId;
use crate::collecting::domain::OwnedRollingStockId;
use crate::collecting::domain::PurchaseInfoId;
use crate::collecting::domain::{BoxCondition, CollectionId, ModelCondition, PurchaseCondition};
use crate::dcc_inventory::domain::{DecoderId, DecoderType, DigitalProtocol};
use crate::sellers::domain::seller_id::SellerId;
use chrono::{NaiveDate, NaiveDateTime};

/// Row mapping for the `collections` table.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CollectionRow {
    pub id: CollectionId,
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
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CollectionItemRow {
    pub id: CollectionItemId,
    pub collection_id: CollectionId,
    pub railway_model_id: RailwayModelId,
    pub added_date: NaiveDate,
    pub removed_date: Option<NaiveDate>,
    pub purchase_condition: Option<PurchaseCondition>,
    pub model_condition: Option<ModelCondition>,
    pub box_condition: Option<BoxCondition>,
    pub notes: Option<String>,
}

/// Row mapping for the `owned_rolling_stocks` table.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct OwnedRollingStockRow {
    pub id: OwnedRollingStockId,
    pub collection_item_id: CollectionItemId,
    pub rolling_stock_id: Option<RollingStockId>,
    pub notes: Option<String>,
    // DCC address stored as INTEGER in DB; mapped to u16 in domain when present
    pub dcc_address: Option<i64>,
    // References decoders(id) when a decoder is installed
    pub installed_decoder_id: Option<String>,
    // When joined with `decoders`, the following fields are populated (all optional):
    pub decoder_id: Option<DecoderId>,
    pub decoder_manufacturer_id: Option<ManufacturerId>,
    pub decoder_product_code: Option<String>,
    pub decoder_type: Option<DecoderType>,
    pub decoder_protocol: Option<DigitalProtocol>,
    pub decoder_interface: Option<DccInterface>,
}

/// Row mapping for the `decoders` table. Used when LEFT JOINing decoder data
/// into collection queries and mapping the flat result into domain types.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DecoderRow {
    pub id: DecoderId,
    pub manufacturer_id: ManufacturerId,
    pub product_code: Option<String>,
    pub decoder_type: DecoderType,
    pub protocol: DigitalProtocol,
    pub decoder_interface: DccInterface,
}

/// Row mapping for the `purchase_infos` table.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PurchaseInfoRow {
    pub id: PurchaseInfoId,
    pub collection_item_id: CollectionItemId,
    pub purchase_type: Option<String>,
    pub purchase_date: NaiveDate,
    pub seller_id: Option<SellerId>,
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
