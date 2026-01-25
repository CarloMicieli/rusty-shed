use super::track_id::TrackId;
use serde::{Deserialize, Serialize};

/// Value object representing the available quantity for a specific track
/// product.
///
/// `TrackQuantity` pairs a canonical `TrackId` with the integer `quantity`
/// currently recorded in an inventory. Use domain repositories and use-cases
/// to update quantities rather than mutating instances directly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type, Hash)]
#[serde(rename_all = "camelCase")]
pub struct TrackQuantity {
    /// Canonical TRN identifier for the track product this quantity refers to.
    pub track_id: TrackId,

    /// Number of track items available in the inventory for this product.
    ///
    /// This is an integer count; domain logic should enforce non-negative
    /// invariants when appropriate.
    pub quantity: i64,
}
