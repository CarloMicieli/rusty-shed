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
            with_roadbed: input.with_roadbed,
            length: input.length,
            radius: input.radius,
            track_type: input.track_type,
            track_code: input.track_code,
            metadata: Metadata::default(),
        };

        {
            let mut repo = uow.track_products_repo();
            repo.insert_track(&track_product).await?;
            repo.upsert_translation(
                &track_id,
                input.lang,
                input.description,
                input.details,
            )
            .await?;
        }

        Ok(track_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_uow::testing::{MockAppUow, OneShotFactory};
    use crate::app_uow::{AppUnitOfWork, AppUowFactory};
    use crate::catalog::domain::manufacturer::ManufacturerId;
    use crate::core::domain::identifiers::Identifier;
    use crate::core::domain::test_utils::MockIdProvider;
    use crate::tracks_inventory::domain::{MockTrackProductRepository, TrackCode, TrackType};

    #[tokio::test]
    async fn it_creates_track_product_and_returns_id() {
        let raw_id = "trn:track:acme:r1".to_string();
        let expected_track_id = TrackId(raw_id.clone());

        let mut repo = MockTrackProductRepository::new();
        repo.expect_insert_track().times(1).returning(|_| Ok(()));
        repo.expect_upsert_translation()
            .times(1)
            .returning(|_, _, _, _| Ok(()));

        let uow = MockAppUow::new().with_track_product(repo);
        let factory = OneShotFactory::new(uow);
        let mut uow_box: Box<dyn AppUnitOfWork> = factory.create_uow().await.unwrap();

        let id_provider = MockIdProvider::new(raw_id);
        let input = CreateTrackProductInput {
            manufacturer_id: ManufacturerId::new_from_parts(&["acme"]),
            lang: crate::core::domain::Language::English,
            product_code: "R1".to_string(),
            description: Some("Standard straight track".to_string()),
            details: None,
            track_type: TrackType::Straight,
            track_code: TrackCode::Code100,
            with_roadbed: false,
            length: None,
            radius: None,
        };

        let result = CreateTrackProductUseCase::execute(&mut uow_box, id_provider, input).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_track_id);
    }
}
