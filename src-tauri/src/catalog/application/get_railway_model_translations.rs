use crate::catalog::domain::railway_model::railway_model_translation::RailwayModelTranslations;
use crate::catalog::domain::railway_model::{RailwayModelId, RailwayModelUowExt};
use crate::core::domain::domain_error::DomainError;

/// Query to retrieve all stored translations for a railway model.
pub struct GetRailwayModelTranslations;

impl GetRailwayModelTranslations {
    /// Execute the query to retrieve all translations for a railway model.
    ///
    /// # Arguments
    /// * `unit_of_work` - The unit of work managing the database transaction.
    /// * `railway_model_id` - The identifier of the railway model.
    ///
    /// # Returns
    /// - `Ok(Some(RailwayModelTranslations))` when the model exists with translations.
    /// - `Ok(None)` when the model does not exist.
    /// - `Err(DomainError)` on database failure.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        railway_model_id: &RailwayModelId,
    ) -> Result<Option<RailwayModelTranslations>, DomainError>
    where
        U: RailwayModelUowExt + Send,
    {
        let mut repository = unit_of_work.railway_model_repository();
        repository.find_translations(railway_model_id).await
    }
}
