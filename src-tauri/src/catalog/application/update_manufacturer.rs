use crate::catalog::domain::manufacturer::{Manufacturer, ManufacturerId, ManufacturerUowExt};
use crate::core::domain::domain_error::DomainError;

/// Input data required to update an existing manufacturer.
pub struct UpdateManufacturerInput {
    /// Identifier of the manufacturer to update.
    pub id: ManufacturerId,
    /// The new display name (already validated and trimmed by the caller).
    pub name: String,
    /// Optional ISO 3166-1 alpha-2 country code (already upper-cased).
    pub country_code: Option<String>,
    /// Optional website URL as a string (already validated by the caller).
    pub website_url: Option<String>,
}

/// Use case that updates an existing manufacturer.
pub struct UpdateManufacturer;

impl UpdateManufacturer {
    /// Enforces the seeded-name-immutability business rule, then persists the
    /// update and returns the updated [`Manufacturer`] aggregate.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: UpdateManufacturerInput,
    ) -> Result<Manufacturer, DomainError>
    where
        U: ManufacturerUowExt + Send,
    {
        let mut repo = unit_of_work.manufacturers_repo();

        let (current_name, is_seeded) =
            repo.find_seeded_and_name(&input.id)
                .await?
                .ok_or_else(|| DomainError::NotFound {
                    resource: "Manufacturer".to_string(),
                    identifier: input.id.to_string(),
                })?;

        if is_seeded && current_name.trim() != input.name.trim() {
            return Err(DomainError::BusinessRule(
                "System-seeded manufacturer names cannot be edited".to_string(),
            ));
        }

        repo.update(&input.id, input.name, input.country_code, input.website_url)
            .await
    }
}
