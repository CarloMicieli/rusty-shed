use crate::catalog::domain::railway_model::RailwayModelUowExt;
use crate::collecting::application::AddCollectionItem;
use crate::collecting::application::AddCollectionItemInput;
use crate::collecting::domain::CollectionUowExt;
use crate::collecting::domain::ModelCondition;
use crate::collecting::domain::PurchaseCondition;
use crate::collecting::domain::{CollectionItemId, PurchaseInfoId};
use crate::core::domain::IdProvider;
use crate::core::domain::MonetaryAmount;
use crate::core::domain::domain_error::DomainError;
use crate::sellers::domain::seller_id::SellerId;
use crate::wishlist::domain::repository::WishlistUowExt;
use crate::wishlist::domain::wishlist_id::WishlistId;
use crate::wishlist::domain::wishlist_item_id::WishlistItemId;
use chrono::NaiveDate;

/// Command object carrying the information required to purchase
/// a wishlist item and move it into the collection.
#[derive(Debug, Clone)]
pub struct PurchaseWishlistItemCommand {
    /// The wishlist ID from which the item is being purchased.
    pub wishlist_id: WishlistId,
    /// The specific wishlist item ID to be purchased.
    pub wishlist_item_id: WishlistItemId,
    /// The purchase price paid for the item.
    pub purchase_price: MonetaryAmount,
    /// The date the item was purchased.
    pub purchase_date: NaiveDate,
    /// Optional seller ID from whom the item was purchased.
    pub seller_id: Option<SellerId>,
    /// The purchase condition (New or Pre-Owned).
    pub purchase_condition: Option<PurchaseCondition>,
    /// The model condition grade (for pre-owned items).
    pub model_condition: Option<ModelCondition>,
}

/// Service that orchestrates purchasing a wishlist item into the collection
/// (i.e. recording the purchase and creating the corresponding collection item).
pub struct PurchaseWishlistItemService;

