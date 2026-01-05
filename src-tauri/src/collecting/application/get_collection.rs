use crate::collecting::domain::Collection;
use crate::collecting::infrastructure::repositories::CollectingUowExt;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;

#[derive(Debug, Default)]
pub struct GetCollectionUseCase;

impl GetCollectionUseCase {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self
    }

    /// Executes the business logic within the context of a Unit of Work.
    ///
    /// By taking `SqliteUnitOfWork` as an argument, the Use Case can ensure
    /// that all operations are part of the same transaction.
    pub async fn execute(&self, uow: &mut SqliteUnitOfWork<'_>) -> anyhow::Result<Collection> {
        // 1. Access the repository through the Extension Trait
        // This re-borrows the transaction inside the UoW
        let mut repo = uow.collection_repo();

        // 2. Perform the domain logic
        let collection = repo.get_collection().await?;

        // 3. Return the result
        // Note: The caller (usually a Controller or Service) typically
        // decides when to call uow.commit()
        Ok(collection)
    }
}
