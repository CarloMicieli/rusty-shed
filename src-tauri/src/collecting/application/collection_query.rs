use crate::collecting::domain::Collection;
use crate::collecting::infrastructure::repositories::CollectingUowExt;
use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;

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
    pub async fn execute(
        unit_of_work: &mut SqliteUnitOfWork<'_>,
    ) -> Result<Collection, DomainError> {
        let mut repo = unit_of_work.collection_repo();

        let items = repo.find_collection().await?;

        Ok(items)
    }
}
