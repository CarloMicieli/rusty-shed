use crate::core::domain::domain_error::DomainError;
use crate::sellers::domain::SellersUowExt;
use crate::sellers::domain::seller::Seller;
use crate::sellers::domain::seller_id::SellerId;

pub struct GetSellerByIdUseCase;

impl GetSellerByIdUseCase {
    /// Retrieves a seller by its ID.
    ///
    /// # Arguments
    /// - `unit_of_work`: The unit of work providing access to the sellers repository.
    /// - `id`: The ID of the seller to be retrieved.
    ///
    /// # Returns
    /// - `Ok(Some(Seller))` if the seller was found.
    /// - `Ok(None)` if the seller was not found.
    /// - `Err(DomainError)` if an error occurred during the operation.
    ///
    /// # Type Parameters
    /// - `U`: Unit of work type implementing `SellersUowExt` and `Send`.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        id: &SellerId,
    ) -> Result<Option<Seller>, DomainError>
    where
        U: SellersUowExt + Send,
    {
        let mut repo = unit_of_work.sellers_repository();
        repo.get(id).await
    }
}
