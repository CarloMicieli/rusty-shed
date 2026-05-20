use crate::core::domain::domain_error::DomainError;
use crate::sellers::application::seller_view::SellerView;
use crate::sellers::domain::SellersUowExt;
use crate::sellers::domain::seller_id::SellerId;

pub struct GetSellerById;

impl GetSellerById {
    /// Retrieves a seller by its ID.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        id: &SellerId,
    ) -> Result<Option<SellerView>, DomainError>
    where
        U: SellersUowExt + Send,
    {
        let mut repo = unit_of_work.sellers_repository();
        // repository provides a dedicated view lookup to avoid loading/pulling events
        // for the full aggregate when only a presentation model is required.
        repo.find_seller_view_by_id(id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::domain_error::DomainError;
    use crate::core::domain::identifiers::Identifier;
    use crate::sellers::application::seller_view::SellerView;
    use crate::sellers::application::testing::FakeUow;
    use crate::sellers::domain::MockSellersRepository;
    use crate::sellers::domain::seller_id::SellerId;
    use crate::sellers::domain::seller_type::SellerType;

    #[tokio::test]
    async fn returns_some_when_found() -> Result<(), DomainError> {
        let id = SellerId::new_from_parts(&["found"]);

        let view = SellerView {
            id: id.clone(),
            name: "Found".to_string(),
            seller_type: SellerType::Shop,
            email: None,
            phone: None,
            website_url: None,
            address: None,
            is_system_seeded: false,
            usage_count: 0,
        };

        let mut mock = MockSellersRepository::new();
        let view_clone = view.clone();
        mock.expect_find_seller_view_by_id()
            .returning(move |_id| Ok(Some(view_clone.clone())));

        let mut uow = FakeUow::with_sellers_repo(Box::new(mock));

        let res = GetSellerById::execute(&mut uow, &id).await?;
        assert!(res.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn returns_none_when_not_found() -> Result<(), DomainError> {
        let id = SellerId::new_from_parts(&["missing"]);

        let mut mock = MockSellersRepository::new();
        mock.expect_find_seller_view_by_id()
            .returning(|_id| Ok(None));

        let mut uow = FakeUow::with_sellers_repo(Box::new(mock));

        let res = GetSellerById::execute(&mut uow, &id).await?;
        assert!(res.is_none());
        Ok(())
    }
}
