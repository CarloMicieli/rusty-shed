use crate::collecting::domain::{CollectionUowExt, DepotView};
use crate::core::domain::domain_error::DomainError;

/// Query to retrieve depot view (list of owned rolling stocks for UI depot).
pub struct GetDepot;

impl GetDepot {
    /// Execute the query to retrieve the depot view.
    ///
    /// # Arguments
    /// * `unit_of_work` - The unit of work managing the database transaction.
    ///
    /// # Returns
    /// - `Ok(DepotView)` containing the depot view on success.
    /// - `Err(DomainError)` with an error message on failure.
    ///
    /// # Type Parameters
    /// - `U`: Unit of work type implementing `CollectionUowExt` and `Send`.
    pub async fn execute<U>(unit_of_work: &mut U) -> Result<DepotView, DomainError>
    where
        U: CollectionUowExt + Send,
    {
        let mut repo = unit_of_work.collections_repository();

        let view = repo.find_depot_view().await?;

        Ok(view)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::domain::railway_model::MockRailwayModelRepository;
    use crate::collecting::application::testing::FakeUow;
    use crate::collecting::domain::MockCollectionRepository;

    #[tokio::test]
    async fn it_should_return_depot_view() {
        let mut mock = MockCollectionRepository::new();
        mock.expect_find_depot_view().times(1).returning(move || {
            let view = DepotView {
                rolling_stocks: vec![],
            };
            Ok(view.clone())
        });

        let mut unit_of_work = FakeUow::new(mock, MockRailwayModelRepository::new());

        let result = GetDepot::execute(&mut unit_of_work)
            .await
            .expect("Failed to retrieve depot view");

        assert_eq!(result.rolling_stocks.len(), 0);
    }
}
