use crate::collecting::domain::CollectionItemId;
use crate::collecting::domain::CollectionUowExt;
use crate::core::domain::domain_error::DomainError;
use chrono::NaiveDate;

/// Application use case: mark a pre-ordered item as received.
///
/// Converts the `purchase_type` from `PREORDER` to `PURCHASED` for the
/// given collection item, using `received_date` as the new `purchase_date`.
/// After the update the collection's summary and total value are recalculated
/// so the item is now counted in the active inventory.
pub struct ReceivePreorder;

impl ReceivePreorder {
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: ReceivePreorderInput,
    ) -> Result<(), DomainError>
    where
        U: CollectionUowExt + Send,
    {
        let mut repo = unit_of_work.collections_repository();
        repo.receive_preorder(&input.collection_item_id, input.received_date)
            .await
    }
}

/// Input for the [`ReceivePreorder`] use case.
#[derive(Debug, Clone)]
pub struct ReceivePreorderInput {
    /// The item that has been physically received.
    pub collection_item_id: CollectionItemId,
    /// The date the item arrived (used as the new purchase_date).
    pub received_date: NaiveDate,
}
