use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::sellers::domain::seller::Seller;
use crate::sellers::infrastructure::repository::SellersUowExt;

pub struct GetSellersUseCase;

impl GetSellersUseCase {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self
    }

    pub async fn execute(
        &self,
        uow: &mut SqliteUnitOfWork<'_>,
    ) -> Result<Vec<Seller>, DomainError> {
        let mut repo = uow.sellers_repository();
        repo.list().await
    }
}
