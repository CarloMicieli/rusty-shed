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
    pub r#type: SellerType,
    pub url: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub website_url: Option<String>,
    pub street: Option<String>,
    pub house_number: Option<String>,
    pub city: Option<String>,
    pub state_region: Option<String>,
    pub postal_code: Option<String>,
    pub country_code: Option<String>,
}

pub struct CreateSellerUseCase;

impl CreateSellerUseCase {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self
    }

    pub async fn execute(
        &self,
        uow: &mut SqliteUnitOfWork<'_>,
        input: CreateSellerInput,
    ) -> anyhow::Result<Seller> {
        let now = Utc::now().to_rfc3339();
        let seller = Seller {
            id: SellerId::new_from_name(&input.name),
            name: input.name,
            r#type: input.r#type,
            url: input.url,
            email: input.email,
            phone: input.phone,
            website_url: input.website_url,
            street: input.street,
            house_number: input.house_number,
            city: input.city,
            state_region: input.state_region,
            postal_code: input.postal_code,
            country_code: input.country_code,
            created_at: now.clone(),
            updated_at: now,
        };

        let mut repo = uow.sellers_repo();
        repo.upsert(&seller).await?;

        Ok(seller)
    }
}
