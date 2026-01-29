use crate::catalog::domain::railway_model::RailwayModelUowExt;
use crate::collecting::application::AddCollectionItem;
use crate::collecting::application::AddCollectionItemInput;
use crate::collecting::domain::{CollectionId, CollectionUowExt};
use crate::collecting::domain::{CollectionItemId, PurchaseInfoId};
use crate::core::domain::IdProvider;
use crate::core::domain::MonetaryAmount;
use crate::core::domain::domain_error::DomainError;
use crate::sellers::domain::seller_id::SellerId;
use crate::wishlist::domain::repository::WishlistUowExt;
use crate::wishlist::domain::wishlist_id::WishlistId;
use crate::wishlist::domain::wishlist_item_id::WishlistItemId;
use crate::wishlist::domain::wishlist_status::WishlistStatus;
use chrono::NaiveDate;

/// Command object carrying the information required to move (purchase)
/// a wishlist item into the collection.
#[derive(Debug, Clone)]
pub struct MoveWishlistItemId {
    /// The target collection ID where the item will be added.
    pub collection_id: CollectionId,
    /// The wishlist ID from which the item is being moved.
    pub wishlist_id: WishlistId,
    /// The specific wishlist item ID to be moved.
    pub wishlist_item_id: WishlistItemId,
    /// The purchase price paid for the item.
    pub purchase_price: MonetaryAmount,
    /// The date the item was purchased.
    pub purchase_date: NaiveDate,
    /// Optional seller ID from whom the item was purchased.
    pub seller_id: Option<SellerId>,
}

/// Service that orchestrates moving a wishlist item into the collection
/// (i.e. recording the purchase and creating the corresponding collection item).
pub struct PurchaseWishlistItemService;

