use crate::collecting::domain::DepotView;
use crate::collecting::infrastructure::repositories::CollectingUowExt;
use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;

/// Query to retrieve depot view (list of owned rolling stocks for UI depot).
#[derive(Debug)]
pub struct GetDepotQuery;

impl GetDepotQuery {
    pub async fn execute(
        unit_of_work: &mut SqliteUnitOfWork<'_>,
    ) -> Result<DepotView, DomainError> {
        let mut repo = unit_of_work.collection_repository();

        let view = repo.find_depot_view().await?;

        Ok(view)
    }
}
