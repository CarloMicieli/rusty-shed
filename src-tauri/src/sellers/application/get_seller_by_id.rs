use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::sellers::domain::seller::Seller;
use crate::sellers::domain::seller_id::SellerId;
use crate::sellers::infrastructure::repository::SellersUowExt;

pub struct GetSellerByIdUseCase;

impl GetSellerByIdUseCase {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self
    }

    pub async fn execute(
        &self,
        uow: &mut SqliteUnitOfWork<'_>,
        id: &SellerId,
    ) -> anyhow::Result<Option<Seller>> {
        let mut repo = uow.sellers_repo();
        repo.get(id).await
    }
}
