use crate::collecting::domain::{CollectionUowExt, CollectionView};
use crate::core::domain::domain_error::DomainError;

/// Query to get the entire collection.
#[derive(Debug)]
pub struct GetCollectionQuery;

impl GetCollectionQuery {
    /// Execute the query to retrieve the entire collection.
    ///
    /// # Arguments
    /// * `unit_of_work` - The unit of work managing the database transaction.
    ///
    /// # Returns
    /// - `Ok(Collection)` containing the entire collection on success.
    /// - `Err(DomainError)` with an error message on failure.
    ///
    /// # Type Parameters
    /// - `U`: Unit of work type implementing `CollectionUowExt` and `Send`.
    pub async fn execute<U>(unit_of_work: &mut U) -> Result<CollectionView, DomainError>
    where
        U: CollectionUowExt + Send,
    {
        let mut repo = unit_of_work.collections_repository();

        let items = repo.find_view().await?;

        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::domain::railway_model::MockRailwayModelRepository;
    use crate::collecting::application::testing::FakeUow;
    use crate::collecting::domain::{CollectionId, CollectionSummary, MockCollectionRepository};

    #[tokio::test]
    async fn it_should_return_collection_view() {
        let mut mock = MockCollectionRepository::new();
        mock.expect_find_view().times(1).returning(move || {
            let view = CollectionView {
                id: CollectionId::default(),
                name: "My Collection".to_string(),
                summary: CollectionSummary::default(),
                total_value: None,
                items: vec![],
            };
            Ok(view.clone())
        });

        let mut unit_of_work = FakeUow::new(mock, MockRailwayModelRepository::new());

        let result = GetCollectionQuery::execute(&mut unit_of_work)
            .await
            .expect("Failed to retrieve collection item");

        assert_eq!(result.items.len(), 0);
    }
}
