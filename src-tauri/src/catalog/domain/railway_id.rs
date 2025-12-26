use serde::{Deserialize, Serialize};

/// Strongly-typed identifier for a railway in the catalog domain.
///
/// `RailwayId` wraps a string value and provides a distinct type instead of
/// using plain strings everywhere. This improves type safety and makes intent
/// explicit in function signatures and data structures.
///
/// The inner string is kept private to allow the crate to enforce invariants
/// or to provide controlled constructors/parsers elsewhere. `RailwayId` also
/// derives Serde traits so it serializes/deserializes as a plain string.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct RailwayId(String);

impl RailwayId {
    /// Creates a new `RailwayId` from the given string.
    ///
    /// # Parameters
    ///
    /// - `id`: the string identifier for the railway
    ///
    /// # Returns
    ///
    /// A new `RailwayId` instance wrapping the provided string.
    pub fn new<S: Into<String>>(id: S) -> Self {
        RailwayId(id.into())
    }
}
