use crate::catalog::domain::railway_id::RailwayId;
use serde::{Deserialize, Serialize};
use std::fmt;

/// A railway association for a rolling stock item.
///
/// `RollingStockRailway` ties a rolling stock to a specific railway by
/// containing the railway's unique identifier and the display name used in
/// user interfaces and listings. This is a lightweight DTO-like value used in
/// domains where the rolling stock's owning or related railway must be shown
/// or serialized.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct RollingStockRailway {
    /// the railway unique identifier
    pub railway_id: RailwayId,
    /// the railway display name
    pub display: String,
}

impl RollingStockRailway {
    /// Creates a new `RollingStockRailway` with the given `railway_id` and
    /// human-friendly `display` text.
    pub fn new(railway_id: RailwayId, display: &str) -> Self {
        RollingStockRailway {
            railway_id,
            display: display.to_owned(),
        }
    }

    /// Returns this railway's unique identifier.
    pub fn id(&self) -> &RailwayId {
        &self.railway_id
    }

    /// Returns this railway's display text.
    pub fn display_text(&self) -> &str {
        &self.display
    }
}

impl fmt::Display for RollingStockRailway {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.display)
    }
}
