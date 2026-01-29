use crate::core::domain::domain_error::DomainError;

use crate::dcc_inventory::application::DigitalRollingStockView;
use crate::dcc_inventory::domain::{Decoder, DigitalRollingStock, DigitalRollingStockId};

/// Repository trait for accessing and persisting `DigitalRollingStock` aggregates
/// and related decoder master-data.
#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait DigitalRollingStockRepository: Send + Sync {
    /// Find a `DigitalRollingStock` aggregate by its id.
    ///
    /// # Parameters
    /// - `id`: The `DigitalRollingStockId` of the target aggregate.
    ///
    /// # Returns
    /// - `Ok(Some(DigitalRollingStock))` if found.
    /// - `Ok(None)` if not found.
    /// - `Err(DomainError)` on repository errors.
    async fn find_by_id(
        &mut self,
        id: &DigitalRollingStockId,
    ) -> Result<Option<DigitalRollingStock>, DomainError>;

    /// Persist a `DigitalRollingStock` aggregate.
    ///
    /// # Parameters
    /// - `digital_rolling_stock`: The `DigitalRollingStock` aggregate to persist.
    ///
    /// # Returns
    /// - `Ok(())` on successful persistence.
    /// - `Err(DomainError)` on repository errors.
    async fn save(&mut self, digital_rolling_stock: DigitalRollingStock)
    -> Result<(), DomainError>;

    /// Return all known `Decoder` master records.
    async fn find_all_decoders(&mut self) -> Result<Vec<Decoder>, DomainError>;

    /// Return all digitalized rolling stocks as views.
    ///
    /// # Returns
    /// - `Ok(Vec<DigitalRollingStockView>)` containing all digital rolling stock views.
    /// - `Err(DomainError)` on repository errors.
    async fn find_all_digital_rolling_stocks(
        &mut self,
    ) -> Result<Vec<DigitalRollingStockView>, DomainError>;
}

/// Unit of Work extension providing access to DCC inventory repository.
pub trait DccInventoryUowExt: Send {
    /// Returns a repository for digital rolling stock bound to the UoW lifetime.
    fn digital_rolling_stocks_repository(&mut self) -> Box<dyn DigitalRollingStockRepository + '_>;
}
