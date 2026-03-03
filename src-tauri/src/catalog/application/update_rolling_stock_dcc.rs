use crate::catalog::domain::railway_model::{
    Control, DccInterface, LengthOverBuffers, RailwayModelId, RailwayModelUowExt,
    RollingStockDccPatch, RollingStockId,
};
use crate::core::domain::domain_error::DomainError;

/// Input for [`UpdateRollingStockDcc::execute`].
pub struct UpdateRollingStockDccInput {
    /// The parent railway model.
    pub railway_model_id: RailwayModelId,
    /// The rolling stock unit to update.
    pub rolling_stock_id: RollingStockId,
    /// Optional control type; `None` clears the field.
    pub control: Option<Control>,
    /// Optional DCC interface connector; `None` clears the field.
    pub dcc_interface: Option<DccInterface>,
    /// Optional length over buffers; `None` clears the field.
    pub length_over_buffers: Option<LengthOverBuffers>,
}

/// Use case that updates only the control type, DCC interface, and length of a single
/// rolling stock unit without touching any other technical specification fields.
pub struct UpdateRollingStockDcc;

impl UpdateRollingStockDcc {
    /// Execute the use case.
    ///
    /// # Errors
    /// - [`DomainError::NotFound`] when no railway model with the given id exists.
    /// - [`DomainError::NotFound`] when no rolling stock with `rolling_stock_id` exists.
    /// - [`DomainError::Infrastructure`] on database failure.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: UpdateRollingStockDccInput,
    ) -> Result<(), DomainError>
    where
        U: RailwayModelUowExt + Send,
    {
        let mut repo = unit_of_work.railway_model_repository();

        let mut model = repo
            .find_by_id(&input.railway_model_id, "en")
            .await?
            .ok_or_else(|| DomainError::NotFound {
                resource: "RailwayModel".to_string(),
                identifier: input.railway_model_id.to_string(),
            })?;

        model.update_rolling_stock_dcc(
            &input.rolling_stock_id,
            RollingStockDccPatch {
                control: input.control,
                dcc_interface: input.dcc_interface,
                length_over_buffers: input.length_over_buffers,
            },
        )?;

        repo.save(&mut model).await
    }
}
