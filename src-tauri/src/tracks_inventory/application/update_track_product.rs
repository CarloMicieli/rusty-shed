use crate::core::domain::domain_error::DomainError;
use crate::core::domain::metadata::Metadata;
use crate::core::infrastructure::error::CommandError;
use crate::tracks_inventory::application::UpdateTrackProductInput;
use crate::tracks_inventory::domain::{TrackProduct, TrackProductUowExt};

/// Use case for updating an existing track product in the catalog.
pub struct UpdateTrackProductUseCase;

impl UpdateTrackProductUseCase {
    /// Executes the use case to update a track product.
    pub async fn execute(
        uow: &mut impl TrackProductUowExt,
        input: UpdateTrackProductInput,
    ) -> Result<(), CommandError> {
        let mut repo = uow.track_products_repo();

        let existing = repo.find_by_id(&input.track_id).await?;
        if existing.is_none() {
            return Err(CommandError::from(DomainError::NotFound {
                resource: "TrackProduct".to_string(),
                identifier: input.track_id.to_string(),
            }));
        }

        let track_product = TrackProduct {
            track_id: input.track_id,
            product_code: input.product_code,
            manufacturer_id: input.manufacturer_id,
            with_roadbed: input.with_roadbed,
            length: input.length,
            radius: input.radius,
            track_type: input.track_type,
            track_code: input.track_code,
            metadata: Metadata::default(),
        };

        repo.update_track(&track_product).await?;

        Ok(())
    }
}
