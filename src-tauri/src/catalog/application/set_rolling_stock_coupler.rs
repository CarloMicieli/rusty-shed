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
