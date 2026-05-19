use crate::core::domain::domain_error::DomainError;
use crate::sellers::application::merge_seller::MergeSeller;
use crate::sellers::domain::seller_id::SellerId;

/// Use case that merges one buyer entity into another.
pub struct MergeBuyer;

impl MergeBuyer {
    /// Merges `source_id` into `target_id` and returns the number of relinked references.
    ///
    /// This delegates to the shared seller-merge flow because buyers are represented by the
    /// same canonical party aggregate.
    pub async fn execute(
        tx: &mut sqlx::SqliteConnection,
        source_id: &SellerId,
        target_id: &SellerId,
    ) -> Result<i64, DomainError> {
        MergeSeller::execute(tx, source_id, target_id).await
    }
}
