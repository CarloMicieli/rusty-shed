use crate::core::domain::domain_error::DomainError;
use crate::trains::domain::formation::train_formation::TrainFormation;

/// Async trait for persisting and retrieving [`TrainFormation`] aggregates.
#[async_trait::async_trait]
pub trait TrainFormationRepository {
    /// Fetch a formation by its unique ID.
    ///
    /// Returns [`DomainError::NotFound`] when no matching record exists.
    async fn find_by_id(&mut self, id: &str) -> Result<TrainFormation, DomainError>;

    /// List all formations (summary level — elements not loaded).
    async fn find_all(&mut self) -> Result<Vec<TrainFormation>, DomainError>;

    /// Persist a formation (INSERT on new, UPDATE on existing).
    ///
    /// Increments the optimistic-concurrency `version` counter.
    async fn save(&mut self, formation: &TrainFormation) -> Result<(), DomainError>;

    /// Delete a formation and all its elements (cascade).
    async fn delete(&mut self, id: &str) -> Result<(), DomainError>;
}
