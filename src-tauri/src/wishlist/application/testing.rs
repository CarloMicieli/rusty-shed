use crate::catalog::domain::railway_model::MockRailwayModelRepository;
use crate::catalog::domain::railway_model::RailwayModel;
use crate::catalog::domain::railway_model::RailwayModelId;
use crate::catalog::domain::railway_model::RailwayModelParams;
use crate::catalog::domain::railway_model::RailwayModelRepository;
use crate::catalog::domain::railway_model::RailwayModelUowExt;
use crate::catalog::domain::railway_model::RailwayModelView;
use crate::collecting::domain::Collection;
use crate::collecting::domain::CollectionId;
use crate::collecting::domain::CollectionRepository;
use crate::collecting::domain::CollectionUowExt;
use crate::collecting::domain::CollectionView;
use crate::collecting::domain::MockCollectionRepository;
use crate::core::domain::domain_error::DomainError;
use crate::wishlist::domain::MockWishlistRepository;
use crate::wishlist::domain::repository::WishlistRepository;
use crate::wishlist::domain::repository::WishlistUowExt;

#[cfg(test)]
#[derive(Default)]
pub struct FakeUow {
    repo: Option<MockWishlistRepository>,
}

impl FakeUow {
    pub fn new(repo: MockWishlistRepository) -> Self {
        Self { repo: Some(repo) }
    }
}

impl WishlistUowExt for FakeUow {
    fn wishlist_repository(&mut self) -> Box<dyn WishlistRepository + '_> {
        Box::new(self.repo.take().expect("wishlist repo already taken"))
    }
}

/// A lightweight wrapper that borrows a mock wishlist repository and
/// implements `WishlistRepository` by delegating calls.
pub struct WishlistRepoRef<'a> {
    pub inner: &'a mut MockWishlistRepository,
}

#[async_trait::async_trait]
impl<'a> WishlistRepository for WishlistRepoRef<'a> {
    async fn find_by_id(
        &mut self,
        id: &crate::wishlist::domain::wishlist_id::WishlistId,
    ) -> Result<
        Option<crate::wishlist::domain::wishlist::Wishlist>,
        crate::core::domain::domain_error::DomainError,
    > {
        self.inner.find_by_id(id).await
    }

    async fn find_wishlists(
        &mut self,
    ) -> Result<
        Vec<crate::wishlist::domain::wishlist_preview::WishlistPreview>,
        crate::core::domain::domain_error::DomainError,
    > {
        self.inner.find_wishlists().await
    }

    async fn create_wishlist(
        &mut self,
        wishlist: &crate::wishlist::domain::wishlist::Wishlist,
    ) -> Result<(), crate::core::domain::domain_error::DomainError> {
        self.inner.create_wishlist(wishlist).await
    }

    async fn save_wishlist(
        &mut self,
        wishlist: &crate::wishlist::domain::wishlist::Wishlist,
    ) -> Result<(), crate::core::domain::domain_error::DomainError> {
        self.inner.save_wishlist(wishlist).await
    }

    async fn rename_wishlist(
        &mut self,
        id: &crate::wishlist::domain::wishlist_id::WishlistId,
        name: &str,
    ) -> Result<(), crate::core::domain::domain_error::DomainError> {
        self.inner.rename_wishlist(id, name).await
    }

    async fn delete_wishlist(
        &mut self,
        id: &crate::wishlist::domain::wishlist_id::WishlistId,
    ) -> Result<(), crate::core::domain::domain_error::DomainError> {
        self.inner.delete_wishlist(id).await
    }

    async fn set_default_wishlist(
        &mut self,
        id: &crate::wishlist::domain::wishlist_id::WishlistId,
    ) -> Result<(), crate::core::domain::domain_error::DomainError> {
        self.inner.set_default_wishlist(id).await
    }

