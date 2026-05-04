use crate::collecting::domain::{CollectionItemId, CollectionUowExt};
use crate::core::domain::{MonetaryAmount, domain_error::DomainError};
use chrono::NaiveDate;

/// Use case to mark a collection item as sold.
pub struct SellCollectionItem;

impl SellCollectionItem {
    /// Execute the sell collection item use case.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: SellCollectionItemInput,
    ) -> Result<(), DomainError>
    where
        U: CollectionUowExt + Send,
    {
        let mut repo = unit_of_work.collections_repository();
        repo.sell_item(
            &input.collection_item_id,
            input.sale_date,
            input.sale_price,
            input.buyer_id,
        )
        .await
    }
}

/// Input payload for selling a collection item.
#[derive(Debug, Clone)]
pub struct SellCollectionItemInput {
    /// Collection item identifier.
    pub collection_item_id: CollectionItemId,
    /// Sale date.
    pub sale_date: NaiveDate,
    /// Sale price in minor units with currency.
    pub sale_price: MonetaryAmount,
    /// Optional buyer identifier.
    pub buyer_id: Option<String>,
}
