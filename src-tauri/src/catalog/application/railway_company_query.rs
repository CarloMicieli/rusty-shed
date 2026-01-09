use crate::catalog::domain::railway_company::{
    RailwayCompany, RailwayCompanyId, RailwayCompanyUowExt,
};
use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;

/// Query to retrieve all railway companies from the database.
pub struct GetRailwayCompaniesQuery;

impl GetRailwayCompaniesQuery {
    /// Execute the query to retrieve all railway companies.
    ///
    /// # Arguments
    /// * `unit_of_work` - The unit of work managing the database transaction.
    ///
    /// # Returns
    /// - `Ok(Vec<RailwayCompany>)` containing all railway companies on success.
    /// - `Err(DomainError)` with an error message on failure.
    pub async fn execute(
        unit_of_work: &mut SqliteUnitOfWork<'_>,
    ) -> Result<Vec<RailwayCompany>, DomainError> {
        let mut repository = unit_of_work.railway_companies_repo();
        repository.find_all().await
    }
}

/// Query to retrieve a railway company by id from the database.
pub struct GetRailwayCompanyByIdQuery;

impl GetRailwayCompanyByIdQuery {
    /// Execute the query to get a railway company by id
    ///
    /// # Arguments
    /// * `unit_of_work` - The unit of work managing the database transaction.
    /// * `railway_company_id` - The identifier of the railway company to retrieve.
    ///
    /// # Returns
    /// - `Ok(Some(RailwayCompany))` when the railway company is found.
    /// - `Ok(None)` when the railway company is not found.
    /// - `Err(DomainError)` with an error message on failure.
    pub async fn execute(
        unit_of_work: &mut SqliteUnitOfWork<'_>,
        railway_company_id: RailwayCompanyId,
    ) -> Result<Option<RailwayCompany>, DomainError> {
        let mut repository = unit_of_work.railway_companies_repo();
        repository.find_by_id(&railway_company_id).await
    }
}
