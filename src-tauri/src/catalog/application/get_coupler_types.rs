use crate::catalog::domain::railway_model::{CouplerType, CouplerUowExt, CouplingSocket};
use crate::core::domain::domain_error::DomainError;

/// Input for [`GetCouplerTypes::execute`].
pub struct GetCouplerTypesInput {
    /// When `Some`, only couplers compatible with this socket are returned.
    pub socket: Option<CouplingSocket>,
}

/// Query use-case that returns the coupler type catalogue, optionally filtered by socket.
pub struct GetCouplerTypes;

impl GetCouplerTypes {
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: GetCouplerTypesInput,
    ) -> Result<Vec<CouplerType>, DomainError>
    where
        U: CouplerUowExt + Send,
    {
        let mut repo = unit_of_work.coupler_repository();
        repo.find_all(input.socket).await
    }
}
