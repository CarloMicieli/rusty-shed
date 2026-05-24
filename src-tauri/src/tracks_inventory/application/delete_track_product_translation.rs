use crate::core::infrastructure::error::CommandError;
use crate::tracks_inventory::application::DeleteTrackProductTranslationInput;
use crate::tracks_inventory::domain::TrackProductUowExt;

/// Use case for deleting one translation for a track product.
pub struct DeleteTrackProductTranslationUseCase;

impl DeleteTrackProductTranslationUseCase {
    /// Executes the use case to delete one translation row.
    pub async fn execute(
        uow: &mut impl TrackProductUowExt,
        input: DeleteTrackProductTranslationInput,
    ) -> Result<(), CommandError> {
        let mut repo = uow.track_products_repo();

        repo.delete_translation(&input.track_id, input.lang).await?;

        Ok(())
    }
}
