use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::wishlist::domain::commands::CreateWishlistCommand;
use crate::wishlist::domain::wishlist::Wishlist;
use crate::wishlist::domain::wishlist_id::WishlistId;
use crate::wishlist::domain::wishlist_preview::WishlistPreview;
use crate::wishlist::infrastructure::repository::WishlistUowExt;

/// Use case responsible for creating a new wishlist aggregate.
///
/// This use case constructs a `Wishlist` from the provided command,
/// persists it via the repository exposed by the `unit_of_work`, and
/// returns a lightweight `WishlistPreview` representing the created
/// resource.
pub struct CreateWishlistUseCase;

impl CreateWishlistUseCase {
    /// Execute the create wishlist use case.
    ///
    /// - `unit_of_work`: transactional unit providing repository access.
    /// - `cmd`: validated domain command describing the wishlist to create.
    ///
    /// Returns the created `WishlistPreview` on success or a `DomainError`.
    pub async fn execute(
        unit_of_work: &mut SqliteUnitOfWork<'_>,
        cmd: CreateWishlistCommand,
    ) -> Result<WishlistPreview, DomainError> {
        let mut repo = unit_of_work.wishlist_repo();

        let wishlist = Wishlist {
            id: WishlistId::default(),
            name: cmd.name,
            notes: cmd.notes,
            is_default: cmd.is_default,
            items: Vec::new(),
        };

        repo.create_wishlist(&wishlist).await?;

        // Return the freshly created preview by listing previews and finding by id
        let previews = repo.list_wishlist_previews().await?;
        let maybe = previews
            .into_iter()
            .find(|p| p.id.to_string() == wishlist.id.to_string());

        match maybe {
            Some(p) => Ok(p),
            None => Err(DomainError::NotFound {
                resource: "WishlistPreview".to_string(),
                identifier: wishlist.id.to_string(),
            }),
        }
    }
}
