use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::sellers::domain::seller::Seller;
use crate::sellers::domain::seller_id::SellerId;
use crate::sellers::infrastructure::repository::SellersUowExt;

pub struct GetSellerByIdUseCase;

impl GetSellerByIdUseCase {
    pub async fn execute(
        unit_of_work: &mut SqliteUnitOfWork<'_>,
        id: &SellerId,
    ) -> Result<Option<Seller>, DomainError> {
        let mut repo = unit_of_work.sellers_repository();
        repo.get(id).await
    }
}
