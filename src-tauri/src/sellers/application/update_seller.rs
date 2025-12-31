use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::sellers::domain::seller::Seller;
use crate::sellers::domain::seller_id::SellerId;
use crate::sellers::domain::seller_type::SellerType;
use crate::sellers::infrastructure::repository::SellersUowExt;
use anyhow::anyhow;
use chrono::Utc;

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
        let mut seller = Seller {
            id: input.id.clone(),
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
            created_at: input.created_at.unwrap_or_default(),
            updated_at: String::new(),
        };

        let derived = SellerId::new_from_name(&seller.name);
        if seller.id != derived {
            return Err(anyhow!("seller id is immutable and must match slug"));
        }

        let now = Utc::now().to_rfc3339();
        if seller.created_at.trim().is_empty() {
            seller.created_at = now.clone();
        }
        seller.updated_at = now;

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
    pub created_at: Option<String>,
}
