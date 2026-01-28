use crate::collecting::domain::OwnedRollingStockId;
use crate::core::domain::metadata::Metadata;
use crate::dcc_inventory::domain::digital_rolling_stock_id::DigitalRollingStockId;
use crate::dcc_inventory::domain::{DccAddress, DecoderId, DigitalRollingStockEvent};

#[derive(Debug, Clone)]
pub struct DigitalRollingStock {
    /// Unique identifier for this digital rolling stock entry.
    pub id: DigitalRollingStockId,

    /// Reference to the owning rolling stock (from the collecting domain).
    pub owned_rolling_stock_id: OwnedRollingStockId,

    /// Assigned DCC address.
    pub dcc_address: DccAddress,

    /// Decoder reference (master record).
    pub decoder_id: DecoderId,

    /// Pending domain events for the aggregate.
    pub pending_events: Vec<DigitalRollingStockEvent>,

    /// Metadata for the resource.
    pub metadata: Metadata,
}

impl DigitalRollingStock {
    /// Create a new `DigitalRollingStock` instance.
    pub fn new(
        id: DigitalRollingStockId,
        owned_rolling_stock_id: OwnedRollingStockId,
        dcc_address: DccAddress,
        decoder_id: DecoderId,
    ) -> Self {
        Self {
            id,
            owned_rolling_stock_id,
            dcc_address,
            decoder_id,
            pending_events: Vec::new(),
            metadata: Metadata::default(),
        }
    }

    /// Change the decoder and emit an event.
    pub fn change_decoder(&mut self, decoder_id: DecoderId) {
        self.decoder_id = decoder_id.clone();
        self.pending_events
            .push(DigitalRollingStockEvent::DecoderChanged { decoder_id });
    }

    /// Change the DCC address and emit an event.
    pub fn change_dcc_address(&mut self, new_address: DccAddress) {
        self.dcc_address = new_address;
        self.pending_events
            .push(DigitalRollingStockEvent::DccAddressChanged {
                dcc_address: new_address,
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use uuid::Uuid;

    #[test]
    fn digital_rolling_stock_create_struct() {
        use crate::collecting::domain::OwnedRollingStockId;

        let u = Uuid::new_v4();
        let id = crate::dcc_inventory::domain::DigitalRollingStockId::from_uuid(u);
        let owned = OwnedRollingStockId::from(Uuid::new_v4());
        let addr = crate::dcc_inventory::domain::DccAddress::new(500).unwrap();

        let decoder = DecoderId::try_from("trn:decoder:acme:d-100").expect("should parse");

        let drs = DigitalRollingStock::new(id.clone(), owned.clone(), addr, decoder.clone());

        assert_eq!(drs.id, id);
        assert_eq!(drs.owned_rolling_stock_id, owned);
        assert_eq!(drs.pending_events.len(), 0);
        assert_eq!(drs.decoder_id, decoder);
    }
}
