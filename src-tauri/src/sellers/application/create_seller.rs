use crate::core::domain::address::{Address, AddressFields};
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::identifiers::Identifier;
use crate::sellers::domain::SellersUowExt;
use crate::sellers::domain::seller::Seller;
use crate::sellers::domain::seller_event::SellerEvent;
use crate::sellers::domain::seller_id::SellerId;
use crate::sellers::domain::seller_type::SellerType;
use chrono::Utc;

pub struct CreateSeller;

impl CreateSeller {
    /// Creates a new seller and persists it using the provided unit of work.
    ///
    /// # Arguments
    /// - `unit_of_work`: The unit of work providing access to the sellers repository.
    /// - `input`: The input data required to create a new seller.
    ///
    /// # Returns
    /// - `Ok(Seller)` if the operation was successful, containing the created seller.
    /// - `Err(DomainError)` if an error occurred during the operation.
    ///
    /// # Type Parameters
    /// - `U`: Unit of work type implementing `SellersUowExt` and `Send`.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: CreateSellerInput,
    ) -> Result<Seller, DomainError>
    where
        U: SellersUowExt + Send,
    {
        let now = Utc::now();
        let address_fields = AddressFields {
            street: input.street_address.clone(),
            extended: input.extended_address.clone(),
            city: input.city.clone(),
            region: input.state_region.clone(),
            postal: input.postal_code.clone(),
            country: input.country_code.clone(),
        };
        let address = Address::try_from(address_fields).ok();

        let seller = Seller {
            id: SellerId::new_from_parts(&[&input.name]),
            name: input.name,
            seller_type: input.seller_type,
            email: input.email,
            phone: input.phone,
            website_url: input.website_url,
            address,
            created_at: now,
            updated_at: now,
            pending_events: Vec::new(),
        };

        let mut seller = seller;
        seller.pending_events.push(SellerEvent::Created {
            aggregate_id: seller.id.clone(),
            name: seller.name.clone(),
            seller_type: seller.seller_type.clone(),
            email: seller.email.clone(),
            phone: seller.phone.clone(),
            website_url: seller.website_url.clone(),
            address: seller.address.clone(),
            created_at: seller.created_at,
            updated_at: seller.updated_at,
        });

        let mut repo = unit_of_work.sellers_repository();
        repo.save(&mut seller).await?;

        Ok(seller)
    }
}

/// Input data required to create a new seller.
#[derive(Debug, Clone)]
pub struct CreateSellerInput {
    /// Name of the seller.
    pub name: String,
    /// Type of the seller.
    pub seller_type: SellerType,
    /// Contact email of the seller.
    pub email: Option<String>,
    /// Contact phone number of the seller.
    pub phone: Option<String>,
    /// Website URL of the seller.
    pub website_url: Option<String>,
    pub street_address: Option<String>,
    pub extended_address: Option<String>,
    pub city: Option<String>,
    pub state_region: Option<String>,
    pub postal_code: Option<String>,
    pub country_code: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::domain_error::DomainError;
    use crate::sellers::application::testing::FakeUow;
    use crate::sellers::domain::MockSellersRepository;
    use crate::sellers::domain::seller_type::SellerType;

    #[tokio::test]
    async fn create_saves_seller() -> Result<(), DomainError> {
        let mut mock = MockSellersRepository::new();
        mock.expect_save().returning(|_seller| Ok(()));

        let mut uow = FakeUow::with_sellers_repo(Box::new(mock));
        let input = CreateSellerInput {
            name: "Test Shop".to_string(),
            seller_type: SellerType::Shop,
            email: None,
            phone: None,
            website_url: None,
            street_address: None,
            extended_address: None,
            city: None,
            state_region: None,
            postal_code: None,
            country_code: None,
        };

        let seller = CreateSeller::execute(&mut uow, input).await?;
        assert_eq!(seller.name, "Test Shop");
        Ok(())
    }
}
