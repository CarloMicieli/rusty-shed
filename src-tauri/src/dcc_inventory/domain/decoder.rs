use crate::catalog::domain::manufacturer::ManufacturerId;
use crate::catalog::domain::railway_model::DccInterface;
use crate::dcc_inventory::domain::{DecoderId, DecoderType, DigitalProtocol};
use serde::{Deserialize, Serialize};

/// A Decoder models a real-world model-railway decoder product.
///
/// In the application domain a decoder is the canonical master record for a
/// specific manufacturer product (for example an ESU LokPilot model). It
/// captures the identity and capabilities of the physical module that can be
/// installed into locomotives and other rolling stock.
///
/// Important points for developers:
/// - `Decoder` instances are stored centrally so many `OwnedRollingStock`
///   records can reference the same decoder model via `installed_decoder_id`.
/// - The `id` is a stable URN that identifies the product (example:
///   `trn:decoder:esu:54621`). Use that URN when linking or displaying decoder
///   information.
///
/// Fields (plain language):
/// - `id` — stable product identifier (URN) for the decoder model.
/// - `manufacturer_id` — reference to the manufacturer entry.
/// - `product_code` — human-facing product code or name from the manufacturer.
/// - `decoder_type` — functional family (e.g. plain, sound).
/// - `protocol` — digital protocol(s) the decoder supports (DCC, etc.).
/// - `decoder_interface` — the physical DCC interface the decoder exposes.
///
/// This struct is primarily used when recording which decoder model is
/// installed into a piece of rolling stock and when presenting decoder details
/// to the user (for example in an equipment detail view).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "camelCase")]
pub struct Decoder {
    /// Unique identifier (URN) for this decoder.
    pub id: DecoderId,

    /// Reference to the manufacturer (foreign key to `manufacturers.id`).
    pub manufacturer_id: ManufacturerId,

    /// Product code as provided by the manufacturer (human-readable).
    pub product_code: String,

    /// The functional type of the decoder (plain, sound, etc.).
    pub decoder_type: DecoderType,

    /// The communication protocol implemented by this decoder.
    pub protocol: DigitalProtocol,

    /// The physical DCC interface the decoder exposes (if any).
    pub decoder_interface: DccInterface,
}
