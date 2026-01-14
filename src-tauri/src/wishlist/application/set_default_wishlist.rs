use crate::core::domain::domain_error::DomainError;
use crate::wishlist::domain::commands::SetDefaultWishlistCommand;
use crate::wishlist::domain::repository::WishlistUowExt;

/// Use case that marks a wishlist as the default.
///
/// The use case ensures the provided wishlist is recorded as the single
/// default list for the user (repository handles exclusivity).
pub struct SetDefaultWishlistUseCase;

impl SetDefaultWishlistUseCase {
    /// Execute the set default wishlist use case.
    ///
    /// # Arguments
    /// - `unit_of_work`: transactional unit providing repository access.
    /// - `cmd`: command carrying the wishlist id to mark default.
    ///
    /// # Returns
    /// * `()` on success
    /// * `DomainError` on failure.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        cmd: SetDefaultWishlistCommand,
    ) -> Result<(), DomainError>
    where
        U: WishlistUowExt + Send,
    {
        let mut repo = unit_of_work.wishlist_repository();
        repo.set_default_wishlist(&cmd.id).await?;
        Ok(())
    }
}
