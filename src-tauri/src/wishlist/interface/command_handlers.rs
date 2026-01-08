use specta::specta;

use crate::catalog::domain::railway_model::RailwayModelId;
use crate::core::domain::{Currency, MonetaryAmount};
use crate::core::infrastructure::error::CommandError;
use crate::state::AppState;
use crate::wishlist::application::get_wishlist_by_id::GetWishlistUseCase;
use crate::wishlist::application::get_wishlists::GetWishlistsUseCase;
use crate::wishlist::domain::wishlist::Wishlist;
use crate::wishlist::domain::wishlist_id::WishlistId;
use crate::wishlist::domain::wishlist_item::WishlistItem;
use crate::wishlist::domain::wishlist_item_id::WishlistItemId;
use crate::wishlist::domain::wishlist_preview::WishlistPreview;
use crate::wishlist::domain::wishlist_priority::WishlistPriority;
use crate::wishlist::domain::wishlist_status::WishlistStatus;
use crate::wishlist::infrastructure::repository::WishlistUowExt;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[tauri::command]
#[specta]
pub async fn get_wishlist_by_id(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<Option<Wishlist>, CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;

    let use_case = GetWishlistUseCase;

    let wid = WishlistId::try_from(id.as_str())
        .map_err(|e| CommandError::validation_field("id", e.to_string()))?;

    let result = use_case.execute(&mut unit_of_work, &wid).await?;

    unit_of_work.commit().await?;

    Ok(result)
}

