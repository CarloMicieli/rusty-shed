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

        repo.upsert_translation(
            &input.track_id,
            input.lang,
            input.description,
            input.details,
        )
        .await?;

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
    async fn execute_forwards_translation_payload_to_repository() {
        let track_id = TrackId::try_from("trn:track:acme:60100").expect("valid track id");
        let expected_track_id = track_id.clone();

        let mut repo = MockTrackProductRepository::new();
        repo.expect_upsert_translation()
            .once()
            .withf(move |id, lang, description, details| {
                id == &expected_track_id
                    && *lang == Language::English
                    && description.as_deref() == Some("Mainline turnout")
                    && details.as_deref() == Some("Large radius")
            })
            .returning(|_, _, _, _| Ok(()));

        let mut uow = FakeTrackProductUow::new(repo);
        let input = UpsertTrackProductTranslationInput {
            track_id,
            lang: Language::English,
            description: Some("Mainline turnout".to_string()),
            details: Some("Large radius".to_string()),
        };

        let result = UpsertTrackProductTranslationUseCase::execute(&mut uow, input).await;

        assert!(result.is_ok(), "{result:?}");
    }

    #[tokio::test]
    async fn execute_maps_repository_upsert_error_to_command_error() {
        let track_id = TrackId::try_from("trn:track:acme:60100").expect("valid track id");

        let mut repo = MockTrackProductRepository::new();
        repo.expect_upsert_translation()
            .once()
            .returning(|_, _, _, _| Err(DomainError::Infrastructure("db upsert failed".to_string())));

        let mut uow = FakeTrackProductUow::new(repo);
        let input = UpsertTrackProductTranslationInput {
            track_id,
            lang: Language::Italian,
            description: Some("Scambio".to_string()),
            details: None,
        };

        let result = UpsertTrackProductTranslationUseCase::execute(&mut uow, input).await;

        assert!(
            matches!(result, Err(CommandError::DatabaseError(ref msg)) if msg.contains("db upsert failed")),
            "{result:?}"
        );
    }
}
