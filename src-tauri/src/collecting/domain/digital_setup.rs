use crate::catalog::domain::railway_model::DccInterface;
use crate::collecting::domain::decoder_id::DecoderId;
use serde::{Deserialize, Serialize};

/// Represents the installation of a decoder into a locomotive (owned rolling stock).
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct DigitalSetup {
    /// The physical interface present on the locomotive (socket/plug type).
    pub interface: DccInterface,

    /// The DCC address assigned to the decoder.
    pub dcc_address: u16,

    /// The installed decoder id (URN) referencing the `decoders` master table.
    pub installed_decoder_id: DecoderId,
}
