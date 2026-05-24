use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::error::CommandError;
use crate::tracks_inventory::application::DeleteTrackProductInput;
use crate::tracks_inventory::domain::TrackProductUowExt;

/// Use case for deleting an existing track product.
pub struct DeleteTrackProductUseCase;

impl DeleteTrackProductUseCase {
    /// Executes the use case to delete a track product.
    pub async fn execute(
        uow: &mut impl TrackProductUowExt,
        input: DeleteTrackProductInput,
    ) -> Result<(), CommandError> {
        let mut repo = uow.track_products_repo();

        let existing = repo.find_by_id(&input.track_id).await?;
        if existing.is_none() {
            return Err(CommandError::from(DomainError::NotFound {
                resource: "TrackProduct".to_string(),
                identifier: input.track_id.to_string(),
            }));
        }

        repo.delete_track(&input.track_id).await?;

        Ok(())
    }
}
