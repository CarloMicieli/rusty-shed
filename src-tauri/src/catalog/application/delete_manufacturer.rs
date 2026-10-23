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

        let manufacturer = repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound {
                resource: "Manufacturer".to_string(),
                identifier: id.to_string(),
            })?;

        if manufacturer.is_system_seeded {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::application::testing::FakeUow;
    use crate::catalog::domain::manufacturer::{
        Manufacturer, ManufacturerStatus, MockManufacturerRepository,
    };
    use crate::core::domain::identifiers::Identifier;
    use crate::core::domain::metadata::Metadata;
    use mockall::predicate::eq;

    fn make_manufacturer(id: ManufacturerId, is_system_seeded: bool) -> Manufacturer {
        Manufacturer {
            id,
            name: "Test".to_string(),
            registered_company_name: None,
            country_code: None,
            status: ManufacturerStatus::Active,
            website_url: None,
            is_system_seeded,
            metadata: Metadata::default(),
        }
    }

    #[tokio::test]
    async fn delete_manufacturer_happy_path() -> Result<(), DomainError> {
        let id = ManufacturerId::new_from_parts(&["m1"]);
        let mut repo = MockManufacturerRepository::new();

        let id_for_find = id.clone();
        let m = make_manufacturer(id.clone(), false);
        repo.expect_find_by_id()
            .with(eq(id_for_find))
            .returning(move |_| Ok(Some(m.clone())));

        let id_for_usage = id.clone();
        repo.expect_find_usage_count()
            .with(eq(id_for_usage))
            .returning(|_| Ok(0));

        let id_for_delete = id.clone();
        repo.expect_delete_by_id()
            .with(eq(id_for_delete))
            .returning(|_| Ok(1));

        let mut uow = FakeUow::with_manufacturers_repo(repo);
        DeleteManufacturer::execute(&mut uow, &id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn delete_manufacturer_returns_not_found_when_missing() {
        let id = ManufacturerId::new_from_parts(&["missing"]);
        let mut repo = MockManufacturerRepository::new();

        repo.expect_find_by_id()
            .with(eq(id.clone()))
            .returning(|_| Ok(None));

        let mut uow = FakeUow::with_manufacturers_repo(repo);
        let result = DeleteManufacturer::execute(&mut uow, &id).await;

        assert!(matches!(result, Err(DomainError::NotFound { .. })));
    }

    #[tokio::test]
    async fn delete_manufacturer_rejects_seeded_entity() {
        let id = ManufacturerId::new_from_parts(&["seeded"]);
        let mut repo = MockManufacturerRepository::new();

        let m = make_manufacturer(id.clone(), true);
        repo.expect_find_by_id()
            .with(eq(id.clone()))
            .returning(move |_| Ok(Some(m.clone())));

        let mut uow = FakeUow::with_manufacturers_repo(repo);
        let result = DeleteManufacturer::execute(&mut uow, &id).await;

        assert!(matches!(result, Err(DomainError::BusinessRule(_))));
    }

    #[tokio::test]
    async fn delete_manufacturer_rejects_in_use_entity() {
        let id = ManufacturerId::new_from_parts(&["in-use"]);
        let mut repo = MockManufacturerRepository::new();

        let m = make_manufacturer(id.clone(), false);
        repo.expect_find_by_id()
            .with(eq(id.clone()))
            .returning(move |_| Ok(Some(m.clone())));

        let id_for_usage = id.clone();
        repo.expect_find_usage_count()
            .with(eq(id_for_usage))
            .returning(|_| Ok(2));

        let mut uow = FakeUow::with_manufacturers_repo(repo);
        let result = DeleteManufacturer::execute(&mut uow, &id).await;

        assert!(matches!(result, Err(DomainError::BusinessRule(_))));
    }

    #[tokio::test]
    async fn delete_manufacturer_returns_not_found_when_delete_affects_zero_rows() {
        let id = ManufacturerId::new_from_parts(&["gone"]);
        let mut repo = MockManufacturerRepository::new();

        let m = make_manufacturer(id.clone(), false);
        repo.expect_find_by_id()
            .with(eq(id.clone()))
            .returning(move |_| Ok(Some(m.clone())));

        let id_for_usage = id.clone();
        repo.expect_find_usage_count()
            .with(eq(id_for_usage))
            .returning(|_| Ok(0));

        let id_for_delete = id.clone();
        repo.expect_delete_by_id()
            .with(eq(id_for_delete))
            .returning(|_| Ok(0));

        let mut uow = FakeUow::with_manufacturers_repo(repo);
        let result = DeleteManufacturer::execute(&mut uow, &id).await;

        assert!(matches!(result, Err(DomainError::NotFound { .. })));
    }
}
