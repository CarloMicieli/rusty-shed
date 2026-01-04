use crate::catalog::domain::railway_model::{RailwayModelId, RailwayModelParams};
use crate::core::domain::domain_error::DomainError;

/// A domain-agnostic interface for railway models data access.
///
/// This trait abstracts away the database engine (SQLite, Postgres, etc.),
/// allowing the business logic to remain decoupled from infrastructure details.
#[async_trait::async_trait]
pub trait RailwayModelRepository: Send + Sync {
    /// Persists a new Railway Model aggregate and all its associated rolling stocks to the storage.
    ///
    /// # Arguments
    /// * `params` - The domain parameters for the railway model to create.
    ///
    /// # Returns
    /// * Returns the unique identifier of the newly created Railway Model.
    /// * Returns a [`DomainError::DatabaseError`] if the transaction fails or
    /// if there is a constraint violation (e.g., duplicate invoice number).
    ///
    /// ## Implementation Requirements (Infrastructure Layer)
    /// * **Atomicity:** Must use a database transaction. If the parent railway model or
    ///   any of the [`RollingStockParams`] fail to save, the entire operation must roll back.
    /// * **ID Generation:** The implementation is responsible for generating the
    ///   unique database ID and returning it.
    /// * **Mapping:** Must map the Domain [`RailwayModelParams`] into the specific
    ///   database schema (e.g., `railway_models` and `rolling_stocks` tables).
    async fn create(&mut self, params: &RailwayModelParams) -> Result<RailwayModelId, DomainError>;
}
