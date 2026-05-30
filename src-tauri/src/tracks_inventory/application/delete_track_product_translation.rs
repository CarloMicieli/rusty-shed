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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::Language;
    use crate::core::domain::domain_error::DomainError;
    use crate::tracks_inventory::domain::{
        MockTrackProductRepository, TrackId, TrackProductRepository, TrackProductUowExt,
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

    #[tokio::test]
    async fn execute_deletes_translation_with_requested_language() {
        let track_id = TrackId::try_from("trn:track:acme:60100").expect("valid track id");
        let expected_track_id = track_id.clone();

        let mut repo = MockTrackProductRepository::new();
        repo.expect_delete_translation()
            .once()
            .withf(move |id, lang| id == &expected_track_id && *lang == Language::Italian)
            .returning(|_, _| Ok(()));

        let mut uow = FakeTrackProductUow::new(repo);
        let input = DeleteTrackProductTranslationInput {
            track_id,
            lang: Language::Italian,
        };

        let result = DeleteTrackProductTranslationUseCase::execute(&mut uow, input).await;

        assert!(result.is_ok(), "{result:?}");
    }

    #[tokio::test]
    async fn execute_maps_repository_error_to_command_error() {
        let track_id = TrackId::try_from("trn:track:acme:60100").expect("valid track id");

        let mut repo = MockTrackProductRepository::new();
        repo.expect_delete_translation()
            .once()
            .returning(|_, _| Err(DomainError::Infrastructure("db delete failed".to_string())));

        let mut uow = FakeTrackProductUow::new(repo);
        let input = DeleteTrackProductTranslationInput {
            track_id,
            lang: Language::English,
        };

        let result = DeleteTrackProductTranslationUseCase::execute(&mut uow, input).await;

        assert!(
            matches!(result, Err(CommandError::DatabaseError(ref msg)) if msg.contains("db delete failed")),
            "{result:?}"
        );
    }
}
