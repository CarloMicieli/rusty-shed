use crate::core::domain::domain_error::DomainError;
use crate::wishlist::domain::wishlist::Wishlist;
use crate::wishlist::domain::wishlist_id::WishlistId;
use crate::wishlist::domain::wishlist_item::WishlistItem;
use crate::wishlist::domain::wishlist_item_id::WishlistItemId;
use crate::wishlist::domain::wishlist_preview::WishlistPreview;

/// Abstraction over wishlist persistence.
///
/// Implementations provide data access operations for wishlist aggregates and
/// lightweight previews. Methods are designed to be used within a Unit of
/// Work / transaction and return `anyhow::Result` to surface underlying DB
/// failures to callers.
#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait WishlistRepository: Send + Sync {
    /// Fetch a wishlist aggregate by its identifier.
    ///
    /// Returns `Ok(Some(Wishlist))` when the wishlist exists, `Ok(None)` when no
    /// wishlist is found for the provided id, or an `Err` when the underlying
    /// repository fails.
    async fn get_wishlist_by_id(
        &mut self,
        id: &WishlistId,
    ) -> Result<Option<Wishlist>, DomainError>;

    /// List lightweight wishlist previews.
    ///
    /// Previews contain summary information (counts and total values) and are
    /// suitable for list views where the full wishlist aggregate is not
    /// required.
    async fn list_wishlist_previews(&mut self) -> Result<Vec<WishlistPreview>, DomainError>;

    /// Create a new wishlist.
    async fn create_wishlist(&mut self, wishlist: &Wishlist) -> Result<(), DomainError>;

    /// Rename an existing wishlist.
    async fn rename_wishlist(&mut self, id: &WishlistId, name: &str) -> Result<(), DomainError>;

    /// Delete a wishlist (and cascade its items).
    async fn delete_wishlist(&mut self, id: &WishlistId) -> Result<(), DomainError>;

    /// Set a single wishlist as default by clearing previous defaults and marking the target.
    async fn set_default_wishlist(&mut self, id: &WishlistId) -> Result<(), DomainError>;

    /// Add a wishlist item to a given wishlist.
    async fn add_item(
        &mut self,
        wishlist_id: &WishlistId,
        item: &WishlistItem,
    ) -> Result<(), DomainError>;

    /// Remove a wishlist item by id.
    async fn remove_item(&mut self, item_id: &WishlistItemId) -> Result<(), DomainError>;

    /// Move a wishlist item to another wishlist.
    async fn move_item(
        &mut self,
        item_id: &WishlistItemId,
        destination_wishlist: &WishlistId,
    ) -> Result<(), DomainError>;
}

/// An extension trait that provides access to the `WishlistRepository`.
///
/// This follows the **Interface Segregation Principle**. By using extension traits,
/// we avoid a "God Object" where one struct knows about every repository in the
/// system. Instead, repositories are grouped by domain logic.
pub trait WishlistUowExt: Send {
    /// Returns a trait object for interacting with wishlist data.
    ///
    /// The repository is bound to the lifetime of the Unit of Work to ensure
    /// it cannot outlive the transaction it relies on.
    fn wishlist_repository(&mut self) -> Box<dyn WishlistRepository + '_>;
}
