use crate::core::infrastructure::error::CommandError;
use crate::tracks_inventory::application::UpsertTrackProductTranslationInput;
use crate::tracks_inventory::domain::TrackProductUowExt;

/// Use case for creating or replacing one translation for a track product.
pub struct UpsertTrackProductTranslationUseCase;

impl UpsertTrackProductTranslationUseCase {
    /// Executes the use case to upsert one translation row.
    pub async fn execute(
        uow: &mut impl TrackProductUowExt,
        input: UpsertTrackProductTranslationInput,
    ) -> Result<(), CommandError> {
        let mut repo = uow.track_products_repo();

        repo.upsert_translation(&input.track_id, input.lang, input.description, input.details)
            .await?;

        Ok(())
    }
}
