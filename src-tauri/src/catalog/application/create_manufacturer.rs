use crate::catalog::domain::manufacturer::{Manufacturer, ManufacturerId, ManufacturerUowExt};
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::identifiers::Identifier;

/// Input data required to create a new manufacturer.
pub struct CreateManufacturerInput {
    /// The display name (already validated and trimmed by the caller).
    pub name: String,
    /// Optional ISO 3166-1 alpha-2 country code (already upper-cased).
    pub country_code: Option<String>,
    /// Optional website URL as a string (already validated by the caller).
    pub website_url: Option<String>,
}

/// Use case that creates a new manufacturer and persists it.
pub struct CreateManufacturer;

impl CreateManufacturer {
    /// Generates a stable identifier from `name`, inserts the row, and returns
    /// the persisted [`Manufacturer`] aggregate.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: CreateManufacturerInput,
    ) -> Result<Manufacturer, DomainError>
    where
        U: ManufacturerUowExt + Send,
    {
        let id = ManufacturerId::new_from_parts(&[input.name.as_str()]);
        let mut repo = unit_of_work.manufacturers_repo();
        repo.insert(&id, input.name, input.country_code, input.website_url)
            .await
    }
}
