use crate::catalog::domain::railway_model::coupler_type::CouplerType;
use crate::catalog::domain::railway_model::coupler_type_id::CouplerTypeId;
use crate::catalog::domain::railway_model::coupling_socket::CouplingSocket;
use crate::collecting::domain::OwnedRollingStockId;
use crate::core::domain::domain_error::DomainError;

/// Data-access interface for coupler type lookups and owned-stock coupler assignment.
#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait CouplerRepository: Send {
    /// Return all coupler types, optionally filtered to those compatible with `socket`.
    async fn find_all(
        &mut self,
        socket: Option<CouplingSocket>,
    ) -> Result<Vec<CouplerType>, DomainError>;

    /// Return the currently installed coupler id for an owned rolling stock, if any.
    async fn get_current_coupler(
        &mut self,
        owned_rs_id: &OwnedRollingStockId,
    ) -> Result<Option<CouplerTypeId>, DomainError>;

    /// Set (or clear) the installed coupler on an owned rolling stock.
    async fn set_current_coupler(
        &mut self,
        owned_rs_id: &OwnedRollingStockId,
        coupler_id: Option<CouplerTypeId>,
    ) -> Result<(), DomainError>;
}

/// Extension trait to attach the coupler repository to the Unit of Work.
pub trait CouplerUowExt: Send {
    /// Returns a coupler repository bound to the current unit-of-work transaction.
    fn coupler_repository(&mut self) -> Box<dyn CouplerRepository + '_>;
}
