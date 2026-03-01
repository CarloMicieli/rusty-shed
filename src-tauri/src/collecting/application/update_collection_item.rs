use crate::collecting::domain::{CollectionUowExt, UpdateCollectionItemInput};
use crate::core::domain::domain_error::DomainError;

/// Use case to update mutable fields on a collection item.
pub struct UpdateCollectionItem;

impl UpdateCollectionItem {
    /// Execute a single-field update for an existing collection item.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: UpdateCollectionItemInput,
    ) -> Result<(), DomainError>
    where
        U: CollectionUowExt + Send,
    {
        let mut repo = unit_of_work.collections_repository();
        repo.update_item(&input).await
    }
}