#[tauri::command]
#[specta]
pub async fn get_wishlists(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<WishlistPreview>, CommandError> {
    let mut uow = state.unit_of_work().await?;

    let use_case = GetWishlistsUseCase;

    let result = use_case.execute(&mut uow).await?;

    uow.commit().await?;

    Ok(result)
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct CreateWishlistInput {
    pub name: String,
    pub notes: Option<String>,
    pub is_default: Option<bool>,
}

#[tauri::command]
#[specta]
pub async fn create_wishlist(
    state: tauri::State<'_, AppState>,
    input: CreateWishlistInput,
) -> Result<WishlistPreview, CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;

    let wishlist = Wishlist {
        id: WishlistId::default(),
        name: input.name,
        notes: input.notes,
        is_default: input.is_default.unwrap_or(false),
        items: vec![],
    };

    {
        let mut repo = unit_of_work.wishlist_repo();
        repo.create_wishlist(&wishlist).await?;
    }

    // Return previews to keep UI in sync
    let mut repo = unit_of_work.wishlist_repo();
    let previews = repo.list_wishlist_previews().await?;

    drop(repo);

    unit_of_work.commit().await?;

    let preview = previews
        .into_iter()
        .find(|p| p.id == wishlist.id)
        .unwrap_or_else(|| WishlistPreview {
            id: wishlist.id,
            name: wishlist.name,
            notes: wishlist.notes,
            is_default: wishlist.is_default,
            count: 0,
            updated_at: chrono::Utc::now().naive_utc(),
            total_value: std::collections::HashMap::new(),
        });

    Ok(preview)
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RenameWishlistInput {
    pub id: String,
    pub name: String,
}

#[tauri::command]
#[specta]
pub async fn rename_wishlist(
    state: tauri::State<'_, AppState>,
    input: RenameWishlistInput,
) -> Result<(), CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;

    let wishlist_id = WishlistId::try_from(input.id.as_str())
        .map_err(|e| CommandError::validation_field("id", e.to_string()))?;

    let mut repo = unit_of_work.wishlist_repo();
    repo.rename_wishlist(&wishlist_id, &input.name).await?;

    drop(repo);

    unit_of_work.commit().await?;

    Ok(())
}

#[tauri::command]
#[specta]
pub async fn delete_wishlist(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<(), CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;

    let wishlist_id = WishlistId::try_from(id.as_str())
        .map_err(|e| CommandError::validation_field("id", e.to_string()))?;

    let mut repo = unit_of_work.wishlist_repo();
    repo.delete_wishlist(&wishlist_id).await?;

    drop(repo);

    unit_of_work.commit().await?;

    Ok(())
}

#[tauri::command]
#[specta]
pub async fn set_default_wishlist(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<(), CommandError> {
    let mut uow = state.unit_of_work().await?;

    let wid = WishlistId::try_from(id.as_str())
        .map_err(|e| CommandError::validation_field("id", e.to_string()))?;

    let mut repo = uow.wishlist_repo();
    repo.set_default_wishlist(&wid).await?;

    drop(repo);

    uow.commit().await?;

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct AddToWishlistInput {
    pub wishlist_id: String,
    pub railway_model_id: String,
    pub priority: Option<WishlistPriority>,
    pub status: Option<WishlistStatus>,
    pub desired_price_amount: Option<i64>,
    pub desired_price_currency: Option<String>,
    pub notes: Option<String>,
    pub added_date: Option<String>,
}

#[tauri::command]
#[specta]
pub async fn add_to_wishlist(
    state: tauri::State<'_, AppState>,
    input: AddToWishlistInput,
) -> Result<WishlistItem, CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;

    let wishlist_id = WishlistId::try_from(input.wishlist_id.as_str())
        .map_err(|e| CommandError::validation_field("wishlist_id", e.to_string()))?;
    let railway_model_id = RailwayModelId::try_from(input.railway_model_id.as_str())
        .map_err(|e| CommandError::validation_field("railway_model_id", e.to_string()))?;

    let desired_price = match (input.desired_price_amount, input.desired_price_currency) {
        (Some(amount), Some(code)) => {
            let currency = Currency::from_code(&code).map_err(|e| {
                CommandError::validation_field("desired_price_currency", e.to_string())
            })?;
            Some(MonetaryAmount::new(amount, currency))
        }
        _ => None,
    };

    let added_date = if let Some(s) = input.added_date {
        NaiveDate::parse_from_str(&s, "%Y-%m-%d")
            .map_err(|e| CommandError::validation_field("added_date", e.to_string()))?
    } else {
        chrono::Utc::now().date_naive()
    };

    let item = WishlistItem {
        id: WishlistItemId::default(),
        railway_model_id,
        priority: input.priority.unwrap_or_default(),
        status: input.status.unwrap_or_default(),
        added_date,
        removed_date: None,
        notes: input.notes,
        desired_price,
        purchased_price: None,
    };

    let mut repo = unit_of_work.wishlist_repo();
    repo.add_item(&wishlist_id, &item).await?;

    drop(repo);

    unit_of_work.commit().await?;

    Ok(item)
}

#[tauri::command]
#[specta]
pub async fn remove_from_wishlist(
    state: tauri::State<'_, AppState>,
    item_id: String,
) -> Result<(), CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;

    let iid = WishlistItemId::try_from(item_id.as_str())
        .map_err(|e| CommandError::validation_field("item_id", e.to_string()))?;

    let mut repo = unit_of_work.wishlist_repo();
    repo.remove_item(&iid).await?;

    drop(repo);

    unit_of_work.commit().await?;

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct MoveWishlistItemInput {
    pub item_id: String,
    pub destination_wishlist_id: String,
}

#[tauri::command]
#[specta]
pub async fn move_item_to_list(
    state: tauri::State<'_, AppState>,
    input: MoveWishlistItemInput,
) -> Result<(), CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;

    let item_id = WishlistItemId::try_from(input.item_id.as_str())
        .map_err(|e| CommandError::validation_field("item_id", e.to_string()))?;
    let dest_id = WishlistId::try_from(input.destination_wishlist_id.as_str())
        .map_err(|e| CommandError::validation_field("destination_wishlist_id", e.to_string()))?;

    let mut repo = unit_of_work.wishlist_repo();
    repo.move_item(&item_id, &dest_id).await?;

    drop(repo);

    unit_of_work.commit().await?;

    Ok(())
}
