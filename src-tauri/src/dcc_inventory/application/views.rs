use crate::catalog::domain::railway_model::DccInterface;
use crate::collecting::domain::OwnedRollingStockId;
use crate::dcc_inventory::domain::{
    DccAddress, DecoderId, DecoderType, DigitalProtocol, DigitalRollingStockId,
};

/// View representation returned by queries
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct DigitalRollingStockView {
    /// The unique identifier of the digital rolling stock
    pub id: DigitalRollingStockId,
    /// The associated owned rolling stock identifier
    pub owned_rolling_stock_id: OwnedRollingStockId,
    /// The DCC address assigned to the digital rolling stock
    pub dcc_address: DccAddress,
    /// The decoder information associated with the digital rolling stock
    pub decoder: DecoderView,
}

/// View representation of a decoder used within `DigitalRollingStockView`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct DecoderView {
    /// The unique identifier of the decoder
    pub id: DecoderId,
    /// The manufacturer name of the decoder
    pub manufacturer: String,
    /// The product code of the decoder
    pub product_code: String,
    /// The type of the decoder
    pub decoder_type: DecoderType,
    /// The digital protocol supported by the decoder
    pub protocol: DigitalProtocol,
    /// The DCC interface type of the decoder
    pub decoder_interface: DccInterface,
}
