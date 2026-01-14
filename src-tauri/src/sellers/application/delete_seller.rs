use crate::core::domain::domain_error::DomainError;
use crate::sellers::domain::SellersUowExt;
use crate::sellers::domain::seller_id::SellerId;

pub struct DeleteSellerUseCase;

impl DeleteSellerUseCase {
    /// Deletes a seller by its ID.
    ///
    /// # Arguments
    /// - `unit_of_work`: The unit of work providing access to the sellers repository.
    /// - `id`: The ID of the seller to be deleted.
    ///
    /// # Returns
    /// - `Ok(u64)` containing the number of deleted records (typically 1 if the seller was found and deleted, 0 if not found).
    /// - `Err(DomainError)` if an error occurred during the operation.
    pub async fn execute<U>(unit_of_work: &mut U, id: &SellerId) -> Result<u64, DomainError>
    where
        U: SellersUowExt + Send,
    {
        let mut repo = unit_of_work.sellers_repository();
        repo.delete(id).await
    }
}
