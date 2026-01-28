use crate::collecting::domain::OwnedRollingStockId;
use crate::dcc_inventory::domain::{DccAddress, DecoderId, DigitalRollingStockId};

/// Input for creating a new digital rolling stock
#[derive(Debug, Clone)]
pub struct NewDigitalRollingStockInput {
    pub owned_rolling_stock_id: OwnedRollingStockId,
    pub dcc_address: DccAddress,
    pub decoder_id: DecoderId,
}

/// Input for changing a decoder
#[derive(Debug, Clone)]
pub struct ChangeDecoderInput {
    pub id: DigitalRollingStockId,
    pub decoder_id: DecoderId,
}

/// Input for changing DCC address
#[derive(Debug, Clone)]
pub struct ChangeDccAddressInput {
    pub id: DigitalRollingStockId,
    pub new_dcc_address: DccAddress,
}

/// View representation returned by queries
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct DigitalRollingStockView {
    pub id: DigitalRollingStockId,
    pub owned_rolling_stock_id: OwnedRollingStockId,
    pub dcc_address: DccAddress,
    pub decoder_id: DecoderId,
}
