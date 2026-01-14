use crate::core::domain::domain_error::DomainError;
use crate::sellers::domain::SellersUowExt;
use crate::sellers::domain::seller::Seller;

pub struct GetSellersUseCase;

impl GetSellersUseCase {
    /// Retrieves a list of all sellers.
    ///
    /// # Arguments
    /// - `unit_of_work`: The unit of work providing access to the sellers repository.
    ///
    /// # Returns
    /// - `Ok(Vec<Seller>)` containing the list of sellers.
    /// - `Err(DomainError)` if an error occurred during the operation.
    pub async fn execute<U>(unit_of_work: &mut U) -> Result<Vec<Seller>, DomainError>
    where
        U: SellersUowExt + Send,
    {
        let mut repo = unit_of_work.sellers_repository();
        repo.list().await
    }
}
