//! Repository trait for [`Prototype`] persistence and querying.

use crate::core::domain::domain_error::DomainError;
use crate::trains::domain::prototype::prototype::Prototype;

/// Async trait for persisting and retrieving [`Prototype`] entities.
#[async_trait::async_trait]
pub trait PrototypeRepository {
    /// Fetch a prototype by its unique ID.
    ///
    /// Returns [`DomainError::NotFound`] when no matching record exists.
    async fn find_by_id(&mut self, id: &str) -> Result<Prototype, DomainError>;

    /// Search prototypes, optionally filtering by a query string matched
    /// against `series_code` and `car_type`.
    async fn search(&mut self, query: Option<&str>) -> Result<Vec<Prototype>, DomainError>;

    /// Persist a prototype (INSERT on new, UPDATE on existing).
    async fn save(&mut self, prototype: &Prototype) -> Result<(), DomainError>;

    /// Return all prototypes grouped by railway company.
    async fn find_all_grouped(&mut self) -> Result<Vec<(String, Vec<Prototype>)>, DomainError>;
}
