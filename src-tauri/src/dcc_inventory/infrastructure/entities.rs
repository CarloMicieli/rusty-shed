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
