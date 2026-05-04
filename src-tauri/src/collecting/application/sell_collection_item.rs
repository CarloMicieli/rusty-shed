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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::domain::railway_model::MockRailwayModelRepository;
    use crate::collecting::application::testing::FakeUow;
    use crate::collecting::domain::{CollectionItemId, MockCollectionRepository};
    use crate::core::domain::identifiers::Identifier;
    use crate::core::domain::{Currency, MonetaryAmount, domain_error::DomainError};

    fn make_input() -> SellCollectionItemInput {
        SellCollectionItemInput {
            collection_item_id: CollectionItemId::new_from_parts(&["item-1"]),
            sale_date: chrono::NaiveDate::from_ymd_opt(2025, 6, 1).unwrap(),
            sale_price: MonetaryAmount::new(5000, Currency::EUR),
            buyer_id: None,
        }
    }

    #[tokio::test]
    async fn happy_path_sells_item() {
        let mut collection_repo = MockCollectionRepository::new();
        collection_repo
            .expect_sell_item()
            .times(1)
            .returning(|_, _, _, _| Ok(()));

        let railway_repo = MockRailwayModelRepository::new();
        let mut uow = FakeUow::new(collection_repo, railway_repo);
        let result = SellCollectionItem::execute(&mut uow, make_input()).await;

        assert!(result.is_ok(), "{result:?}");
    }

    #[tokio::test]
    async fn repo_error_propagates() {
        let mut collection_repo = MockCollectionRepository::new();
        collection_repo
            .expect_sell_item()
            .times(1)
            .returning(|_, _, _, _| Err(DomainError::Infrastructure("write failed".into())));

        let railway_repo = MockRailwayModelRepository::new();
        let mut uow = FakeUow::new(collection_repo, railway_repo);
        let result = SellCollectionItem::execute(&mut uow, make_input()).await;

        assert!(matches!(result, Err(DomainError::Infrastructure(_))));
    }
}
