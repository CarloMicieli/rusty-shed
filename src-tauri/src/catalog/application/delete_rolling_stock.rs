use crate::catalog::domain::railway_model::{RailwayModelId, RailwayModelUowExt, RollingStockId};
use crate::core::domain::Language;
use crate::core::domain::domain_error::DomainError;

/// Input for [`DeleteRollingStock::execute`].
pub struct DeleteRollingStockInput {
    /// The parent railway model.
    pub railway_model_id: RailwayModelId,
    /// The rolling stock unit to delete.
    pub rolling_stock_id: RollingStockId,
}

/// Use case that removes a rolling stock unit from an existing [`RailwayModel`] aggregate.
pub struct DeleteRollingStock;

impl DeleteRollingStock {
    /// Execute the use case.
    ///
    /// # Errors
    /// - [`DomainError::NotFound`] when no railway model with the given id exists.
    /// - [`DomainError::Infrastructure`] on database failure.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: DeleteRollingStockInput,
    ) -> Result<(), DomainError>
    where
        U: RailwayModelUowExt + Send,
    {
        let mut repo = unit_of_work.railway_model_repository();

        let mut model = repo
            .find_by_id(&input.railway_model_id, Language::English)
            .await?
            .ok_or_else(|| DomainError::NotFound {
                resource: "RailwayModel".to_string(),
                identifier: input.railway_model_id.to_string(),
            })?;

        model.remove_rolling_stock(&input.rolling_stock_id);
        repo.save(&mut model).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::application::testing::FakeUow;
    use crate::catalog::domain::manufacturer::ManufacturerId;
    use crate::catalog::domain::railway_model::{MockRailwayModelRepository, RailwayModel, RollingStock};
    use fake::{Fake, Faker};

    fn model_id() -> RailwayModelId {
        RailwayModelId::new(
            &ManufacturerId::try_from("trn:manufacturer:acme").expect("valid manufacturer id"),
            "P100",
        )
        .expect("valid model id")
    }

    #[tokio::test]
    async fn removes_rolling_stock_and_saves_model() {
        let railway_model_id = model_id();
        let rolling_stock_id = RollingStockId::from_uuid(&uuid::Uuid::new_v4());
        let mut model: RailwayModel = Faker.fake();
        model.id = railway_model_id.clone();
        model.rolling_stocks = vec![RollingStock::Locomotive {
            id: rolling_stock_id.clone(),
            ..Faker.fake()
        }];

        let mut repo = MockRailwayModelRepository::new();
        repo.expect_find_by_id()
            .times(1)
            .return_once(move |_, _| Ok(Some(model)));
        repo.expect_save().times(1).returning(|_| Ok(()));

        let mut uow = FakeUow::with_railway_models_repo(repo);
        let result = DeleteRollingStock::execute(
            &mut uow,
            DeleteRollingStockInput {
                railway_model_id,
                rolling_stock_id,
            },
        )
        .await;

        assert!(result.is_ok(), "{result:?}");
    }

    #[tokio::test]
    async fn returns_not_found_when_model_is_missing() {
        let railway_model_id = model_id();
        let rolling_stock_id = RollingStockId::from_uuid(&uuid::Uuid::new_v4());

        let mut repo = MockRailwayModelRepository::new();
        repo.expect_find_by_id().times(1).returning(|_, _| Ok(None));
        repo.expect_save().times(0);

        let mut uow = FakeUow::with_railway_models_repo(repo);
        let result = DeleteRollingStock::execute(
            &mut uow,
            DeleteRollingStockInput {
                railway_model_id,
                rolling_stock_id,
            },
        )
        .await;

        assert!(matches!(result, Err(DomainError::NotFound { .. })));
    }
}
