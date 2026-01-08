use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::sellers::domain::seller::Seller;
use crate::sellers::infrastructure::repository::SellersUowExt;

pub struct GetSellersUseCase;

impl GetSellersUseCase {
    pub async fn execute(
        unit_of_work: &mut SqliteUnitOfWork<'_>,
    ) -> Result<Vec<Seller>, DomainError> {
        let mut repo = unit_of_work.sellers_repository();
        repo.list().await
    }
}
