use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::sellers::domain::seller_id::SellerId;
use crate::sellers::infrastructure::repository::SellersUowExt;

pub struct DeleteSellerUseCase;

impl DeleteSellerUseCase {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self
    }

    pub async fn execute(
        &self,
        uow: &mut SqliteUnitOfWork<'_>,
        id: &SellerId,
    ) -> anyhow::Result<u64> {
        let mut repo = uow.sellers_repo();
        repo.delete(id).await
    }
}
