use crate::catalog::domain::railway_model::{RailwayModelId, RailwayModelUowExt};
use crate::core::domain::domain_error::DomainError;

/// Input for [`SearchRailwayModels::execute`].
pub struct SearchRailwayModelsInput {
    /// The search query (minimum 2 characters).
    pub query: String,
}

/// Use case that searches railway models using FTS5 full-text search across all languages.
pub struct SearchRailwayModels;

impl SearchRailwayModels {
    /// Execute the search use case.
    ///
    /// # Errors
    /// - [`DomainError::Validation`] when `query` is shorter than 2 characters.
    /// - [`DomainError::Infrastructure`] on database failure.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: SearchRailwayModelsInput,
    ) -> Result<Vec<RailwayModelId>, DomainError>
    where
        U: RailwayModelUowExt + Send,
    {
        if input.query.trim().len() < 2 {
            return Err(DomainError::Validation(
                "search query must be at least 2 characters".to_string(),
            ));
        }

        let mut repository = unit_of_work.railway_model_repository();
        repository.search(&input.query).await
    }
}
