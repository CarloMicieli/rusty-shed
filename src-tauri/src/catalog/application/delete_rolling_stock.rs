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
