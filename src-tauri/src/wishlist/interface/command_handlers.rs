use specta::specta;
use tauri::State;

use crate::catalog::domain::railway_model_id::RailwayModelId;
use crate::core::infrastructure::error::CommandError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
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
    state: State<'_, AppState>,
    id: String,
) -> Result<Option<Wishlist>, CommandError> {
    // Start a unit of work (transaction)
    let mut uow = SqliteUnitOfWork::new(&state.db_pool())
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    let use_case = GetWishlistUseCase;

    let wid =
        WishlistId::try_from(id.as_str()).map_err(|e| CommandError::Unknown(e.to_string()))?;

    let result = use_case
        .execute(&mut uow, &wid)
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    uow.commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    Ok(result)
}

#[tauri::command]
#[specta]
pub async fn get_wishlists(
    state: State<'_, AppState>,
) -> Result<Vec<WishlistPreview>, CommandError> {
    let mut uow = SqliteUnitOfWork::new(&state.db_pool())
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    let use_case = GetWishlistsUseCase;

    let result = use_case
        .execute(&mut uow)
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    uow.commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

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
    state: State<'_, AppState>,
    input: CreateWishlistInput,
) -> Result<WishlistPreview, CommandError> {
    let mut uow = SqliteUnitOfWork::new(&state.db_pool())
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    let wishlist = Wishlist {
        id: WishlistId::default(),
        name: input.name,
        notes: input.notes,
        is_default: input.is_default.unwrap_or(false),
        items: vec![],
    };

    {
        let mut repo = uow.wishlist_repo();
        repo.create_wishlist(&wishlist)
            .await
            .map_err(|e| CommandError::DatabaseError(e.to_string()))?;
    }

    // Return previews to keep UI in sync
    let mut repo = uow.wishlist_repo();
    let previews = repo
        .list_wishlist_previews()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    drop(repo);

    uow.commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

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
    state: State<'_, AppState>,
    input: RenameWishlistInput,
) -> Result<(), CommandError> {
    let mut uow = SqliteUnitOfWork::new(&state.db_pool())
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    let wid = WishlistId::try_from(input.id.as_str())
        .map_err(|e| CommandError::Unknown(e.to_string()))?;

    let mut repo = uow.wishlist_repo();
    repo.rename_wishlist(&wid, &input.name)
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    drop(repo);

    uow.commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    Ok(())
}

#[tauri::command]
#[specta]
pub async fn delete_wishlist(state: State<'_, AppState>, id: String) -> Result<(), CommandError> {
    let mut uow = SqliteUnitOfWork::new(&state.db_pool())
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    let wid =
        WishlistId::try_from(id.as_str()).map_err(|e| CommandError::Unknown(e.to_string()))?;

    let mut repo = uow.wishlist_repo();
    repo.delete_wishlist(&wid)
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    drop(repo);

    uow.commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    Ok(())
}

#[tauri::command]
#[specta]
pub async fn set_default_wishlist(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), CommandError> {
    let mut uow = SqliteUnitOfWork::new(&state.db_pool())
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    let wid =
        WishlistId::try_from(id.as_str()).map_err(|e| CommandError::Unknown(e.to_string()))?;

    let mut repo = uow.wishlist_repo();
    repo.set_default_wishlist(&wid)
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    drop(repo);

    uow.commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

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
    state: State<'_, AppState>,
    input: AddToWishlistInput,
) -> Result<WishlistItem, CommandError> {
    let mut uow = SqliteUnitOfWork::new(&state.db_pool())
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    let wishlist_id = WishlistId::try_from(input.wishlist_id.as_str())
        .map_err(|e| CommandError::Unknown(e.to_string()))?;
    let railway_model_id = RailwayModelId::try_from(input.railway_model_id.as_str())
        .map_err(|e| CommandError::Unknown(e.to_string()))?;

    let desired_price = match (input.desired_price_amount, input.desired_price_currency) {
        (Some(amount), Some(code)) => {
            let currency = crate::core::domain::currency::Currency::from_code(&code)
                .map_err(|e| CommandError::Unknown(e.to_string()))?;
            Some(crate::core::domain::MonetaryAmount::new(
                amount as u64,
                currency,
            ))
        }
        _ => None,
    };

    let added_date = if let Some(s) = input.added_date {
        NaiveDate::parse_from_str(&s, "%Y-%m-%d")
            .map_err(|e| CommandError::Unknown(format!("invalid date: {e}")))?
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

    let mut repo = uow.wishlist_repo();
    repo.add_item(&wishlist_id, &item)
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    drop(repo);

    uow.commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    Ok(item)
}

#[tauri::command]
#[specta]
pub async fn remove_from_wishlist(
    state: State<'_, AppState>,
    item_id: String,
) -> Result<(), CommandError> {
    let mut uow = SqliteUnitOfWork::new(&state.db_pool())
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    let iid = WishlistItemId::try_from(item_id.as_str())
        .map_err(|e| CommandError::Unknown(e.to_string()))?;

    let mut repo = uow.wishlist_repo();
    repo.remove_item(&iid)
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    drop(repo);

    uow.commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

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
    state: State<'_, AppState>,
    input: MoveWishlistItemInput,
) -> Result<(), CommandError> {
    let mut uow = SqliteUnitOfWork::new(&state.db_pool())
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    let item_id = WishlistItemId::try_from(input.item_id.as_str())
        .map_err(|e| CommandError::Unknown(e.to_string()))?;
    let dest_id = WishlistId::try_from(input.destination_wishlist_id.as_str())
        .map_err(|e| CommandError::Unknown(e.to_string()))?;

    let mut repo = uow.wishlist_repo();
    repo.move_item(&item_id, &dest_id)
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    drop(repo);

    uow.commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    Ok(())
}
