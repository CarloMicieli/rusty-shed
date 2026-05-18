use crate::core::domain::domain_error::DomainError;
use crate::sellers::application::merge_seller::MergeSeller;
use crate::sellers::domain::seller_id::SellerId;

pub struct MergeBuyer;

impl MergeBuyer {
    pub async fn execute(
        tx: &mut sqlx::SqliteConnection,
        source_id: &SellerId,
        target_id: &SellerId,
    ) -> Result<i64, DomainError> {
        MergeSeller::execute(tx, source_id, target_id).await
    }
}
