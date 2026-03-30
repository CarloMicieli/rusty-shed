use crate::collecting::domain::OwnedRollingStockId;
use crate::dcc_inventory::domain::{DccAddress, DecoderId};

/// Events emitted by the `DigitalRollingStock` aggregate.
///
/// Each variant carries all data necessary for the corresponding SQL operation,
/// so the repository's `handle_event` never reads fields from the aggregate directly.
#[derive(Debug, Clone)]
pub enum DigitalRollingStockEvent {
    /// A new digital rolling stock record was created.
    Created {
        owned_rolling_stock_id: OwnedRollingStockId,
        dcc_address: DccAddress,
        decoder_id: DecoderId,
    },
    /// The installed decoder was replaced.
    DecoderChanged { decoder_id: DecoderId },
    /// The DCC address was reassigned.
    DccAddressChanged { dcc_address: DccAddress },
}
