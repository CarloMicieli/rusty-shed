use crate::core::domain::domain_error::DomainError;
use crate::sellers::domain::SellersUowExt;
use crate::sellers::domain::seller_id::SellerId;

pub struct DeleteSeller;

impl DeleteSeller {
    /// Deletes a seller by its ID.
    ///
    /// # Arguments
    /// - `unit_of_work`: The unit of work providing access to the sellers repository.
    /// - `id`: The ID of the seller to be deleted.
    ///
    /// # Returns
    /// - `Ok(u64)` containing the number of deleted records (typically 1 if the seller was found and deleted, 0 if not found).
    /// - `Err(DomainError)` if an error occurred during the operation.
    ///
    /// # Type Parameters
    /// - `U`: Unit of work type implementing `SellersUowExt` and `Send`.
    pub async fn execute<U>(unit_of_work: &mut U, id: &SellerId) -> Result<u64, DomainError>
    where
        U: SellersUowExt + Send,
    {
        let mut repo = unit_of_work.sellers_repository();
        repo.delete(id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::domain_error::DomainError;
    use crate::core::domain::identifiers::Identifier;
    use crate::sellers::application::testing::FakeUow;
    use crate::sellers::domain::MockSellersRepository;
    use crate::sellers::domain::seller_id::SellerId;

    #[tokio::test]
    async fn delete_delegates_to_repo() -> Result<(), DomainError> {
        let id = SellerId::new_from_parts(&["test"]);

        let mut mock = MockSellersRepository::new();
        mock.expect_delete().returning(|_id| Ok(1));

        let mut uow = FakeUow::with_sellers_repo(Box::new(mock));

        let affected = DeleteSeller::execute(&mut uow, &id).await?;
        assert_eq!(affected, 1);
        Ok(())
    }
}
