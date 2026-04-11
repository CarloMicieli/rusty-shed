//! Query to fetch all track inventories with summary information.

use crate::core::domain::domain_error::DomainError;
use crate::tracks_inventory::domain::TracksInventoryUowExt;
use crate::tracks_inventory::domain::views::TrackInventoryListItem;

/// Query to fetch all track inventories.
pub struct GetTrackInventoriesQuery;

impl GetTrackInventoriesQuery {
    /// Execute the query to get all track inventories with summary data.
    ///
    /// # Arguments
    /// - `unit_of_work`: Transactional unit providing repository access.
    ///
    /// # Returns
    /// * `Vec<TrackInventoryListItem>` - List of inventory summaries.
    /// * `DomainError` - On database or domain error.
    ///
    /// # Type Parameters
    /// - `U`: Unit of work type implementing `TracksInventoryUowExt` and `Send`.
    pub async fn execute<U>(
        unit_of_work: &mut U,
    ) -> Result<Vec<TrackInventoryListItem>, DomainError>
    where
        U: TracksInventoryUowExt,
    {
        unit_of_work
            .track_inventories_repo()
            .find_all_summaries()
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_uow::testing::{MockAppUow, OneShotFactory};
    use crate::app_uow::{AppUnitOfWork, AppUowFactory};
    use crate::tracks_inventory::domain::views::TrackInventoryListItem;
    use crate::tracks_inventory::domain::{MockTrackInventoryRepository, TrackInventoryId};

    #[tokio::test]
    async fn it_returns_all_inventory_summaries() {
        let summary = TrackInventoryListItem {
            id: TrackInventoryId::default(),
            name: "My Tracks".to_string(),
            description: None,
            total_items: 3,
            total_quantity: 10,
        };
        let expected = [summary.clone()];

        let mut repo = MockTrackInventoryRepository::new();
        repo.expect_find_all_summaries()
            .times(1)
            .returning(move || Ok(vec![summary.clone()]));

        let uow = MockAppUow::new().with_track_inventory(repo);
        let factory = OneShotFactory::new(uow);
        let mut uow_box: Box<dyn AppUnitOfWork> = factory.create_uow().await.unwrap();

        let result = GetTrackInventoriesQuery::execute(&mut uow_box).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), expected.len());
    }

    #[tokio::test]
    async fn it_returns_empty_list_when_no_inventories() {
        let mut repo = MockTrackInventoryRepository::new();
        repo.expect_find_all_summaries()
            .times(1)
            .returning(|| Ok(vec![]));

        let uow = MockAppUow::new().with_track_inventory(repo);
        let factory = OneShotFactory::new(uow);
        let mut uow_box: Box<dyn AppUnitOfWork> = factory.create_uow().await.unwrap();

        let result = GetTrackInventoriesQuery::execute(&mut uow_box).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }
}
