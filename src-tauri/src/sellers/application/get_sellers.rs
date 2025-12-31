use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::sellers::domain::seller::Seller;
use crate::sellers::infrastructure::repository::SellersUowExt;

pub struct GetSellersUseCase;

impl GetSellersUseCase {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self
    }

    pub async fn execute(&self, uow: &mut SqliteUnitOfWork<'_>) -> anyhow::Result<Vec<Seller>> {
        let mut repo = uow.sellers_repo();
        repo.list().await
    }
}
