use crate::catalog::domain::railway_company::{RailwayCompany, RailwayCompanyId};
use crate::core::domain::domain_error::DomainError;

/// Repository trait for managing Railway company data.
#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait RailwayCompanyRepository: Send + Sync {
    /// Retrieves all Railway company from the database.
    ///
    /// # Arguments
    /// none
    ///
    /// # Returns
    /// - Returns the list of all [`RailwayCompany`]s in the database.
    /// - Returns a [`DomainError::DatabaseError`] if the query fails.
    async fn find_all(&mut self) -> Result<Vec<RailwayCompany>, DomainError>;

    /// Retrieves a Railway company by its unique identifier.
    ///
    /// # Arguments
    /// - `id` - The unique identifier of the railway company.
    ///
    /// # Returns
    /// - Returns `Ok(Some(RailwayCompany))` if found.
    /// - Returns `Ok(None)` if no railway company with the given ID exists.
    /// - Returns a [`DomainError::DatabaseError`] if the query fails.
    async fn find_by_id(
        &mut self,
        id: &RailwayCompanyId,
    ) -> Result<Option<RailwayCompany>, DomainError>;
}

/// An extension trait that provides access to the `RailwayCompanyRepository`.
///
/// This follows the **Interface Segregation Principle**. By using extension traits,
/// we avoid a "God Object" where one struct knows about every repository in the
/// system. Instead, repositories are grouped by domain logic.
pub trait RailwayCompanyUowExt: Send {
    /// Returns a trait object for interacting with railway company model data.
    ///
    /// The repository is bound to the lifetime of the Unit of Work to ensure
    /// it cannot outlive the transaction it relies on.
    fn railway_companies_repo(&mut self) -> Box<dyn RailwayCompanyRepository + '_>;
}