impl PurchaseWishlistItemService {
    /// Purchase a wishlist item and move it into the collection inside the provided unit of work.
    ///
    /// This function performs the following steps atomically within the UoW:
    /// 1. Load the wishlist aggregate and call `purchase_item()` to validate and emit the event.
    /// 2. Create an `AddCollectionItemInput` (with condition data) and call `AddCollectionItem::execute`.
    /// 3. Persist the modified wishlist via the wishlist repository.
    pub async fn execute<U, P, Q>(
        unit_of_work: &mut U,
        collection_item_id_provider: P,
        purchase_info_id_provider: Q,
        cmd: PurchaseWishlistItemCommand,
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

        // Snapshot the item's railway_model_id and notes before calling purchase_item
        let (railway_model_id, item_notes) = {
            let item = wishlist
                .items
                .iter()
                .find(|i| i.id == cmd.wishlist_item_id)
                .ok_or(DomainError::NotFound {
                    resource: "WishlistItem".to_string(),
                    identifier: cmd.wishlist_item_id.to_string(),
                })?;
            (item.railway_model_id.clone(), item.notes.clone())
        };

        // 2. Validate and transition item status via domain method
        wishlist.purchase_item(&cmd.wishlist_item_id, cmd.purchase_price.clone())?;

        // 3. Build AddCollectionItemInput from wishlist item data and provided purchase info
        let add_input = AddCollectionItemInput {
            railway_model_id,
            price: cmd.purchase_price.clone(),
            seller_id: cmd.seller_id.clone(),
            added_date: cmd.purchase_date,
            purchase_date: cmd.purchase_date,
            purchase_condition: cmd.purchase_condition,
            model_condition: cmd.model_condition,
            box_condition: None,
            notes: item_notes,
        };

        // 4. Execute the collecting use-case to add the item to the collection
        let _collection_item_id = AddCollectionItem::execute(
            unit_of_work,
            collection_item_id_provider,
            purchase_info_id_provider,
            add_input,
        )
        .await?;

        // 5. Persist the wishlist changes (reacquire repo after collecting use-case)
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
    use crate::catalog::domain::railway_model::localized_field::LocalizedField;
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
    use crate::wishlist::domain::wishlist_status::WishlistStatus;

    fn make_wishlist(wid: WishlistId, item_id: WishlistItemId, status: WishlistStatus) -> Wishlist {
        let wishlist_item = WishlistItem {
            id: item_id,
            railway_model_id: RailwayModelId::try_from("trn:railway-model:rm:test").unwrap(),
            priority: WishlistPriority::default(),
            status,
            added_date: NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
            removed_date: None,
            notes: Some("note".to_string()),
            desired_price: None,
            purchased_price: None,
        };

        Wishlist {
            id: wid,
            name: "My wishlist".to_string(),
            notes: None,
            is_default: false,
            items: vec![wishlist_item],
            pending_events: vec![],
            metadata: Default::default(),
        }
    }

    fn make_railway_model(rm_id: RailwayModelId) -> RailwayModel {
        RailwayModel {
            id: rm_id,
            manufacturer_id: ManufacturerId::try_from("trn:manufacturer:not-a-trn").unwrap(),
            product_code: ProductCode::try_from("P100").unwrap(),
            description: LocalizedField {
                lang: "en".to_string(),
                value: "Test model".to_string(),
            },
            details: None,
            power_method: PowerMethod::DC,
            scale: Scale::H0,
            epoch: "IV".into(),
            category: Category::Locomotives,
            delivery_date: None,
            availability_status: None,
            rolling_stocks: vec![],
            pending_events: vec![],
        }
    }

    #[tokio::test]
    async fn it_should_purchase_wishlist_item_into_collection() {
        let mut wishlist_mock = MockWishlistRepository::new();
        let wid = WishlistId::default();
        let wid_clone = wid.clone();
        let item_id = WishlistItemId::try_from("trn:wishlist-item:it1").unwrap();
        let item_id_clone = item_id.clone();

        let wishlist = make_wishlist(wid.clone(), item_id.clone(), WishlistStatus::Wanted);
        wishlist_mock
            .expect_find_by_id()
            .withf(move |id| *id == wid_clone)
            .times(1)
            .returning(move |_| Ok(Some(wishlist.clone())));

        wishlist_mock
            .expect_save_wishlist()
            .times(1)
            .returning(move |_w| Ok(()));

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

        let mut railway_mock = MockRailwayModelRepository::new();
        let rm_id = RailwayModelId::try_from("trn:railway-model:rm:test").unwrap();
        let rm_id_clone = rm_id.clone();
        let railway_model = make_railway_model(rm_id.clone());

        railway_mock
            .expect_find_by_id()
            .withf(move |id, _lang| *id == rm_id_clone)
            .times(1)
            .returning(move |_, _| Ok(Some(railway_model.clone())));

        let mut uow = FakeCombinedUow::new(wishlist_mock, collection_mock, railway_mock);
        let cid_provider = DefaultMockIdProvider::<CollectionItemId>::new();
        let purchase_info_provider = DefaultMockIdProvider::<PurchaseInfoId>::new();

        let cmd = PurchaseWishlistItemCommand {
            wishlist_id: wid.clone(),
            wishlist_item_id: item_id_clone,
            purchase_price: MonetaryAmount::new(100, Currency::USD),
            purchase_date: NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
            seller_id: None,
            purchase_condition: None,
            model_condition: None,
        };

        let res = PurchaseWishlistItemService::execute(
            &mut uow,
            cid_provider,
            purchase_info_provider,
            cmd,
        )
        .await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn it_should_fail_when_item_is_already_purchased() {
        let mut wishlist_mock = MockWishlistRepository::new();
        let wid = WishlistId::default();
        let wid_clone = wid.clone();
        let item_id = WishlistItemId::try_from("trn:wishlist-item:it-purchased").unwrap();
        let item_id_clone = item_id.clone();

        let wishlist = make_wishlist(wid.clone(), item_id.clone(), WishlistStatus::Purchased);
        wishlist_mock
            .expect_find_by_id()
            .withf(move |id| *id == wid_clone)
            .times(1)
            .returning(move |_| Ok(Some(wishlist.clone())));

        let collection_mock = MockCollectionRepository::new();
        let railway_mock = MockRailwayModelRepository::new();

        let mut uow = FakeCombinedUow::new(wishlist_mock, collection_mock, railway_mock);
        let cid_provider = DefaultMockIdProvider::<CollectionItemId>::new();
        let purchase_info_provider = DefaultMockIdProvider::<PurchaseInfoId>::new();

        let cmd = PurchaseWishlistItemCommand {
            wishlist_id: wid.clone(),
            wishlist_item_id: item_id_clone,
            purchase_price: MonetaryAmount::new(100, Currency::USD),
            purchase_date: NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
            seller_id: None,
            purchase_condition: None,
            model_condition: None,
        };

        let res = PurchaseWishlistItemService::execute(
            &mut uow,
            cid_provider,
            purchase_info_provider,
            cmd,
        )
        .await;

        assert!(res.is_err());
        match res.err().unwrap() {
            DomainError::BusinessRule(_) => {}
            other => panic!("expected BusinessRule, got: {:?}", other),
        }
    }

    #[tokio::test]
    async fn it_should_forward_condition_to_collection_item() {
        let mut wishlist_mock = MockWishlistRepository::new();
        let wid = WishlistId::default();
        let wid_clone = wid.clone();
        let item_id = WishlistItemId::try_from("trn:wishlist-item:it-cond").unwrap();
        let item_id_clone = item_id.clone();

        let wishlist = make_wishlist(wid.clone(), item_id.clone(), WishlistStatus::Wanted);
        wishlist_mock
            .expect_find_by_id()
            .withf(move |id| *id == wid_clone)
            .times(1)
            .returning(move |_| Ok(Some(wishlist.clone())));

        wishlist_mock
            .expect_save_wishlist()
            .times(1)
            .returning(move |_w| Ok(()));

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

        let mut railway_mock = MockRailwayModelRepository::new();
        let rm_id = RailwayModelId::try_from("trn:railway-model:rm:test").unwrap();
        let rm_id_clone = rm_id.clone();
        let railway_model = make_railway_model(rm_id.clone());

        railway_mock
            .expect_find_by_id()
            .withf(move |id, _lang| *id == rm_id_clone)
            .times(1)
            .returning(move |_, _| Ok(Some(railway_model.clone())));

        let mut uow = FakeCombinedUow::new(wishlist_mock, collection_mock, railway_mock);
        let cid_provider = DefaultMockIdProvider::<CollectionItemId>::new();
        let purchase_info_provider = DefaultMockIdProvider::<PurchaseInfoId>::new();

        let cmd = PurchaseWishlistItemCommand {
            wishlist_id: wid.clone(),
            wishlist_item_id: item_id_clone,
            purchase_price: MonetaryAmount::new(200, Currency::EUR),
            purchase_date: NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
            seller_id: None,
            purchase_condition: Some(PurchaseCondition::PreOwned),
            model_condition: Some(ModelCondition::NearMint),
        };

        let res = PurchaseWishlistItemService::execute(
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
        let item_id_clone = item_id.clone();

        let wishlist = make_wishlist(wid.clone(), item_id.clone(), WishlistStatus::Wanted);
        wishlist_mock
            .expect_find_by_id()
            .withf(move |id| *id == wid_clone)
            .times(1)
            .returning(move |_| Ok(Some(wishlist.clone())));

        let mut railway_mock = MockRailwayModelRepository::new();
        let rm_id = RailwayModelId::try_from("trn:railway-model:rm:test").unwrap();
        railway_mock
            .expect_find_by_id()
            .withf(move |id, _lang| *id == rm_id)
            .times(1)
            .returning(move |_, _| Ok(None));

        let mut collection_mock = MockCollectionRepository::new();
        collection_mock.expect_find_by_id().times(0);

        let mut uow = FakeCombinedUow::new(wishlist_mock, collection_mock, railway_mock);
        let cid_provider = DefaultMockIdProvider::<CollectionItemId>::new();
        let purchase_info_provider = DefaultMockIdProvider::<PurchaseInfoId>::new();

        let cmd = PurchaseWishlistItemCommand {
            wishlist_id: wid.clone(),
            wishlist_item_id: item_id_clone,
            purchase_price: MonetaryAmount::new(100, Currency::USD),
            purchase_date: NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
            seller_id: None,
            purchase_condition: None,
            model_condition: None,
        };

        let res = PurchaseWishlistItemService::execute(
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
        let item_id_clone = item_id.clone();

        let wishlist = make_wishlist(wid.clone(), item_id.clone(), WishlistStatus::Wanted);
        wishlist_mock
            .expect_find_by_id()
            .withf(move |id| *id == wid_clone)
            .times(1)
            .returning(move |_| Ok(Some(wishlist.clone())));

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

        let mut railway_mock = MockRailwayModelRepository::new();
        let rm_id = RailwayModelId::try_from("trn:railway-model:rm:test").unwrap();
        let rm_id_clone = rm_id.clone();
        let railway_model = make_railway_model(rm_id.clone());

        railway_mock
            .expect_find_by_id()
            .withf(move |id, _lang| *id == rm_id_clone)
            .times(1)
            .returning(move |_, _| Ok(Some(railway_model.clone())));

        let mut uow = FakeCombinedUow::new(wishlist_mock, collection_mock, railway_mock);
        let cid_provider = DefaultMockIdProvider::<CollectionItemId>::new();
        let purchase_info_provider = DefaultMockIdProvider::<PurchaseInfoId>::new();

        let cmd = PurchaseWishlistItemCommand {
            wishlist_id: wid.clone(),
            wishlist_item_id: item_id_clone,
            purchase_price: MonetaryAmount::new(100, Currency::USD),
            purchase_date: NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
            seller_id: None,
            purchase_condition: None,
            model_condition: None,
        };

        let res = PurchaseWishlistItemService::execute(
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
