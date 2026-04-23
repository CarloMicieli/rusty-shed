use crate::core::domain::domain_error::DomainError;
use crate::tracks_inventory::domain::views::TrackInventoryView;
use crate::tracks_inventory::domain::{TrackInventoryId, TracksInventoryUowExt};

/// Query to fetch a single track inventory with items and purchases.
pub struct GetTrackInventoryQuery;

impl GetTrackInventoryQuery {
    /// Execute the query to get a specific track inventory.
    ///
    /// # Arguments
    /// - `unit_of_work`: Transactional unit providing repository access.
    /// - `id`: The inventory ID to fetch.
    ///
    /// # Returns
    /// * `TrackInventoryView` - Complete inventory view with items and purchases.
    /// * `DomainError::NotFound` - If inventory doesn't exist.
    /// * `DomainError` - On other errors.
    ///
    /// # Type Parameters
    /// - `U`: Unit of work type implementing `TracksInventoryUowExt`.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        id: &TrackInventoryId,
    ) -> Result<TrackInventoryView, DomainError>
    where
        U: TracksInventoryUowExt,
    {
        unit_of_work
            .track_inventories_repo()
            .find_view_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound {
                resource: "TrackInventory".to_string(),
                identifier: id.to_string(),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_uow::testing::{MockAppUow, OneShotFactory};
    use crate::app_uow::{AppUnitOfWork, AppUowFactory};
    use crate::core::domain::domain_error::DomainError;
    use crate::tracks_inventory::domain::views::TrackInventoryView;
    use crate::tracks_inventory::domain::{MockTrackInventoryRepository, TrackInventoryId};

    fn make_view(id: TrackInventoryId) -> TrackInventoryView {
        TrackInventoryView {
            id,
            name: "My Tracks".to_string(),
            description: None,
            items: vec![],
            purchases: vec![],
        }
    }

    #[tokio::test]
    async fn it_returns_inventory_view_when_found() {
        let id = TrackInventoryId::default();
        let id_clone = id.clone();
        let view = make_view(id.clone());

        let mut repo = MockTrackInventoryRepository::new();
        repo.expect_find_view_by_id()
            .times(1)
            .returning(move |_| Ok(Some(view.clone())));

        let uow = MockAppUow::new().with_track_inventory(repo);
        let factory = OneShotFactory::new(uow);
        let mut uow_box: Box<dyn AppUnitOfWork> = factory.create_uow().await.unwrap();

        let result = GetTrackInventoryQuery::execute(&mut uow_box, &id_clone).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, id_clone);
    }

    #[tokio::test]
    async fn it_returns_not_found_when_inventory_missing() {
        let id = TrackInventoryId::default();
        let id_clone = id.clone();

        let mut repo = MockTrackInventoryRepository::new();
        repo.expect_find_view_by_id()
            .times(1)
            .returning(|_| Ok(None));

        let uow = MockAppUow::new().with_track_inventory(repo);
        let factory = OneShotFactory::new(uow);
        let mut uow_box: Box<dyn AppUnitOfWork> = factory.create_uow().await.unwrap();

        let result = GetTrackInventoryQuery::execute(&mut uow_box, &id_clone).await;

        assert!(matches!(result.unwrap_err(), DomainError::NotFound { .. }));
    }
}
