use crate::collecting::domain::Collection;
use crate::collecting::infrastructure::repositories::CollectingUowExt;
use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;

#[derive(Debug)]
pub struct GetCollectionQuery;

impl GetCollectionQuery {
    pub async fn execute(
        unit_of_work: &mut SqliteUnitOfWork<'_>,
    ) -> Result<Collection, DomainError> {
        let mut repo = unit_of_work.collection_repo();

        let items = repo.get_collection().await?;

        Ok(items)
    }
}
