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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::domain::manufacturer::ManufacturerId;
    use crate::core::domain::metadata::Metadata;
    use crate::tracks_inventory::domain::{
        MockTrackProductRepository, TrackCode, TrackId, TrackProduct, TrackProductRepository,
        TrackProductUowExt, TrackType,
    };

    struct FakeTrackProductUow {
        repo: Option<MockTrackProductRepository>,
    }

    impl FakeTrackProductUow {
        fn new(repo: MockTrackProductRepository) -> Self {
            Self { repo: Some(repo) }
        }
    }

    impl TrackProductUowExt for FakeTrackProductUow {
        fn track_products_repo(&mut self) -> Box<dyn TrackProductRepository + '_> {
            Box::new(
                self.repo
                    .take()
                    .expect("test setup error: repository already taken"),
            )
        }
    }

    fn sample_track_product() -> TrackProduct {
        TrackProduct {
            track_id: TrackId::try_from("trn:track:acme:60100").expect("valid track id"),
            product_code: "60100".to_string(),
            manufacturer_id: ManufacturerId::try_from("trn:manufacturer:acme")
                .expect("valid manufacturer id"),
            with_roadbed: false,
            length: None,
            radius: None,
            track_type: TrackType::Straight,
            track_code: TrackCode::Code83,
            metadata: Metadata::default(),
        }
    }

    #[tokio::test]
    async fn execute_returns_not_found_when_track_does_not_exist() {
        let track_id = TrackId::try_from("trn:track:acme:60100").expect("valid track id");

        let mut repo = MockTrackProductRepository::new();
        repo.expect_find_by_id().once().returning(|_| Ok(None));
        repo.expect_delete_track().never();

        let mut uow = FakeTrackProductUow::new(repo);
        let input = DeleteTrackProductInput { track_id };

        let result = DeleteTrackProductUseCase::execute(&mut uow, input).await;

        assert!(
            matches!(result, Err(CommandError::NotFound(_))),
            "{result:?}"
        );
    }

    #[tokio::test]
    async fn execute_deletes_track_when_it_exists() {
        let track_id = TrackId::try_from("trn:track:acme:60100").expect("valid track id");
        let expected_track_id = track_id.clone();

        let mut repo = MockTrackProductRepository::new();
        repo.expect_find_by_id()
            .once()
            .returning(|_| Ok(Some(sample_track_product())));
        repo.expect_delete_track()
            .once()
            .withf(move |id| id == &expected_track_id)
            .returning(|_| Ok(()));

        let mut uow = FakeTrackProductUow::new(repo);
        let input = DeleteTrackProductInput { track_id };

        let result = DeleteTrackProductUseCase::execute(&mut uow, input).await;

        assert!(result.is_ok(), "{result:?}");
    }

    #[tokio::test]
    async fn execute_maps_repository_delete_error_to_command_error() {
        let track_id = TrackId::try_from("trn:track:acme:60100").expect("valid track id");

        let mut repo = MockTrackProductRepository::new();
        repo.expect_find_by_id()
            .once()
            .returning(|_| Ok(Some(sample_track_product())));
        repo.expect_delete_track()
            .once()
            .returning(|_| Err(DomainError::Infrastructure("db write failed".to_string())));

        let mut uow = FakeTrackProductUow::new(repo);
        let input = DeleteTrackProductInput { track_id };

        let result = DeleteTrackProductUseCase::execute(&mut uow, input).await;

        assert!(
            matches!(result, Err(CommandError::DatabaseError(ref msg)) if msg.contains("db write failed")),
            "{result:?}"
        );
    }
}
