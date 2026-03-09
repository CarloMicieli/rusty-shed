use crate::core::domain::domain_error::DomainError;
use crate::wishlist::application::inputs::UpdateWishlistItemInput;
use crate::wishlist::domain::repository::WishlistUowExt;
use crate::wishlist::domain::wishlist_item::WishlistItem;

/// Use case that updates one or more editable fields on a wishlist item.
///
/// The aggregate validates that at least one field is provided and that the
/// `added_date` (when set) is not in the future. The repository processes the
/// `WishlistEvent::ItemUpdated` event with a targeted SQL UPDATE.
pub struct UpdateWishlistItemUseCase;

impl UpdateWishlistItemUseCase {
    /// Execute the update-wishlist-item use case.
    ///
    /// # Arguments
    /// - `unit_of_work`: transactional unit providing repository access.
    /// - `input`: command containing the wishlist/item IDs and patch fields.
    ///
    /// # Returns
    /// * The updated `WishlistItem` on success.
    /// * `DomainError` on failure (not found, validation, etc.).
    ///
    /// # Type Parameters
    /// - `U`: Unit of work type implementing `WishlistUowExt` and `Send`.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: UpdateWishlistItemInput,
    ) -> Result<WishlistItem, DomainError>
    where
        U: WishlistUowExt + Send,
    {
        let mut repo = unit_of_work.wishlist_repository();

        let maybe = repo.find_by_id(&input.wishlist_id).await?;
        let mut wishlist = maybe.ok_or_else(|| DomainError::NotFound {
            resource: "Wishlist".to_string(),
            identifier: input.wishlist_id.to_string(),
        })?;

        wishlist.update_item(
            &input.item_id,
            input.priority,
            input.status,
            input.desired_price,
            input.added_date,
        )?;

        // The item has been updated in-memory by apply_event; clone it before
        // persisting so we can return it without needing a second DB read.
        let updated_item = wishlist
            .items
            .iter()
            .find(|i| i.id == input.item_id)
            .ok_or_else(|| DomainError::NotFound {
                resource: "WishlistItem".to_string(),
                identifier: input.item_id.to_string(),
            })?
            .clone();

        repo.save_wishlist(&wishlist).await?;

        Ok(updated_item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::domain::railway_model::RailwayModelId;
    use crate::core::domain::metadata::Metadata;
    use crate::wishlist::application::testing::FakeUow;
    use crate::wishlist::domain::MockWishlistRepository;
    use crate::wishlist::domain::wishlist::Wishlist;
    use crate::wishlist::domain::wishlist_id::WishlistId;
    use crate::wishlist::domain::wishlist_item::WishlistItem;
    use crate::wishlist::domain::wishlist_item_id::WishlistItemId;
    use crate::wishlist::domain::wishlist_priority::WishlistPriority;
    use crate::wishlist::domain::wishlist_status::WishlistStatus;
    use chrono::NaiveDate;
    use mockall::predicate::eq;

    fn make_item() -> WishlistItem {
        WishlistItem {
            id: WishlistItemId::try_from("trn:wishlist-item:22222222-2222-2222-2222-222222222222")
                .unwrap(),
            railway_model_id: RailwayModelId::try_from("trn:railway-model:test:1234").unwrap(),
            priority: WishlistPriority::Normal,
            status: WishlistStatus::Wanted,
            added_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            removed_date: None,
            notes: None,
            desired_price: None,
            purchased_price: None,
        }
    }

    fn make_wishlist(wishlist_id: WishlistId, item: WishlistItem) -> Wishlist {
        Wishlist {
            id: wishlist_id,
            name: "Test".to_string(),
            notes: None,
            is_default: false,
            items: vec![item],
            pending_events: vec![],
            metadata: Metadata::default(),
        }
    }

    #[tokio::test]
    async fn it_should_update_item_and_return_updated_item() {
        let wishlist_id =
            WishlistId::try_from("trn:wishlist:11111111-1111-1111-1111-111111111111").unwrap();
        let wishlist_id_for_return = wishlist_id.clone();
        let item = make_item();

        let mut mock = MockWishlistRepository::new();

        mock.expect_find_by_id()
            .times(1)
            .with(eq(wishlist_id.clone()))
            .returning(move |_| {
                Ok(Some(make_wishlist(
                    wishlist_id_for_return.clone(),
                    make_item(),
                )))
            });

        mock.expect_save_wishlist().times(1).returning(|_| Ok(()));

        let mut uow = FakeUow::new(mock);

        let input = UpdateWishlistItemInput {
            wishlist_id,
            item_id: item.id.clone(),
            priority: Some(WishlistPriority::High),
            status: None,
            desired_price: None,
            added_date: None,
        };

        let result = UpdateWishlistItemUseCase::execute(&mut uow, input).await;
        assert!(result.is_ok());
        let updated = result.unwrap();
        assert_eq!(updated.priority, WishlistPriority::High);
        assert_eq!(updated.status, WishlistStatus::Wanted); // unchanged
    }

    #[tokio::test]
    async fn it_should_return_not_found_when_wishlist_missing() {
        let wishlist_id =
            WishlistId::try_from("trn:wishlist:99999999-9999-9999-9999-999999999999").unwrap();
        let item_id =
            WishlistItemId::try_from("trn:wishlist-item:22222222-2222-2222-2222-222222222222")
                .unwrap();

        let mut mock = MockWishlistRepository::new();
        mock.expect_find_by_id().times(1).returning(|_| Ok(None));

        let mut uow = FakeUow::new(mock);

        let input = UpdateWishlistItemInput {
            wishlist_id,
            item_id,
            priority: Some(WishlistPriority::Low),
            status: None,
            desired_price: None,
            added_date: None,
        };

        let result = UpdateWishlistItemUseCase::execute(&mut uow, input).await;
        assert!(matches!(result, Err(DomainError::NotFound { .. })));
    }
}
