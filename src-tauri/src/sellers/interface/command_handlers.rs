use crate::core::infrastructure::error::CommandError;
use crate::sellers::application::create_seller::{CreateSellerInput, CreateSellerUseCase};
use crate::sellers::application::delete_seller::DeleteSellerUseCase;
use crate::sellers::application::get_seller_by_id::GetSellerByIdUseCase;
use crate::sellers::application::get_sellers::GetSellersUseCase;
use crate::sellers::application::update_seller::{UpdateSellerInput, UpdateSellerUseCase};
use crate::sellers::domain::seller::Seller;
use crate::sellers::domain::seller_id::SellerId;
use crate::sellers::domain::seller_type::SellerType;
use crate::state::AppState;
use std::convert::TryFrom;

#[tauri::command]
#[specta::specta]
pub async fn get_sellers(state: tauri::State<'_, AppState>) -> Result<Vec<Seller>, CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;

    match GetSellersUseCase::execute(&mut unit_of_work).await {
        Ok(sellers) => {
            unit_of_work
                .commit()
                .await
                .map_err(|e| CommandError::DatabaseError(e.to_string()))?;
            Ok(sellers)
        }
        Err(e) => Err(e.into()),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_seller_by_id(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<Option<Seller>, CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;

    let sid = SellerId::try_from(id.as_str())
        .map_err(|e| CommandError::validation_field("id", e.to_string()))?;

    let result = GetSellerByIdUseCase::execute(&mut unit_of_work, &sid)
        .await
        .map_err(CommandError::from)?;

    unit_of_work
        .commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    Ok(result)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct CreateSellerPayload {
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

#[tauri::command]
#[specta::specta]
pub async fn create_seller(
    state: tauri::State<'_, AppState>,
    payload: CreateSellerPayload,
) -> Result<Seller, CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;

    let input = CreateSellerInput {
        name: payload.name,
        seller_type: payload.seller_type,
        email: payload.email,
        phone: payload.phone,
        website_url: payload.website_url,
        street_address: payload.street_address,
        extended_address: payload.extended_address,
        city: payload.city,
        state_region: payload.state_region,
        postal_code: payload.postal_code,
        country_code: payload.country_code,
    };
    let result = CreateSellerUseCase::execute(&mut unit_of_work, input)
        .await
        .map_err(CommandError::from)?;

    unit_of_work
        .commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    Ok(result)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSellerPayload {
    pub id: String,
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
    pub created_at: Option<String>,
}

impl TryFrom<UpdateSellerPayload> for UpdateSellerInput {
    type Error = CommandError;

    fn try_from(payload: UpdateSellerPayload) -> Result<Self, Self::Error> {
        let UpdateSellerPayload {
            id,
            name,
            seller_type,
            email,
            phone,
            website_url,
            street_address,
            extended_address,
            city,
            state_region,
            postal_code,
            country_code,
            created_at,
        } = payload;

        let sid = SellerId::try_from(id.as_str())
            .map_err(|e| CommandError::validation_field("id", e.to_string()))?;

        let created_at_dt = if let Some(created_at_str) = created_at {
            match chrono::DateTime::parse_from_rfc3339(created_at_str.as_str()) {
                Ok(dt) => Some(dt.with_timezone(&chrono::Utc)),
                Err(e) => return Err(CommandError::validation_field("createdAt", e.to_string())),
            }
        } else {
            None
        };

        Ok(UpdateSellerInput {
            id: sid,
            name,
            seller_type,
            email,
            phone,
            website_url,
            street_address,
            extended_address,
            city,
            state_region,
            postal_code,
            country_code,
            created_at: created_at_dt,
        })
    }
}

#[tauri::command]
#[specta::specta]
pub async fn update_seller(
    state: tauri::State<'_, AppState>,
    payload: UpdateSellerPayload,
) -> Result<Seller, CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;
    let input = UpdateSellerInput::try_from(payload)?;
    let result = UpdateSellerUseCase::execute(&mut unit_of_work, input)
        .await
        .map_err(CommandError::from)?;

    unit_of_work
        .commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    Ok(result)
}

#[tauri::command]
#[specta::specta]
pub async fn delete_seller(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<u64, CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;

    let sid = SellerId::try_from(id.as_str())
        .map_err(|e| CommandError::validation_field("id", e.to_string()))?;

    let result = DeleteSellerUseCase::execute(&mut unit_of_work, &sid)
        .await
        .map_err(CommandError::from)?;

    unit_of_work
        .commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    Ok(result)
}
