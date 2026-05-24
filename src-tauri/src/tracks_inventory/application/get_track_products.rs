use crate::core::domain::Language;
use crate::core::domain::domain_error::DomainError;
use crate::tracks_inventory::domain::TrackProductUowExt;
use crate::tracks_inventory::domain::views::TrackProductView;

/// Query to fetch all track products.
pub struct GetTrackProductsQuery;

impl GetTrackProductsQuery {
    /// Execute the query to get all track products.
    ///
    /// # Arguments
    /// - `unit_of_work`: Transactional unit providing repository access.
    ///
    /// # Returns
    /// * `Vec<TrackProductView>` - List of all track products.
    /// * `DomainError` - On database error.
    ///
    /// # Type Parameters
    /// - `U`: Unit of work type implementing `TrackProductUowExt`.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        lang: Language,
    ) -> Result<Vec<TrackProductView>, DomainError>
    where
        U: TrackProductUowExt,
    {
        unit_of_work.track_products_repo().find_all_views(lang).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_uow::testing::{MockAppUow, OneShotFactory};
    use crate::app_uow::{AppUnitOfWork, AppUowFactory};
    use crate::core::domain::Language;
    use crate::tracks_inventory::domain::views::TrackProductView;
    use crate::tracks_inventory::domain::{
        MockTrackProductRepository, TrackCode, TrackId, TrackType,
    };

    fn make_product_view(id: TrackId) -> TrackProductView {
        TrackProductView {
            track_id: id,
            manufacturer_name: "ACME".to_string(),
            product_code: "R1".to_string(),
            description: "Straight track".to_string(),
            track_type: TrackType::Straight,
            track_code: TrackCode::Code100,
            with_roadbed: false,
            length: None,
            radius: None,
        }
    }

    #[tokio::test]
    async fn it_returns_all_track_products() {
        let track_id = TrackId::try_from("trn:track:acme:r1").unwrap();
        let view = make_product_view(track_id);
        let view_clone = view.clone();

        let mut repo = MockTrackProductRepository::new();
        repo.expect_find_all_views()
            .times(1)
            .returning(move |_| Ok(vec![view_clone.clone()]));

        let uow = MockAppUow::new().with_track_product(repo);
        let factory = OneShotFactory::new(uow);
        let mut uow_box: Box<dyn AppUnitOfWork> = factory.create_uow().await.unwrap();

        let result = GetTrackProductsQuery::execute(&mut uow_box, Language::English).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn it_returns_empty_list_when_no_products() {
        let mut repo = MockTrackProductRepository::new();
        repo.expect_find_all_views()
            .times(1)
            .returning(|_| Ok(vec![]));

        let uow = MockAppUow::new().with_track_product(repo);
        let factory = OneShotFactory::new(uow);
        let mut uow_box: Box<dyn AppUnitOfWork> = factory.create_uow().await.unwrap();

        let result = GetTrackProductsQuery::execute(&mut uow_box, Language::English).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }
}
