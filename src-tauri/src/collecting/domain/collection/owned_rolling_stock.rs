use crate::catalog::domain::Epoch;
use serde::{Deserialize, Serialize};

/// A lightweight view of rolling stock that references catalog model data.
///
/// This struct intentionally contains only the minimal information needed by
/// the collecting domain to reference a catalog `RailwayModel` and basic
/// provenance (railway and epoch). Detailed model information lives in the
/// catalog domain and should not be duplicated here.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnedRollingStock {
    /// Unique identifier for this owned rolling stock record (e.g. UUID in the DB).
    pub id: String,

    /// Identifier of the related rolling stock in the catalog (or the owned rolling stock id when catalog id is not available).
    pub rolling_stock_id: String,

    /// Free-form notes associated with this owned instance.
    /// Use this for short owner notes or a brief textual label.
    pub notes: String,

    /// Identifier for the railway company.
    pub railway_id: String,

    /// Historical epoch for this owned vehicle (copied from the catalog model).
    pub epoch: Epoch,
}
