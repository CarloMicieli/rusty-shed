use crate::catalog::domain::manufacturer::{Manufacturer, ManufacturerId, ManufacturerUowExt};
use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use log::info;

/// Query to retrieve all manufacturers from the database.
pub struct GetManufacturersQuery;

impl GetManufacturersQuery {
    /// Execute the query to get all manufacturers.
    ///
    /// # Arguments
    /// * `unit_of_work` - The unit of work managing the database transaction.
    ///
    /// # Returns
    /// - `Ok(Vec<Manufacturer>)` containing all manufacturers on success.
    /// - `Err(DomainError)` with an error message on failure.
    pub async fn execute(
        unit_of_work: &mut SqliteUnitOfWork<'_>,
    ) -> Result<Vec<Manufacturer>, DomainError> {
        info!("Getting manufacturers");
        let mut repository = unit_of_work.manufacturers_repo();
        repository.find_all().await
    }
}

/// Query to retrieve all manufacturers from the database.
pub struct GetManufacturerByIdQuery;

impl GetManufacturerByIdQuery {
    /// Execute the query to get a manufacturer by id
    ///
    /// # Arguments
    /// * `unit_of_work` - The unit of work managing the database transaction.
    ///
    /// # Returns
    /// - `Ok(Option<Manufacturer>)` containing the manufacturer on success.
    /// - `Err(DomainError)` with an error message on failure.
    pub async fn execute(
        unit_of_work: &mut SqliteUnitOfWork<'_>,
        manufacturer_id: ManufacturerId,
    ) -> Result<Option<Manufacturer>, DomainError> {
        info!("Getting manufacturer by id");
        let mut repository = unit_of_work.manufacturers_repo();
        repository.find_by_id(&manufacturer_id).await
    }
}
