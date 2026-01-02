use crate::core::domain::address::{Address, AddressFields};
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::sellers::domain::seller::Seller;
use crate::sellers::domain::seller_id::SellerId;
use crate::sellers::domain::seller_type::SellerType;
use crate::sellers::infrastructure::repository::SellersUowExt;
use anyhow::anyhow;
use chrono::{DateTime, Utc};

pub struct UpdateSellerUseCase;

impl UpdateSellerUseCase {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self
    }

    pub async fn execute(
        &self,
        uow: &mut SqliteUnitOfWork<'_>,
        input: UpdateSellerInput,
    ) -> anyhow::Result<Seller> {
        let now = Utc::now();
        let created_at = input.created_at.unwrap_or(now);
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
            id: input.id.clone(),
            name: input.name,
            seller_type: input.seller_type,
            email: input.email,
            phone: input.phone,
            website_url: input.website_url,
            address,
            created_at,
            updated_at: now,
        };

        let derived = SellerId::new_from_name(&seller.name);
        if seller.id != derived {
            return Err(anyhow!("seller id is immutable and must match slug"));
        }

        let mut repo = uow.sellers_repo();
        repo.upsert(&seller).await?;

        Ok(seller)
    }
}

#[derive(Debug, Clone, specta::Type, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSellerInput {
    pub id: SellerId,
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
    pub created_at: Option<DateTime<Utc>>,
}
