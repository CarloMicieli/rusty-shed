use crate::catalog::domain::manufacturer::ManufacturerId;
use crate::core::domain::id_provider::IdProvider;
use crate::core::domain::length::Length;
use crate::core::domain::metadata::Metadata;
use crate::core::infrastructure::error::CommandError;
use crate::tracks_inventory::application::CreateTrackProductInput;
use crate::tracks_inventory::domain::{
    TrackCode, TrackId, TrackProduct, TrackProductUowExt, TrackType,
};

/// Use case for creating a new track product in the catalog.
pub struct CreateTrackProductUseCase;

impl CreateTrackProductUseCase {
    /// Executes the use case to create a new track product.
    ///
    /// # Arguments
    /// - `uow`: The unit of work providing access to the track product repository.
    /// - `id_provider`: Provides unique identifiers for new entities.
    /// - `input`: The input data required to create the track product.
    ///
    /// # Returns
    /// The `TrackId` of the newly created track product.
    pub async fn execute(
        uow: &mut impl TrackProductUowExt,
        id_provider: impl IdProvider<String>,
        input: CreateTrackProductInput,
    ) -> Result<TrackId, CommandError> {
        let track_id = TrackId(id_provider.next_id());

        let track_product = TrackProduct {
            track_id: track_id.clone(),
            product_code: input.product_code,
            manufacturer_id: input.manufacturer_id,
            description: input.description,
            with_roadbed: input.with_roadbed,
            length: input.length,
            radius: input.radius,
            track_type: input.track_type,
            track_code: input.track_code,
            metadata: Metadata::default(),
        };

        uow.track_products_repo().save(track_product).await?;

        Ok(track_id)
    }
}
