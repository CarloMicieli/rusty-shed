use super::track_code::TrackCode;
use super::track_id::TrackId;
use super::track_type::TrackType;
use crate::catalog::domain::manufacturer::ManufacturerId;
use crate::core::domain::length::Length;
use crate::core::domain::metadata::Metadata;
use serde::{Deserialize, Serialize};

/// Domain representation of a track product sold by a manufacturer.
///
/// `TrackProduct` captures the canonical product identity (`track_id`), the
/// manufacturer and product code, physical characteristics (length, radius)
/// and whether the piece includes integrated roadbed. Use this type as the
/// immutable domain model for track products when recording inventory,
/// purchases, or presenting product details in the UI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct TrackProduct {
    /// Canonical TRN identifier for this product (for example `trn:track:man:prod`).
    pub track_id: TrackId,

    /// Manufacturer product code or name (human-facing label supplied by the manufacturer).
    pub product_code: String,

    /// Reference to the manufacturer entity that produces this track product.
    pub manufacturer_id: ManufacturerId,

    /// Human-readable description of the track piece.
    pub description: String,

    /// Whether this track piece includes an integrated roadbed/base.
    pub with_roadbed: bool,

    /// Length for straight track pieces, when applicable.
    pub length: Option<Length>,

    /// Radius for curved track elements, when applicable.
    pub radius: Option<Length>,

    /// The geometric type of the track piece (e.g. Straight, Curve, Turnout).
    pub track_type: TrackType,

    /// The rail profile code (e.g. Code70, Code83) describing the rail height.
    pub track_code: TrackCode,

    /// Additional auxiliary metadata associated with the track product
    /// (for example timestamps, owner id or audit information).
    pub metadata: Metadata,
}
