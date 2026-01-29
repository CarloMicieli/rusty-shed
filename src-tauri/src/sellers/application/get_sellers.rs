use crate::core::domain::domain_error::DomainError;
use crate::sellers::application::seller_view::SellerView;
use crate::sellers::domain::SellersUowExt;

pub struct GetSellers;

impl GetSellers {
    /// Retrieves a list of all sellers.
    ///
    /// # Arguments
    /// - `unit_of_work`: The unit of work providing access to the sellers repository.
    ///
    /// # Returns
    /// - `Ok(Vec<Seller>)` containing the list of sellers.
    /// - `Err(DomainError)` if an error occurred during the operation.
    ///
    /// # Type Parameters
    /// - `U`: Unit of work type implementing `SellersUowExt` and `Send`.
    pub async fn execute<U>(unit_of_work: &mut U) -> Result<Vec<SellerView>, DomainError>
    where
        U: SellersUowExt + Send,
    {
        let mut repo = unit_of_work.sellers_repository();
        let sellers = repo.list().await?;
        Ok(sellers.into_iter().map(Into::into).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::domain_error::DomainError;
    use crate::core::domain::identifiers::Identifier;
    use crate::sellers::application::testing::FakeUow;
    use crate::sellers::domain::MockSellersRepository;
    use crate::sellers::domain::seller::Seller;
    use crate::sellers::domain::seller_id::SellerId;
    use crate::sellers::domain::seller_type::SellerType;
    use chrono::Utc;

    #[tokio::test]
    async fn returns_empty_when_none() -> Result<(), DomainError> {
        let mut mock = MockSellersRepository::new();
        mock.expect_list().returning(|| Ok(vec![]));

        let mut uow = FakeUow::with_sellers_repo(Box::new(mock));
        let res = GetSellers::execute(&mut uow).await?;
        assert!(res.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn returns_views_when_found() -> Result<(), DomainError> {
        let id = SellerId::new_from_parts(&["s1"]);
        let seller = Seller {
            id: id.clone(),
            name: "Shop 1".to_string(),
            seller_type: SellerType::Shop,
            email: None,
            phone: None,
            website_url: None,
            address: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            pending_events: Vec::new(),
        };

        let mut mock = MockSellersRepository::new();
        let sellers_vec = vec![seller.clone()];
        mock.expect_list()
            .returning(move || Ok(sellers_vec.clone()));

        let mut uow = FakeUow::with_sellers_repo(Box::new(mock));
        let res = GetSellers::execute(&mut uow).await?;
        assert_eq!(res.len(), 1);
        Ok(())
    }
}
