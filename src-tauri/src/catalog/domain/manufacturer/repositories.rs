use crate::catalog::domain::manufacturer::{Manufacturer, ManufacturerId};
use crate::core::domain::domain_error::DomainError;

/// Repository trait for managing Manufacturers data.
#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait ManufacturerRepository: Send + Sync {
    /// Retrieves all Railway Model manufacturers from the database.
    ///
    /// # Arguments
    /// none
    ///
    /// # Returns
    /// - Returns the list of all [`Manufacturer`]s in the database.
    /// - Returns a [`DomainError::DatabaseError`] if the query fails.
    async fn find_all(&mut self) -> Result<Vec<Manufacturer>, DomainError>;

    /// Fetch a manufacturer row by its ID.
    ///
    /// This function executes a simple SELECT query against the `manufacturers` table
    /// and returns the matching `ManufacturerRow` if present.
    ///
    /// # Arguments
    /// - `id` - The manufacturer identifier to look up.
    ///
    /// # Returns
    /// - `Ok(Some(ManufacturerRow))` when a matching row is found
    /// - `Ok(None)` when no row matches the provided `id`
    /// - `Err(DomainError)` if the query fails.
    async fn find_by_id(
        &mut self,
        id: &ManufacturerId,
    ) -> Result<Option<Manufacturer>, DomainError>;
}

/// An extension trait that provides access to the `ManufacturerRepository`.
///
/// This follows the **Interface Segregation Principle**. By using extension traits,
/// we avoid a "God Object" where one struct knows about every repository in the
/// system. Instead, repositories are grouped by domain logic.
pub trait ManufacturerUowExt: Send {
    /// Returns a trait object for interacting with manufacturers model data.
    ///
    /// The repository is bound to the lifetime of the Unit of Work to ensure
    /// it cannot outlive the transaction it relies on.
    fn manufacturers_repo(&mut self) -> Box<dyn ManufacturerRepository + '_>;
}
