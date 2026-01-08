use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::wishlist::domain::commands::SetDefaultWishlistCommand;
use crate::wishlist::infrastructure::repository::WishlistUowExt;

/// Use case that marks a wishlist as the default.
///
/// The use case ensures the provided wishlist is recorded as the single
/// default list for the user (repository handles exclusivity).
pub struct SetDefaultWishlistUseCase;

impl SetDefaultWishlistUseCase {
    /// Execute the set-default-wishlist use case.
    ///
    /// - `unit_of_work`: transactional unit providing repository access.
    /// - `cmd`: command carrying the wishlist id to mark default.
    ///
    /// Returns `()` on success or a `DomainError` on failure.
    pub async fn execute(
        unit_of_work: &mut SqliteUnitOfWork<'_>,
        cmd: SetDefaultWishlistCommand,
    ) -> Result<(), DomainError> {
        let mut repo = unit_of_work.wishlist_repo();
        repo.set_default_wishlist(&cmd.id).await?;
        Ok(())
    }
}
