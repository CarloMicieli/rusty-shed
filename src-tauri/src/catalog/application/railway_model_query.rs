use crate::catalog::domain::railway_model::{RailwayModel, RailwayModelId, RailwayModelUowExt};
use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use log::info;

/// Query to retrieve a railway model by id from the database.
pub struct GetRailwayModelByIdQuery;

impl GetRailwayModelByIdQuery {
    /// Execute the query to get a railway model by id
    ///
    /// # Arguments
    /// * `unit_of_work` - The unit of work managing the database transaction.
    /// * `railway_model_id` - The identifier of the railway company to retrieve.
    ///
    /// # Returns
    /// - `Ok(Option<RailwayModel>)` containing all railway companies on success.
    /// - `Err(DomainError)` with an error message on failure.
    pub async fn execute(
        unit_of_work: &mut SqliteUnitOfWork<'_>,
        railway_model_id: RailwayModelId,
    ) -> Result<Option<RailwayModel>, DomainError> {
        info!("Getting railway model by id");
        let mut repository = unit_of_work.railway_model_repository();
        repository.find_by_id(&railway_model_id).await
    }
}
