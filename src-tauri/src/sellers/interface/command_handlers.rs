use crate::core::infrastructure::error::CommandError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::sellers::application::create_seller::{CreateSellerInput, CreateSellerUseCase};
use crate::sellers::application::delete_seller::DeleteSellerUseCase;
use crate::sellers::application::get_seller_by_id::GetSellerByIdUseCase;
use crate::sellers::application::get_sellers::GetSellersUseCase;
use crate::sellers::application::update_seller::{UpdateSellerInput, UpdateSellerUseCase};
use crate::sellers::domain::seller::Seller;
use crate::sellers::domain::seller_id::SellerId;
use crate::sellers::domain::seller_type::SellerType;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
#[specta::specta]
pub async fn get_sellers(state: State<'_, AppState>) -> Result<Vec<Seller>, CommandError> {
    let mut unit_of_work = SqliteUnitOfWork::new(&state.db_pool())
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    let use_case = GetSellersUseCase::new();
    match use_case.execute(&mut unit_of_work).await {
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
    state: State<'_, AppState>,
    id: String,
) -> Result<Option<Seller>, CommandError> {
    let mut uow = SqliteUnitOfWork::new(&state.db_pool())
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    let sid = SellerId::try_from(id.as_str()).map_err(|e| CommandError::Unknown(e.to_string()))?;

    let use_case = GetSellerByIdUseCase::new();
    let result = use_case
        .execute(&mut uow, &sid)
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    uow.commit()
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
    state: State<'_, AppState>,
    payload: CreateSellerPayload,
) -> Result<Seller, CommandError> {
    let mut uow = SqliteUnitOfWork::new(&state.db_pool())
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    let use_case = CreateSellerUseCase::new();
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
    let result = use_case
        .execute(&mut uow, input)
        .await
        .map_err(|e| CommandError::Unknown(e.to_string()))?;

    uow.commit()
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

#[tauri::command]
#[specta::specta]
pub async fn update_seller(
    state: State<'_, AppState>,
    payload: UpdateSellerPayload,
) -> Result<Seller, CommandError> {
    let mut uow = SqliteUnitOfWork::new(&state.db_pool())
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    let sid = SellerId::try_from(payload.id.as_str())
        .map_err(|e| CommandError::Unknown(e.to_string()))?;

    let created_at_dt = if let Some(created_at_str) = payload.created_at.as_deref() {
        match chrono::DateTime::parse_from_rfc3339(created_at_str) {
            Ok(dt) => Some(dt.with_timezone(&chrono::Utc)),
            Err(e) => return Err(CommandError::Unknown(e.to_string())),
        }
    } else {
        None
    };

    let use_case = UpdateSellerUseCase::new();
    let input = UpdateSellerInput {
        id: sid,
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
        created_at: created_at_dt,
    };
    let result = use_case
        .execute(&mut uow, input)
        .await
        .map_err(|e| CommandError::Unknown(e.to_string()))?;

    uow.commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    Ok(result)
}

#[tauri::command]
#[specta::specta]
pub async fn delete_seller(state: State<'_, AppState>, id: String) -> Result<u64, CommandError> {
    let mut uow = SqliteUnitOfWork::new(&state.db_pool())
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    let sid = SellerId::try_from(id.as_str()).map_err(|e| CommandError::Unknown(e.to_string()))?;

    let use_case = DeleteSellerUseCase::new();
    let result = use_case
        .execute(&mut uow, &sid)
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    uow.commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    Ok(result)
}
