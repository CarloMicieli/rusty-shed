use crate::catalog::domain::railway_model::DccInterface;
use crate::catalog::domain::railway_model::{PowerMethod, RollingStockCategory};
use crate::catalog::domain::scale::Scale;
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
    /// The category of the rolling stock (LOCOMOTIVE, PASSENGER_CAR, etc.)
    pub category: RollingStockCategory,
    /// The name of the railway company, if available
    pub railway_company_name: Option<String>,
    /// The scale of the rolling stock model, if available
    pub scale: Option<Scale>,
    /// The power method of the rolling stock, if available
    pub power_method: Option<PowerMethod>,
    /// The road number of the rolling stock, if available
    pub road_number: Option<String>,
    /// The series code of the rolling stock, if available
    pub series_code: Option<String>,
    /// A description of the rolling stock, if available
    pub description: Option<String>,
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

/// Summary statistics for digital rolling stock inventory
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct DigitalSummary {
    /// Total number of non-dummy rolling stocks in the collection
    pub total_non_dummy: u32,
    /// Number of digital rolling stocks (factory-fitted or user-installed)
    pub digital_count: u32,
    /// Percentage of digital rolling stocks (0.0 - 100.0)
    pub percentage: f32,
}

/// Result of checking for duplicate DCC addresses
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct CheckDuplicateAddressResult {
    /// True if the address is already in use by another rolling stock
    pub is_duplicate: bool,
    /// The rolling stock ID using this address, if any
    pub existing_rolling_stock_id: Option<DigitalRollingStockId>,
}

/// View representation of rolling stock that can have a decoder installed
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct InstallableRollingStockView {
    /// The owned rolling stock identifier
    pub owned_rolling_stock_id: OwnedRollingStockId,
    /// The category of the rolling stock
    pub category: RollingStockCategory,
    /// The name of the railway company, if available
    pub railway_company_name: Option<String>,
    /// The road number of the rolling stock, if available
    pub road_number: Option<String>,
    /// The series code of the rolling stock, if available
    pub series_code: Option<String>,
    /// True if this rolling stock already has a decoder installed
    pub has_decoder: bool,
}
