use crate::dcc_inventory::domain::{DccAddress, DecoderId};

/// Represents a digitalized piece of railway equipment as a Domain-Driven Design (DDD)
/// Aggregate Root.
///
/// A `DigitalRollingStock` is the holistic representation of a physical model (locomotive,
/// multiple unit, or functional wagon) paired with its digital control interface (DCC Decoder).
/// Events emitted by the `DigitalRollingStock` aggregate.
#[derive(Debug, Clone)]
pub enum DigitalRollingStockEvent {
    DecoderChanged { decoder_id: DecoderId },
    DccAddressChanged { dcc_address: DccAddress },
}
