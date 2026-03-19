use crate::catalog::domain::railway_model::RailwayModelView;
use crate::catalog::domain::railway_model::railway_model_translation::RailwayModelTranslations;
use crate::catalog::domain::railway_model::{RailwayModel, RailwayModelId, RailwayModelParams};
use crate::core::domain::domain_error::DomainError;

/// A domain-agnostic interface for railway models data access.
///
/// This trait abstracts away the database engine (SQLite, Postgres, etc.),
/// allowing the business logic to remain decoupled from infrastructure details.
#[cfg_attr(test, mockall::automock)]
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

    /// Retrieves a Railway Model aggregate by its unique identifier with language-aware text.
    ///
    /// # Arguments
    /// * `id` - The unique identifier of the Railway Model to retrieve.
    /// * `lang` - The preferred language code ("en" or "it"). Falls back to "en" if unavailable.
    ///
    /// # Returns
    /// * Returns `Ok(Some(RailwayModel))` if found.
    /// * Returns `Ok(None)` if no Railway Model with the given ID exists.
    /// * Returns a [`DomainError::DatabaseError`] if the query fails.
    async fn find_by_id(
        &mut self,
        id: &RailwayModelId,
        lang: &str,
    ) -> Result<Option<RailwayModel>, DomainError>;

    /// Retrieves a UI-focused view of a Railway Model by id with language-aware text.
    /// This method is intended for read-only scenarios where the frontend needs a
    /// serialized view without domain-only metadata.
    ///
    /// # Arguments
    /// * `id` - The unique identifier of the Railway Model.
    /// * `lang` - The preferred language code ("en" or "it"). Falls back to "en" if unavailable.
    async fn find_view_by_id(
        &mut self,
        id: &RailwayModelId,
        lang: &str,
    ) -> Result<Option<RailwayModelView>, DomainError>;

    /// Retrieves all stored translations for a railway model (used by the edit form).
    ///
    /// # Arguments
    /// * `id` - The unique identifier of the Railway Model.
    ///
    /// # Returns
    /// * Returns `Ok(Some(RailwayModelTranslations))` if the model exists with translations.
    /// * Returns `Ok(None)` if the model does not exist.
    async fn find_translations(
        &mut self,
        id: &RailwayModelId,
    ) -> Result<Option<RailwayModelTranslations>, DomainError>;

    /// Persists changes from a `RailwayModel` aggregate by applying its
    /// pending domain events to storage. Implementations should pull events
    /// from the aggregate and map them to appropriate SQL statements.
    async fn save(&mut self, aggregate: &mut RailwayModel) -> Result<(), DomainError>;

    /// Check whether a railway model with the given ID exists in the database.
    ///
    /// Cheaper than `find_by_id` — uses a single-table query with no JOINs.
    async fn exists_by_id(&mut self, id: &RailwayModelId) -> Result<bool, DomainError>;

    /// Search for railway models using FTS5 full-text search across all languages.
    ///
    /// # Arguments
    /// * `query` - The search query (minimum 2 characters).
    ///
    /// # Returns
    /// * Returns a list of matching `RailwayModelId`s ordered by relevance.
    async fn search(&mut self, query: &str) -> Result<Vec<RailwayModelId>, DomainError>;
}

/// An extension trait that provides access to the `RailwayModelRepository`.
///
/// This follows the **Interface Segregation Principle**. By using extension traits,
/// we avoid a "God Object" where one struct knows about every repository in the
/// system. Instead, repositories are grouped by domain logic.
pub trait RailwayModelUowExt: Send {
    /// Returns a trait object for interacting with railway model data.
    ///
    /// The repository is bound to the lifetime of the Unit of Work to ensure
    /// it cannot outlive the transaction it relies on.
    fn railway_model_repository(&mut self) -> Box<dyn RailwayModelRepository + '_>;
}
