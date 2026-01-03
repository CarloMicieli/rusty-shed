use crate::catalog::domain::railway_model_id::RailwayModelId;
use crate::catalog::domain::{RailwayModel, RollingStock};
use crate::catalog::domain::params::RailwayModelParams;
use crate::core::domain::domain_error::DomainError;

/// A domain-agnostic interface for catalog data access.
///
/// This trait abstracts away the database engine (SQLite, Postgres, etc.),
/// allowing the business logic to remain decoupled from infrastructure details.
#[async_trait::async_trait]
pub trait CatalogRepository: Send + Sync {
    /// Inserts a new railway model into the repository.
    ///  
    /// # Arguments
    /// * `railway_model` - The railway model to insert.    
    ///
    /// # Returns
    /// * `Ok(())` on success.
    /// * `Err(anyhow::Error)` if an error occurs during insertion.
    async fn insert_railway_model(&mut self, railway_model: &RailwayModel) -> anyhow::Result<()>;

    /// Inserts a new rolling stock into the repository.
    ///
    /// # Arguments
    /// * `railway_model_id` - The identifier of the railway model to which the rolling stock belongs.
    /// * `rolling_stock` - The input data for creating the rolling stock.
    ///   
    /// # Returns
    /// * `Ok(())` on success.
    /// * `Err(anyhow::Error)` if an error occurs during insertion.
    async fn insert_rolling_stock(
        &mut self,
        railway_model_id: &RailwayModelId,
        rolling_stock: &RollingStock,
    ) -> anyhow::Result<()>;

    /// Persists a new Railway Model aggregate and all its associated rolling stocks to the storage.
    ///
    /// ### Implementation Requirements (Infrastructure Layer)
    /// * **Atomicity:** Must use a database transaction. If the parent railway model or 
    ///   any of the [`RollingStock`]s fail to save, the entire operation must roll back.
    /// * **ID Generation:** The implementation is responsible for generating the 
    ///   unique database ID and returning it.
    /// * **Mapping:** Must map the Domain [`RailwayModelParams`] into the specific 
    ///   database schema (e.g., `railway_models` and `rolling_stocks` tables).
    ///
    /// ### Errors
    /// Returns a [`DomainError::DatabaseError`] if the transaction fails or 
    /// if there is a constraint violation (e.g., duplicate invoice number).
    async fn create(&mut self, params: &RailwayModelParams) -> Result<RailwayModelId, DomainError> {
        todo!()
    }
}
