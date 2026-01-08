use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::sellers::domain::seller_id::SellerId;
use crate::sellers::infrastructure::repository::SellersUowExt;

pub struct DeleteSellerUseCase;

impl DeleteSellerUseCase {
    pub async fn execute(
        unit_of_work: &mut SqliteUnitOfWork<'_>,
        id: &SellerId,
    ) -> Result<u64, DomainError> {
        let mut repo = unit_of_work.sellers_repository();
        repo.delete(id).await
    }
}
