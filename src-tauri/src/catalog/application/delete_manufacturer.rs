use crate::catalog::domain::manufacturer::{ManufacturerId, ManufacturerUowExt};
use crate::core::domain::domain_error::DomainError;

/// Use case that deletes a manufacturer after verifying it is neither
/// system-seeded nor still referenced by any railway models.
pub struct DeleteManufacturer;

impl DeleteManufacturer {
    /// Checks business-rule guards and then deletes the manufacturer row.
    pub async fn execute<U>(unit_of_work: &mut U, id: &ManufacturerId) -> Result<(), DomainError>
    where
        U: ManufacturerUowExt + Send,
    {
        let mut repo = unit_of_work.manufacturers_repo();

        let is_seeded =
            repo.find_is_system_seeded(id)
                .await?
                .ok_or_else(|| DomainError::NotFound {
                    resource: "Manufacturer".to_string(),
                    identifier: id.to_string(),
                })?;

        if is_seeded {
            return Err(DomainError::BusinessRule(
                "Protected entity cannot be deleted".to_string(),
            ));
        }

        let usage_count = repo.find_usage_count(id).await?;
        if usage_count > 0 {
            return Err(DomainError::BusinessRule(format!(
                "Entity is still in use ({usage_count})"
            )));
        }

        let affected = repo.delete_by_id(id).await?;
        if affected == 0 {
            return Err(DomainError::NotFound {
                resource: "Manufacturer".to_string(),
                identifier: id.to_string(),
            });
        }

        Ok(())
    }
}
