use crate::core::domain::address::{Address, AddressFields};
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::identifiers::Identifier;
use crate::sellers::domain::SellersUowExt;
use crate::sellers::domain::seller::Seller;
use crate::sellers::domain::seller_event::SellerEvent;
use crate::sellers::domain::seller_id::SellerId;
use crate::sellers::domain::seller_type::SellerType;
use chrono::{DateTime, Utc};

pub struct UpdateSellerUseCase;

impl UpdateSellerUseCase {
    /// Updates an existing seller and persists the changes using the provided unit of work.
    ///
    /// # Arguments
    /// - `unit_of_work`: The unit of work providing access to the sellers repository.
    /// - `input`: The input data required to update the seller.
    ///
    /// # Returns
    /// - `Ok(Seller)` if the operation was successful, containing the updated seller.
    /// - `Err(DomainError)` if an error occurred during the operation.
    ///
    /// # Type Parameters
    /// - `U`: Unit of work type implementing `SellersUowExt` and `Send`.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: UpdateSellerInput,
    ) -> Result<Seller, DomainError>
    where
        U: SellersUowExt + Send,
    {
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
            pending_events: Vec::new(),
        };

        let derived = SellerId::new_from_parts(&[&seller.name]);
        if seller.id != derived {
            return Err(DomainError::BusinessRule(
                "seller id is immutable and must match slug".to_string(),
            ));
        }

        let mut seller = seller;
        seller.pending_events.push(SellerEvent::Updated {
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
