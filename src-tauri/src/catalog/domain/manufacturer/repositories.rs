use crate::catalog::domain::manufacturer::{Manufacturer, ManufacturerId};
use crate::core::domain::domain_error::DomainError;

/// Repository trait for managing Manufacturers data.
#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait ManufacturerRepository: Send + Sync {
    /// Retrieves all Railway Model manufacturers from the database.
    ///
    /// # Arguments
    /// none
    ///
    /// # Returns
    /// - Returns the list of all [`Manufacturer`]s in the database.
    /// - Returns a [`DomainError::DatabaseError`] if the query fails.
    async fn find_all(&mut self) -> Result<Vec<Manufacturer>, DomainError>;

    /// Fetch a manufacturer row by its ID.
    ///
    /// This function executes a simple SELECT query against the `manufacturers` table
    /// and returns the matching `ManufacturerRow` if present.
    ///
    /// # Arguments
    /// - `id` - The manufacturer identifier to look up.
    ///
    /// # Returns
    /// - `Ok(Some(ManufacturerRow))` when a matching row is found
    /// - `Ok(None)` when no row matches the provided `id`
    /// - `Err(DomainError)` if the query fails.
    async fn find_by_id(
        &mut self,
        id: &ManufacturerId,
    ) -> Result<Option<Manufacturer>, DomainError>;

    /// Returns whether a manufacturer is protected/system-seeded.
    ///
    /// # Returns
    /// - `Ok(Some(true))` when the manufacturer exists and is protected.
    /// - `Ok(Some(false))` when the manufacturer exists and is editable.
    /// - `Ok(None)` when no manufacturer is found for the given id.
    /// - `Err(DomainError)` when persistence access fails.
    async fn find_is_system_seeded(
        &mut self,
        id: &ManufacturerId,
    ) -> Result<Option<bool>, DomainError>;

    /// Returns the `(name, is_system_seeded)` pair for the given manufacturer,
    /// or `None` when no matching row exists.
    ///
    /// Used to enforce the seeded-name-immutability business rule before
    /// mutating a manufacturer without loading the full aggregate.
    async fn find_seeded_and_name(
        &mut self,
        id: &ManufacturerId,
    ) -> Result<Option<(String, bool)>, DomainError>;

    /// Relinks all railway models currently referencing `source_id` so they
    /// reference `target_id` instead.
    ///
    /// Returns the number of affected rows.
    async fn relink_railway_models(
        &mut self,
        source_id: &ManufacturerId,
        target_id: &ManufacturerId,
    ) -> Result<i64, DomainError>;

    /// Deletes a manufacturer row by identifier.
    ///
    /// Returns the number of deleted rows.
    async fn delete_by_id(&mut self, id: &ManufacturerId) -> Result<u64, DomainError>;

    /// Returns the number of railway models that reference this manufacturer.
    ///
    /// Used to populate the `usage_count` field in the view DTO and to guard
    /// against deleting a manufacturer that is still in use.
    async fn find_usage_count(&mut self, id: &ManufacturerId) -> Result<i64, DomainError>;

    /// Inserts a new manufacturer row and returns the persisted aggregate.
    ///
    /// Returns `Err(DomainError::Conflict)` when a manufacturer with a
    /// conflicting unique key already exists.
    async fn insert(
        &mut self,
        id: &ManufacturerId,
        name: String,
        country_code: Option<String>,
        website_url: Option<String>,
    ) -> Result<Manufacturer, DomainError>;

    /// Updates an existing manufacturer row and returns the updated aggregate.
    ///
    /// Returns `Err(DomainError::Conflict)` on a unique-key collision.
    async fn update(
        &mut self,
        id: &ManufacturerId,
        name: String,
        country_code: Option<String>,
        website_url: Option<String>,
    ) -> Result<Manufacturer, DomainError>;
}

/// An extension trait that provides access to the `ManufacturerRepository`.
///
/// This follows the **Interface Segregation Principle**. By using extension traits,
/// we avoid a "God Object" where one struct knows about every repository in the
/// system. Instead, repositories are grouped by domain logic.
pub trait ManufacturerUowExt: Send {
    /// Returns a trait object for interacting with manufacturers model data.
    ///
    /// The repository is bound to the lifetime of the Unit of Work to ensure
    /// it cannot outlive the transaction it relies on.
    fn manufacturers_repo(&mut self) -> Box<dyn ManufacturerRepository + '_>;
}
