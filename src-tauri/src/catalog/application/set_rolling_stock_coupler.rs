use crate::catalog::domain::railway_model::{CouplerTypeId, CouplerUowExt};
use crate::collecting::domain::OwnedRollingStockId;
use crate::core::domain::domain_error::DomainError;
use crate::maintenance::domain::{MaintenanceType, MaintenanceUowExt};
use chrono::Local;
use uuid::Uuid;

/// Input for [`SetRollingStockCoupler::execute`].
pub struct SetRollingStockCouplerInput {
    /// The owned rolling stock to update.
    pub owned_rolling_stock_id: OwnedRollingStockId,
    /// The coupler to install, or `None` to clear the current coupler.
    pub coupler_type_id: Option<CouplerTypeId>,
}

/// Use-case that sets the installed coupler on an owned rolling stock.
///
/// After updating `current_coupler_id`, a `CouplerChange` maintenance event is
/// automatically recorded on the rolling stock's maintenance card (if one exists).
pub struct SetRollingStockCoupler;

impl SetRollingStockCoupler {
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: SetRollingStockCouplerInput,
    ) -> Result<(), DomainError>
    where
        U: CouplerUowExt + MaintenanceUowExt + Send,
    {
        // 1. Persist the coupler assignment.
        {
            let mut coupler_repo = unit_of_work.coupler_repository();
            coupler_repo
                .set_current_coupler(&input.owned_rolling_stock_id, input.coupler_type_id)
                .await?;
        }

        // 2. Auto-record a maintenance event if a card exists.
        {
            let mut maintenance_repo = unit_of_work.maintenance_repository();
            if let Some(mut card) = maintenance_repo
                .find_by_rolling_stock_id(&input.owned_rolling_stock_id)
                .await?
            {
                let event_uuid = Uuid::new_v4();
                let today = Local::now().date_naive();
                card.record_maintenance(
                    event_uuid,
                    today,
                    Some(MaintenanceType::CouplerChange),
                    None,
                );
                maintenance_repo.save(card).await?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::domain::railway_model::{
        CouplerRepository, CouplerTypeId, CouplerUowExt, MockCouplerRepository,
    };
    use crate::core::domain::domain_error::DomainError;
    use crate::core::domain::identifiers::Identifier;
    use crate::maintenance::domain::{
        MaintenanceCard, MaintenanceRepository, MaintenanceUowExt, MockMaintenanceRepository,
    };

    /// Local fake UoW that composes coupler + maintenance repos only.
    struct FakeCouplerMaintenanceUow {
        coupler: Option<MockCouplerRepository>,
        maintenance: Option<MockMaintenanceRepository>,
    }

    impl FakeCouplerMaintenanceUow {
        fn new(coupler: MockCouplerRepository, maintenance: MockMaintenanceRepository) -> Self {
            Self {
                coupler: Some(coupler),
                maintenance: Some(maintenance),
            }
        }
    }

    impl CouplerUowExt for FakeCouplerMaintenanceUow {
        fn coupler_repository(&mut self) -> Box<dyn CouplerRepository + '_> {
            Box::new(
                self.coupler
                    .take()
                    .expect("coupler repository already taken"),
            )
        }
    }

    impl MaintenanceUowExt for FakeCouplerMaintenanceUow {
        fn maintenance_repository(&mut self) -> Box<dyn MaintenanceRepository + Send + '_> {
            Box::new(
                self.maintenance
                    .take()
                    .expect("maintenance repository already taken"),
            )
        }
    }

    fn owned_rs_id() -> OwnedRollingStockId {
        OwnedRollingStockId::from_uuid(&uuid::Uuid::new_v4())
    }

    #[tokio::test]
    async fn it_sets_coupler_without_maintenance_card() {
        let id = owned_rs_id();

        let mut coupler_mock = MockCouplerRepository::new();
        coupler_mock
            .expect_set_current_coupler()
            .times(1)
            .returning(|_, _| Ok(()));

        let mut maintenance_mock = MockMaintenanceRepository::new();
        maintenance_mock
            .expect_find_by_rolling_stock_id()
            .times(1)
            .returning(|_| Ok(None));
        maintenance_mock.expect_save().times(0);

        let mut uow = FakeCouplerMaintenanceUow::new(coupler_mock, maintenance_mock);
        SetRollingStockCoupler::execute(
            &mut uow,
            SetRollingStockCouplerInput {
                owned_rolling_stock_id: id,
                coupler_type_id: None,
            },
        )
        .await
        .expect("should succeed when no maintenance card");
    }

    #[tokio::test]
    async fn it_sets_coupler_and_records_maintenance_event() {
        let id = owned_rs_id();
        let card = MaintenanceCard::from_id(uuid::Uuid::new_v4());

        let mut coupler_mock = MockCouplerRepository::new();
        coupler_mock
            .expect_set_current_coupler()
            .times(1)
            .returning(|_, _| Ok(()));

        let mut maintenance_mock = MockMaintenanceRepository::new();
        maintenance_mock
            .expect_find_by_rolling_stock_id()
            .times(1)
            .returning(move |_| Ok(Some(card.clone())));
        maintenance_mock
            .expect_save()
            .times(1)
            .returning(|_| Ok(()));

        let mut uow = FakeCouplerMaintenanceUow::new(coupler_mock, maintenance_mock);
        SetRollingStockCoupler::execute(
            &mut uow,
            SetRollingStockCouplerInput {
                owned_rolling_stock_id: id,
                coupler_type_id: Some(CouplerTypeId::from_string_unchecked(
                    "trn:coupler:roco:universal".to_string(),
                )),
            },
        )
        .await
        .expect("should succeed and record maintenance event");
    }

    #[tokio::test]
    async fn it_returns_error_when_coupler_set_fails() {
        let id = owned_rs_id();

        let mut coupler_mock = MockCouplerRepository::new();
        coupler_mock
            .expect_set_current_coupler()
            .times(1)
            .returning(|_, _| {
                Err(DomainError::Infrastructure(
                    "db connection lost".to_string(),
                ))
            });

        let maintenance_mock = MockMaintenanceRepository::new();

        let mut uow = FakeCouplerMaintenanceUow::new(coupler_mock, maintenance_mock);
        let err = SetRollingStockCoupler::execute(
            &mut uow,
            SetRollingStockCouplerInput {
                owned_rolling_stock_id: id,
                coupler_type_id: None,
            },
        )
        .await
        .expect_err("coupler persistence failure should propagate");

        assert!(
            matches!(err, DomainError::Infrastructure(_)),
            "expected Infrastructure error, got {err:?}"
        );
    }
}
