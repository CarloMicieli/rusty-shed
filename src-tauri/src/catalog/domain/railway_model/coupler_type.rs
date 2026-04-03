use crate::catalog::domain::railway_model::coupler_type_id::CouplerTypeId;
use crate::catalog::domain::railway_model::coupling_socket::CouplingSocket;
use serde::{Deserialize, Serialize};

/// A physical coupler product that can be installed on owned rolling stock.
///
/// `CouplerType` is a reference/lookup entity — rows are seeded from `couplers.csv`
/// and are keyed by `CouplerTypeId`.  The `compatible_socket` field drives UI
/// filtering so only couplers that fit the vehicle's socket are shown.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct CouplerType {
    /// Unique TRN identifier (e.g. `trn:coupler:roco:roco-universal-40397`).
    pub id: CouplerTypeId,
    /// Manufacturer of the coupler (e.g. "Roco").
    pub manufacturer: String,
    /// Commercial name / product description (e.g. "Roco Universal (40397)").
    pub name: String,
    /// Socket standard this coupler fits into.
    pub compatible_socket: CouplingSocket,
}
