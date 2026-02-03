use crate::core::domain::domain_error::DomainError;

use crate::dcc_inventory::application::{
    CheckDuplicateAddressResult, DigitalRollingStockView, DigitalSummary,
    InstallableRollingStockView,
};
use crate::dcc_inventory::domain::{
    DccAddress, Decoder, DigitalRollingStock, DigitalRollingStockId,
};

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

    /// Get summary statistics for the digital rolling stock inventory.
    ///
    /// # Returns
    /// - `Ok(DigitalSummary)` containing summary statistics.
    /// - `Err(DomainError)` on repository errors.
    async fn get_digital_summary(&mut self) -> Result<DigitalSummary, DomainError>;

    /// Check if a DCC address is already in use.
    ///
    /// # Parameters
    /// - `address`: The DCC address to check.
    /// - `exclude_id`: Optional ID to exclude from the check (for edit scenarios).
    ///
    /// # Returns
    /// - `Ok(CheckDuplicateAddressResult)` with duplicate information.
    /// - `Err(DomainError)` on repository errors.
    async fn check_address_exists(
        &mut self,
        address: DccAddress,
        exclude_id: Option<DigitalRollingStockId>,
    ) -> Result<CheckDuplicateAddressResult, DomainError>;

    /// Find all rolling stocks that can have a decoder installed (non-dummies).
    ///
    /// # Returns
    /// - `Ok(Vec<InstallableRollingStockView>)` containing installable rolling stock views.
    /// - `Err(DomainError)` on repository errors.
    async fn find_installable_rolling_stocks(
        &mut self,
    ) -> Result<Vec<InstallableRollingStockView>, DomainError>;
}

/// Unit of Work extension providing access to DCC inventory repository.
pub trait DccInventoryUowExt: Send {
    /// Returns a repository for digital rolling stock bound to the UoW lifetime.
    fn digital_rolling_stocks_repository(&mut self) -> Box<dyn DigitalRollingStockRepository + '_>;
}
