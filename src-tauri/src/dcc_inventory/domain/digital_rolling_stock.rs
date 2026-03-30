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
    /// Create a new `DigitalRollingStock` aggregate and emit a `Created` event.
    ///
    /// Use this constructor when provisioning a brand-new record. The emitted
    /// event is consumed by the repository's `save()` to perform the INSERT.
    pub fn new(
        id: DigitalRollingStockId,
        owned_rolling_stock_id: OwnedRollingStockId,
        dcc_address: DccAddress,
        decoder_id: DecoderId,
    ) -> Self {
        let mut aggregate = Self {
            id,
            owned_rolling_stock_id: owned_rolling_stock_id.clone(),
            dcc_address,
            decoder_id: decoder_id.clone(),
            pending_events: Vec::new(),
            metadata: Metadata::default(),
        };
        aggregate
            .pending_events
            .push(DigitalRollingStockEvent::Created {
                owned_rolling_stock_id,
                dcc_address,
                decoder_id,
            });
        aggregate
    }

    /// Reconstitute a `DigitalRollingStock` from persisted data without emitting events.
    ///
    /// Used exclusively by the repository's `find_by_id` to hydrate the aggregate
    /// from a database row without producing spurious `Created` events.
    pub(crate) fn reconstitute(
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

    /// Drain and return all pending domain events.
    ///
    /// The internal buffer is cleared after this call. The repository's `save()`
    /// calls this once to obtain the events to persist.
    pub fn pull_events(&mut self) -> Vec<DigitalRollingStockEvent> {
        std::mem::take(&mut self.pending_events)
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
    fn digital_rolling_stock_new_emits_created_event() {
        use crate::collecting::domain::OwnedRollingStockId;

        let u = Uuid::new_v4();
        let id = crate::dcc_inventory::domain::DigitalRollingStockId::from_uuid(u);
        let owned = OwnedRollingStockId::from(Uuid::new_v4());
        let addr = crate::dcc_inventory::domain::DccAddress::new(500).unwrap();

        let decoder = DecoderId::try_from("trn:decoder:acme:d-100").expect("should parse");

        let drs = DigitalRollingStock::new(id.clone(), owned.clone(), addr, decoder.clone());

        assert_eq!(drs.id, id);
        assert_eq!(drs.owned_rolling_stock_id, owned);
        // new() must emit exactly one Created event for the repository to INSERT
        assert_eq!(drs.pending_events.len(), 1);
        assert!(matches!(
            drs.pending_events[0],
            crate::dcc_inventory::domain::DigitalRollingStockEvent::Created { .. }
        ));
        assert_eq!(drs.decoder_id, decoder);
    }

    #[test]
    fn digital_rolling_stock_reconstitute_has_no_events() {
        use crate::collecting::domain::OwnedRollingStockId;

        let u = Uuid::new_v4();
        let id = crate::dcc_inventory::domain::DigitalRollingStockId::from_uuid(u);
        let owned = OwnedRollingStockId::from(Uuid::new_v4());
        let addr = crate::dcc_inventory::domain::DccAddress::new(500).unwrap();
        let decoder = DecoderId::try_from("trn:decoder:acme:d-100").expect("should parse");

        let drs =
            DigitalRollingStock::reconstitute(id.clone(), owned.clone(), addr, decoder.clone());

        assert_eq!(drs.pending_events.len(), 0);
    }
}