impl PurchaseWishlistItemService {
    /// Move a wishlist item into the collection inside the provided unit of work.
    ///
    /// This function performs the following steps atomically within the UoW:
    /// 1. Load the wishlist aggregate and locate the wishlist item.
    /// 2. Create an `AddCollectionItemInput` and call `AddCollectionItem::execute`.
    /// 3. Update the wishlist item to `Purchased` and store the `purchased_price`.
    /// 4. Persist the modified wishlist via the wishlist repository.
    pub async fn move_wishlist_item<U, P, Q>(
        unit_of_work: &mut U,
        collection_item_id_provider: P,
        purchase_info_id_provider: Q,
        cmd: MoveWishlistItemId,
    ) -> Result<(), DomainError>
    where
        U: WishlistUowExt + CollectionUowExt + RailwayModelUowExt + Send,
        P: IdProvider<CollectionItemId>,
        Q: IdProvider<PurchaseInfoId>,
    {
        // 1. Load wishlist aggregate (borrow repo only briefly)
        let mut wishlist = {
            let mut wishlist_repo = unit_of_work.wishlist_repository();
            wishlist_repo
                .find_by_id(&cmd.wishlist_id)
                .await?
                .ok_or(DomainError::NotFound {
                    resource: "Wishlist".to_string(),
                    identifier: cmd.wishlist_id.to_string(),
                })?
        };

        // Find the mutable item inside the wishlist
        let maybe_item = wishlist
            .items
            .iter_mut()
            .find(|i| i.id == cmd.wishlist_item_id);

        let item = match maybe_item {
            Some(i) => i.clone(),
            None => {
                return Err(DomainError::NotFound {
                    resource: "WishlistItem".to_string(),
                    identifier: cmd.wishlist_item_id.to_string(),
                });
            }
        };

        // 2. Build AddCollectionItemInput from wishlist item data and provided purchase info
        let add_input = AddCollectionItemInput {
            railway_model_id: item.railway_model_id.clone(),
            price: cmd.purchase_price.clone(),
            seller_id: cmd.seller_id.clone(),
            added_date: cmd.purchase_date,
            purchase_date: cmd.purchase_date,
            purchase_condition: None,
            model_condition: None,
            box_condition: None,
            notes: item.notes.clone(),
        };

        // 3. Execute the collecting use-case to add the item to the collection
        let _collection_item_id = AddCollectionItem::execute(
            unit_of_work,
            collection_item_id_provider,
            purchase_info_id_provider,
            add_input,
        )
        .await?;

        // 4. Update wishlist item to mark purchased and set purchased_price
        if let Some(it) = wishlist
            .items
            .iter_mut()
            .find(|i| i.id == cmd.wishlist_item_id)
        {
            it.purchased_price = Some(cmd.purchase_price.clone());
            it.status = WishlistStatus::Purchased;
        }

        // Persist the wishlist changes (reacquire repo after collecting use-case)
        let mut wishlist_repo = unit_of_work.wishlist_repository();
        wishlist_repo.save_wishlist(&wishlist).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::domain::manufacturer::ManufacturerId;
    use crate::catalog::domain::railway_model::ProductCode;
    use crate::catalog::domain::railway_model::RailwayModel;
    use crate::catalog::domain::railway_model::RailwayModelId;
    use crate::catalog::domain::railway_model::{
        Category, MockRailwayModelRepository, PowerMethod,
    };
    use crate::catalog::domain::scale::Scale;
    use crate::collecting::domain::CollectionId;
    use crate::collecting::domain::{Collection, CollectionSummary, MockCollectionRepository};
    use crate::core::domain::Currency;
    use crate::core::domain::test_utils::DefaultMockIdProvider;
    use crate::wishlist::application::testing::FakeCombinedUow;
    use crate::wishlist::domain::MockWishlistRepository;
    use crate::wishlist::domain::wishlist::Wishlist;
    use crate::wishlist::domain::wishlist_id::WishlistId;
    use crate::wishlist::domain::wishlist_item::WishlistItem;
    use crate::wishlist::domain::wishlist_item_id::WishlistItemId;
    use crate::wishlist::domain::wishlist_priority::WishlistPriority;

    #[tokio::test]
    async fn it_should_move_wishlist_item_to_collection() {
        // Prepare mocks
        let mut wishlist_mock = MockWishlistRepository::new();
        let wid = WishlistId::default();
        let wid_clone = wid.clone();
        let item_id = WishlistItemId::try_from("trn:wishlist-item:it1").unwrap();

        let wishlist_item = WishlistItem {
            id: item_id.clone(),
            railway_model_id: RailwayModelId::try_from("trn:railway-model:rm:test").unwrap(),
            priority: WishlistPriority::default(),
            status: WishlistStatus::Wanted,
            added_date: NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
            removed_date: None,
            notes: Some("note".to_string()),
            desired_price: None,
            purchased_price: None,
        };

        let wishlist = Wishlist {
            id: wid.clone(),
            name: "My wishlist".to_string(),
            notes: None,
            is_default: false,
            items: vec![wishlist_item.clone()],
            pending_events: vec![],
            metadata: Default::default(),
        };

        wishlist_mock
            .expect_find_by_id()
            .withf(move |id| *id == wid_clone)
            .times(1)
            .returning(move |_| Ok(Some(wishlist.clone())));

        wishlist_mock
            .expect_save_wishlist()
            .times(1)
            .returning(move |_w| Ok(()));

        // Collection mock
        let mut collection_mock = MockCollectionRepository::new();
        collection_mock
            .expect_find_by_id()
            .times(1)
            .returning(move |_id| {
                let coll = Collection {
                    id: CollectionId::default(),
                    name: "My Collection".to_string(),
                    summary: CollectionSummary::default(),
                    total_value: None,
                    items: vec![],
                    pending_events: vec![],
                    metadata: Default::default(),
                };
                Ok(Some(coll))
            });

        collection_mock
            .expect_save()
            .times(1)
            .returning(move |_c| Ok(()));

        // Railway model mock
        let mut railway_mock = MockRailwayModelRepository::new();
        let rm_id = RailwayModelId::try_from("trn:railway-model:rm:test").unwrap();
        let railway_model = RailwayModel {
            id: rm_id.clone(),
            manufacturer_id: ManufacturerId::try_from("trn:manufacturer:not-a-trn").unwrap(),
            product_code: ProductCode::try_from("P100").unwrap(),
            description: "Test model".to_string(),
            details: None,
            power_method: PowerMethod::DC,
            scale: Scale::H0,
            epoch: "IV".into(),
            category: Category::Locomotives,
            delivery_date: None,
            availability_status: None,
            rolling_stocks: vec![],
            pending_events: vec![],
        };

        railway_mock
            .expect_find_by_id()
            .withf(move |id| *id == rm_id)
            .times(1)
            .returning(move |_| Ok(Some(railway_model.clone())));

        // Build combined fake uow
        let mut uow = FakeCombinedUow::new(wishlist_mock, collection_mock, railway_mock);

        let cid_provider = DefaultMockIdProvider::<CollectionItemId>::new();
        let purchase_info_provider = DefaultMockIdProvider::<PurchaseInfoId>::new();

        let cmd = MoveWishlistItemId {
            collection_id: CollectionId::default(),
            wishlist_id: wid.clone(),
            wishlist_item_id: item_id.clone(),
            purchase_price: MonetaryAmount::new(100, Currency::USD),
            purchase_date: NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
            seller_id: None,
        };

        let res = PurchaseWishlistItemService::move_wishlist_item(
            &mut uow,
            cid_provider,
            purchase_info_provider,
            cmd,
        )
        .await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn it_should_fail_when_railway_model_missing() {
        let mut wishlist_mock = MockWishlistRepository::new();
        let wid = WishlistId::default();
        let wid_clone = wid.clone();
        let item_id = WishlistItemId::try_from("trn:wishlist-item:it2").unwrap();

        let wishlist_item = WishlistItem {
            id: item_id.clone(),
            railway_model_id: RailwayModelId::try_from("trn:railway-model:rm:notfound").unwrap(),
            priority: WishlistPriority::default(),
            status: WishlistStatus::Wanted,
            added_date: NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
            removed_date: None,
            notes: Some("note".to_string()),
            desired_price: None,
            purchased_price: None,
        };

        let wishlist = Wishlist {
            id: wid.clone(),
            name: "WL".to_string(),
            notes: None,
            is_default: false,
            items: vec![wishlist_item.clone()],
            pending_events: vec![],
            metadata: Default::default(),
        };

        wishlist_mock
            .expect_find_by_id()
            .withf(move |id| *id == wid_clone)
            .times(1)
            .returning(move |_| Ok(Some(wishlist.clone())));

        // railway returns not found
        let mut railway_mock = MockRailwayModelRepository::new();
        let rm_id = RailwayModelId::try_from("trn:railway-model:rm:notfound").unwrap();
        railway_mock
            .expect_find_by_id()
            .withf(move |id| *id == rm_id)
            .times(1)
            .returning(move |_| Ok(None));

        // collection not involved for this failure, but provide a default
        let mut collection_mock = MockCollectionRepository::new();
        collection_mock.expect_find_by_id().times(0);

        let mut uow = FakeCombinedUow::new(wishlist_mock, collection_mock, railway_mock);

        let cid_provider = DefaultMockIdProvider::<CollectionItemId>::new();
        let purchase_info_provider = DefaultMockIdProvider::<PurchaseInfoId>::new();

        let cmd = MoveWishlistItemId {
            collection_id: CollectionId::default(),
            wishlist_id: wid.clone(),
            wishlist_item_id: item_id.clone(),
            purchase_price: MonetaryAmount::new(100, Currency::USD),
            purchase_date: NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
            seller_id: None,
        };

        let res = PurchaseWishlistItemService::move_wishlist_item(
            &mut uow,
            cid_provider,
            purchase_info_provider,
            cmd,
        )
        .await;

        assert!(res.is_err());
        match res.err().unwrap() {
            DomainError::NotFound { resource, .. } => assert_eq!(resource, "RailwayModel"),
            other => panic!("unexpected error: {:?}", other),
        }
    }

    #[tokio::test]
    async fn it_should_fail_when_collection_save_fails() {
        let mut wishlist_mock = MockWishlistRepository::new();
        let wid = WishlistId::default();
        let wid_clone = wid.clone();
        let item_id = WishlistItemId::try_from("trn:wishlist-item:it3").unwrap();

        let wishlist_item = WishlistItem {
            id: item_id.clone(),
            railway_model_id: RailwayModelId::try_from("trn:railway-model:rm:test").unwrap(),
            priority: WishlistPriority::default(),
            status: WishlistStatus::Wanted,
            added_date: NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
            removed_date: None,
            notes: Some("note".to_string()),
            desired_price: None,
            purchased_price: None,
        };

        let wishlist = Wishlist {
            id: wid.clone(),
            name: "WL2".to_string(),
            notes: None,
            is_default: false,
            items: vec![wishlist_item.clone()],
            pending_events: vec![],
            metadata: Default::default(),
        };

        wishlist_mock
            .expect_find_by_id()
            .withf(move |id| *id == wid_clone)
            .times(1)
            .returning(move |_| Ok(Some(wishlist.clone())));

        // Collection save will fail
        let mut collection_mock = MockCollectionRepository::new();
        collection_mock
            .expect_find_by_id()
            .times(1)
            .returning(move |_id| {
                let coll = Collection {
                    id: CollectionId::default(),
                    name: "My Collection".to_string(),
                    summary: CollectionSummary::default(),
                    total_value: None,
                    items: vec![],
                    pending_events: vec![],
                    metadata: Default::default(),
                };
                Ok(Some(coll))
            });

        collection_mock
            .expect_save()
            .times(1)
            .returning(move |_c| Err(DomainError::Infrastructure(sqlx::Error::RowNotFound)));

        // Railway model present
        let mut railway_mock = MockRailwayModelRepository::new();
        let rm_id = RailwayModelId::try_from("trn:railway-model:rm:test").unwrap();
        let railway_model = RailwayModel {
            id: rm_id.clone(),
            manufacturer_id: ManufacturerId::try_from("trn:manufacturer:not-a-trn").unwrap(),
            product_code: ProductCode::try_from("P100").unwrap(),
            description: "Test model".to_string(),
            details: None,
            power_method: PowerMethod::DC,
            scale: Scale::H0,
            epoch: "IV".into(),
            category: Category::Locomotives,
            delivery_date: None,
            availability_status: None,
            rolling_stocks: vec![],
            pending_events: vec![],
        };

        railway_mock
            .expect_find_by_id()
            .withf(move |id| *id == rm_id)
            .times(1)
            .returning(move |_| Ok(Some(railway_model.clone())));

        let mut uow = FakeCombinedUow::new(wishlist_mock, collection_mock, railway_mock);

        let cid_provider = DefaultMockIdProvider::<CollectionItemId>::new();
        let purchase_info_provider = DefaultMockIdProvider::<PurchaseInfoId>::new();

        let cmd = MoveWishlistItemId {
            collection_id: CollectionId::default(),
            wishlist_id: wid.clone(),
            wishlist_item_id: item_id.clone(),
            purchase_price: MonetaryAmount::new(100, Currency::USD),
            purchase_date: NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
            seller_id: None,
        };

        let res = PurchaseWishlistItemService::move_wishlist_item(
            &mut uow,
            cid_provider,
            purchase_info_provider,
            cmd,
        )
        .await;

        assert!(res.is_err());
        match res.err().unwrap() {
            DomainError::Infrastructure(_) => {}
            other => panic!("unexpected error: {:?}", other),
        }
    }
}