    async fn add_item(
        &mut self,
        wishlist_id: &crate::wishlist::domain::wishlist_id::WishlistId,
        item: &crate::wishlist::domain::wishlist_item::WishlistItem,
    ) -> Result<(), crate::core::domain::domain_error::DomainError> {
        self.inner.add_item(wishlist_id, item).await
    }

    async fn remove_item(
        &mut self,
        item_id: &crate::wishlist::domain::wishlist_item_id::WishlistItemId,
    ) -> Result<(), crate::core::domain::domain_error::DomainError> {
        self.inner.remove_item(item_id).await
    }

    async fn move_item(
        &mut self,
        item_id: &crate::wishlist::domain::wishlist_item_id::WishlistItemId,
        destination_wishlist: &crate::wishlist::domain::wishlist_id::WishlistId,
    ) -> Result<(), crate::core::domain::domain_error::DomainError> {
        self.inner.move_item(item_id, destination_wishlist).await
    }
}

/// Wrapper for collection repository
pub struct CollectionRepoRef<'a> {
    pub inner: &'a mut MockCollectionRepository,
}

#[async_trait::async_trait]
impl<'a> CollectionRepository for CollectionRepoRef<'a> {
    async fn find_view(&mut self) -> Result<CollectionView, DomainError> {
        self.inner.find_view().await
    }

    async fn find_by_id(&mut self, id: &CollectionId) -> Result<Option<Collection>, DomainError> {
        self.inner.find_by_id(id).await
    }

    async fn save(&mut self, collection: &mut Collection) -> Result<(), DomainError> {
        self.inner.save(collection).await
    }

    async fn find_depot_view(
        &mut self,
    ) -> Result<crate::collecting::domain::DepotView, DomainError> {
        self.inner.find_depot_view().await
    }
}

/// Wrapper for railway model repository
pub struct RailwayRepoRef<'a> {
    pub inner: &'a mut MockRailwayModelRepository,
}

#[async_trait::async_trait]
impl<'a> RailwayModelRepository for RailwayRepoRef<'a> {
    async fn create(&mut self, params: &RailwayModelParams) -> Result<RailwayModelId, DomainError> {
        self.inner.create(params).await
    }

    async fn find_by_id(
        &mut self,
        id: &RailwayModelId,
    ) -> Result<Option<RailwayModel>, DomainError> {
        self.inner.find_by_id(id).await
    }

    async fn find_view_by_id(
        &mut self,
        id: &RailwayModelId,
    ) -> Result<Option<RailwayModelView>, DomainError> {
        self.inner.find_view_by_id(id).await
    }

    async fn save(&mut self, aggregate: &mut RailwayModel) -> Result<(), DomainError> {
        self.inner.save(aggregate).await
    }
}

/// Combined fake unit of work that exposes wishlist, collection and railway model repositories.
pub struct FakeCombinedUow {
    pub wishlist: MockWishlistRepository,
    pub collection: MockCollectionRepository,
    pub railway: MockRailwayModelRepository,
}

impl FakeCombinedUow {
    pub fn new(
        wishlist: MockWishlistRepository,
        collection: MockCollectionRepository,
        railway: MockRailwayModelRepository,
    ) -> Self {
        Self {
            wishlist,
            collection,
            railway,
        }
    }
}

impl WishlistUowExt for FakeCombinedUow {
    fn wishlist_repository(&mut self) -> Box<dyn WishlistRepository + '_> {
        Box::new(WishlistRepoRef {
            inner: &mut self.wishlist,
        })
    }
}

impl CollectionUowExt for FakeCombinedUow {
    fn collections_repository(&mut self) -> Box<dyn CollectionRepository + '_> {
        Box::new(CollectionRepoRef {
            inner: &mut self.collection,
        })
    }
}

impl RailwayModelUowExt for FakeCombinedUow {
    fn railway_model_repository(&mut self) -> Box<dyn RailwayModelRepository + '_> {
        Box::new(RailwayRepoRef {
            inner: &mut self.railway,
        })
    }
}
