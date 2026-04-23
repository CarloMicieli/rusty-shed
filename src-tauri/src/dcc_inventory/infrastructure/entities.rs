use crate::catalog::domain::manufacturer::ManufacturerId;
use crate::catalog::domain::railway_model::DccInterface;
use crate::collecting::domain::OwnedRollingStockId;
use crate::dcc_inventory::domain::DigitalRollingStockId;
use crate::dcc_inventory::domain::{DecoderId, DecoderType, DigitalProtocol};

/// Row mapping for the `digital_rolling_stocks` table.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DigitalRollingStockRow {
    pub id: DigitalRollingStockId,
    pub owned_rolling_stock_id: OwnedRollingStockId,
    pub dcc_address: u16,
    pub installed_decoder_id: Option<DecoderId>,
}

/// Row mapping for the `decoders` table.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DecoderRow {
    pub id: DecoderId,
    pub manufacturer_id: ManufacturerId,
    pub product_code: String,
    pub decoder_type: DecoderType,
    pub protocol: DigitalProtocol,
    pub decoder_interface: DccInterface,
}

#[derive(sqlx::FromRow)]
pub struct ManufacturerNameRow {
    pub id: ManufacturerId,
    pub name: String,
}

/// Full projection row used by the "all digital rolling stocks" view query.
///
/// This struct is `pub(crate)` so that `mappers` can convert it to the
/// application-level `DigitalRollingStockView` without exposing the raw DB
/// shape to consumers outside this crate.
#[derive(sqlx::FromRow)]
pub struct EnrichedRow {
    pub id: DigitalRollingStockId,
    pub owned_rolling_stock_id: OwnedRollingStockId,
    pub dcc_address: u16,
    pub decoder_id: DecoderId,
    pub decoder_product_code: String,
    pub decoder_type: DecoderType,
    pub decoder_protocol: DigitalProtocol,
    pub decoder_interface: DccInterface,
    pub manufacturer_name: Option<String>,
    pub category: Option<String>,
    pub road_number: Option<String>,
    pub series_code: Option<String>,
    pub description: Option<String>,
    pub railway_company_name: Option<String>,
    pub scale: Option<String>,
    pub power_method: Option<String>,
}

/// Projection row for the "installable rolling stocks" query.
#[derive(sqlx::FromRow)]
pub struct InstallableRow {
    pub owned_rolling_stock_id: OwnedRollingStockId,
    pub category: Option<String>,
    pub road_number: Option<String>,
    pub series_code: Option<String>,
    pub railway_company_name: Option<String>,
    pub has_decoder: i32,
    pub dcc_interface: Option<DccInterface>,
}

/// Aggregated count row returned by the digital summary query.
#[derive(sqlx::FromRow)]
pub struct SummaryRow {
    pub total_non_dummy: i64,
    pub digital_count: i64,
}
