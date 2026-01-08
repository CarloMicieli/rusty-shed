use crate::core::domain::address::{Address, AddressFields};
use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::sellers::domain::seller::Seller;
use crate::sellers::domain::seller_id::SellerId;
use crate::sellers::domain::seller_type::SellerType;
use crate::sellers::infrastructure::repository::SellersUowExt;
use chrono::Utc;

#[derive(Debug, Clone, specta::Type, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSellerInput {
    pub name: String,
    pub seller_type: SellerType,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub website_url: Option<String>,
    pub street_address: Option<String>,
    pub extended_address: Option<String>,
    pub city: Option<String>,
    pub state_region: Option<String>,
    pub postal_code: Option<String>,
    pub country_code: Option<String>,
}

pub struct CreateSellerUseCase;

impl CreateSellerUseCase {
    pub async fn execute(
        unit_of_work: &mut SqliteUnitOfWork<'_>,
        input: CreateSellerInput,
    ) -> Result<Seller, DomainError> {
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
            id: SellerId::new_from_name(&input.name),
            name: input.name,
            seller_type: input.seller_type,
            email: input.email,
            phone: input.phone,
            website_url: input.website_url,
            address,
            created_at: now,
            updated_at: now,
        };

        let mut repo = unit_of_work.sellers_repository();
        repo.upsert(&seller).await?;

        Ok(seller)
    }
}
