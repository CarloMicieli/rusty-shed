use crate::catalog::domain::manufacturer::{ManufacturerId, ManufacturerUowExt};
use crate::core::domain::domain_error::DomainError;

/// Use case that merges two manufacturers by relinking models and removing the source.
pub struct MergeManufacturers;

impl MergeManufacturers {
    /// Merges `source_id` into `target_id` and returns the number of relinked models.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        source_id: &ManufacturerId,
        target_id: &ManufacturerId,
    ) -> Result<i64, DomainError>
    where
        U: ManufacturerUowExt + Send,
    {
        if source_id == target_id {
            return Err(DomainError::BusinessRule(
                "Source and target must be different".to_string(),
            ));
        }

        let mut repository = unit_of_work.manufacturers_repo();

        let source_seeded = repository
            .find_is_system_seeded(source_id)
            .await?
            .ok_or_else(|| DomainError::NotFound {
                resource: "Manufacturer".to_string(),
                identifier: source_id.to_string(),
            })?;

        let target_seeded = repository
            .find_is_system_seeded(target_id)
            .await?
            .ok_or_else(|| DomainError::NotFound {
                resource: "Manufacturer".to_string(),
                identifier: target_id.to_string(),
            })?;

        if source_seeded || target_seeded {
            return Err(DomainError::BusinessRule(
                "Protected entities cannot be merged".to_string(),
            ));
        }

        let relinked_count = repository
            .relink_railway_models(source_id, target_id)
            .await?;

        let deleted = repository.delete_by_id(source_id).await?;

        if deleted == 0 {
            return Err(DomainError::NotFound {
                resource: "Manufacturer".to_string(),
                identifier: source_id.to_string(),
            });
        }

        Ok(relinked_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::application::testing::FakeUow;
    use crate::catalog::domain::manufacturer::MockManufacturerRepository;
    use crate::core::domain::identifiers::Identifier;
    use mockall::predicate::eq;

    #[tokio::test]
    async fn merge_manufacturers_happy_path_returns_relinked_count() -> Result<(), DomainError> {
        let source_id = ManufacturerId::new_from_parts(&["source"]);
        let target_id = ManufacturerId::new_from_parts(&["target"]);
        let mut mock = MockManufacturerRepository::new();

        let source_id_for_seeded = source_id.clone();
        mock.expect_find_is_system_seeded()
            .with(eq(source_id_for_seeded))
            .returning(|_| Ok(Some(false)));

        let target_id_for_seeded = target_id.clone();
        mock.expect_find_is_system_seeded()
            .with(eq(target_id_for_seeded))
            .returning(|_| Ok(Some(false)));

        let source_id_for_relink = source_id.clone();
        let target_id_for_relink = target_id.clone();
        mock.expect_relink_railway_models()
            .with(eq(source_id_for_relink), eq(target_id_for_relink))
            .returning(|_, _| Ok(3));

        let source_id_for_delete = source_id.clone();
        mock.expect_delete_by_id()
            .with(eq(source_id_for_delete))
            .returning(|_| Ok(1));

        let mut uow = FakeUow::with_manufacturers_repo(mock);
        let result = MergeManufacturers::execute(&mut uow, &source_id, &target_id).await?;

        assert_eq!(result, 3);
        Ok(())
    }

    #[tokio::test]
    async fn merge_manufacturers_same_id_returns_business_rule() {
        let shared_id = ManufacturerId::new_from_parts(&["same"]);
        let mock = MockManufacturerRepository::new();
        let mut uow = FakeUow::with_manufacturers_repo(mock);

        let result = MergeManufacturers::execute(&mut uow, &shared_id, &shared_id).await;

        assert!(matches!(result, Err(DomainError::BusinessRule(_))));
    }

    #[tokio::test]
    async fn merge_manufacturers_missing_source_returns_not_found() {
        let source_id = ManufacturerId::new_from_parts(&["missing-source"]);
        let target_id = ManufacturerId::new_from_parts(&["target"]);
        let mut mock = MockManufacturerRepository::new();

        let source_id_for_seeded = source_id.clone();
        mock.expect_find_is_system_seeded()
            .with(eq(source_id_for_seeded))
            .returning(|_| Ok(None));

        let mut uow = FakeUow::with_manufacturers_repo(mock);
        let result = MergeManufacturers::execute(&mut uow, &source_id, &target_id).await;

        assert!(matches!(result, Err(DomainError::NotFound { .. })));
    }

    #[tokio::test]
    async fn merge_manufacturers_seeded_entity_returns_business_rule() {
        let source_id = ManufacturerId::new_from_parts(&["seeded-source"]);
        let target_id = ManufacturerId::new_from_parts(&["target"]);
        let mut mock = MockManufacturerRepository::new();

        let source_id_for_seeded = source_id.clone();
        mock.expect_find_is_system_seeded()
            .with(eq(source_id_for_seeded))
            .returning(|_| Ok(Some(true)));

        let target_id_for_seeded = target_id.clone();
        mock.expect_find_is_system_seeded()
            .with(eq(target_id_for_seeded))
            .returning(|_| Ok(Some(false)));

        let mut uow = FakeUow::with_manufacturers_repo(mock);
        let result = MergeManufacturers::execute(&mut uow, &source_id, &target_id).await;

        assert!(matches!(result, Err(DomainError::BusinessRule(_))));
    }

    #[tokio::test]
    async fn merge_manufacturers_delete_zero_rows_returns_not_found() {
        let source_id = ManufacturerId::new_from_parts(&["source"]);
        let target_id = ManufacturerId::new_from_parts(&["target"]);
        let mut mock = MockManufacturerRepository::new();

        let source_id_for_seeded = source_id.clone();
        mock.expect_find_is_system_seeded()
            .with(eq(source_id_for_seeded))
            .returning(|_| Ok(Some(false)));

        let target_id_for_seeded = target_id.clone();
        mock.expect_find_is_system_seeded()
            .with(eq(target_id_for_seeded))
            .returning(|_| Ok(Some(false)));

        let source_id_for_relink = source_id.clone();
        let target_id_for_relink = target_id.clone();
        mock.expect_relink_railway_models()
            .with(eq(source_id_for_relink), eq(target_id_for_relink))
            .returning(|_, _| Ok(0));

        let source_id_for_delete = source_id.clone();
        mock.expect_delete_by_id()
            .with(eq(source_id_for_delete))
            .returning(|_| Ok(0));

        let mut uow = FakeUow::with_manufacturers_repo(mock);
        let result = MergeManufacturers::execute(&mut uow, &source_id, &target_id).await;

        assert!(matches!(result, Err(DomainError::NotFound { .. })));
    }
}
