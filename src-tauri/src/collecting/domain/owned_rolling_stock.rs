use crate::catalog::domain::railway_model::{DccInterface, RollingStockId};
use crate::collecting::domain::OwnedRollingStockId;
use crate::dcc_inventory::domain::{Decoder, DecoderId};

#[derive(Debug, Clone)]
pub struct OwnedRollingStock {
    /// Unique identifier for this owned rolling stock record (e.g. UUID in the DB).
    pub id: OwnedRollingStockId,

    /// Identifier of the related rolling stock in the catalog (or the owned rolling stock id when catalog id is not available).
    pub rolling_stock_id: RollingStockId,

    /// Free-form notes associated with this owned instance.
    /// Use this for short owner notes or a brief textual label.
    pub notes: Option<String>,

    /// Optional digital setup information if a decoder is installed.
    pub installed_decoder_id: Option<DecoderId>,
}

/// Error conditions that may occur when attempting to install a decoder.
#[derive(Debug, thiserror::Error)]
pub enum InstallError {
    #[error("invalid dcc address: {0}")]
    InvalidAddress(u16),

    #[error("decoder interface incompatible: expected {expected}, found {found}")]
    IncompatibleInterface {
        expected: DccInterface,
        found: DccInterface,
    },
}

impl OwnedRollingStock {
    /// Install a decoder into this owned rolling stock.
    ///
    /// This function validates the `address` to be within the allowed DCC
    /// address range (1..=9999) and enforces that the decoder's exposed
    /// `decoder_interface` exactly matches the provided locomotive `interface`.
    ///
    /// On success the `digital` field is populated with the `DigitalSetup`.
    pub fn install_decoder(
        &mut self,
        interface: DccInterface,
        address: u16,
        decoder: &Decoder,
    ) -> Result<(), InstallError> {
        if address == 0 || address > 9999 {
            return Err(InstallError::InvalidAddress(address));
        }

        if decoder.decoder_interface != interface {
            return Err(InstallError::IncompatibleInterface {
                expected: decoder.decoder_interface,
                found: interface,
            });
        }

        self.installed_decoder_id = Some(decoder.id.clone());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::domain::manufacturer::ManufacturerId;
    use crate::collecting::domain::owned_rolling_stock_id::OwnedRollingStockId;
    use crate::dcc_inventory::domain::{Decoder, DecoderId, DecoderType, DigitalProtocol};

    #[test]
    fn it_should_install_decoder_validates_address_and_interface() {
        let rolling_stock_id = RollingStockId::new();
        let mut ors = OwnedRollingStock {
            id: OwnedRollingStockId::new("ors-1"),
            rolling_stock_id,
            notes: None,
            installed_decoder_id: None,
        };

        let decoder = Decoder {
            id: DecoderId::from_parts("Acme", "P100"),
            manufacturer_id: ManufacturerId::new("MN-ACME"),
            product_code: "P100".to_string(),
            decoder_type: DecoderType::Plain,
            protocol: DigitalProtocol::Dcc,
            decoder_interface: DccInterface::Mtc21,
        };

        // invalid address
        let err = ors
            .install_decoder(DccInterface::Mtc21, 0, &decoder)
            .expect_err("address 0 should fail");
        match err {
            InstallError::InvalidAddress(0) => {}
            _ => panic!("unexpected error"),
        }

        // incompatible interface
        let err2 = ors
            .install_decoder(DccInterface::Nem651, 10, &decoder)
            .expect_err("interface mismatch should fail");
        match err2 {
            InstallError::IncompatibleInterface { expected, found } => {
                assert_eq!(expected, decoder.decoder_interface);
                assert_eq!(found, DccInterface::Nem651);
            }
            _ => panic!("unexpected error variant"),
        }
        /*
        // success
        ors.install_decoder(DccInterface::Mtc21, 1, &decoder)
            .expect("install should succeed");
        assert!(ors.digital.is_some());
        let ds = ors.digital.unwrap();
        assert_eq!(ds.dcc_address, 1u16);
        assert_eq!(ds.interface, DccInterface::Mtc21);
        assert_eq!(ds.installed_decoder_id, decoder.id);*/
    }
}
