use crate::core::infrastructure::error::CommandError;
use crate::state::AppState;
use crate::wishlist::application::add_to_wishlist::AddToWishlistUseCase;
use crate::wishlist::application::create_wishlist::CreateWishlistUseCase;
use crate::wishlist::application::delete_wishlist::DeleteWishlistUseCase;
use crate::wishlist::application::get_wishlist_by_id::GetWishlistUseCase;
use crate::wishlist::application::get_wishlists::GetWishlistsUseCase;
use crate::wishlist::application::move_wishlist_item::MoveWishlistItemUseCase;
use crate::wishlist::application::remove_wishlist_item::RemoveWishlistItemUseCase;
use crate::wishlist::application::rename_wishlist::RenameWishlistUseCase;
use crate::wishlist::application::set_default_wishlist::SetDefaultWishlistUseCase;
use crate::wishlist::domain::commands::{
    AddToWishlistCommand, CreateWishlistCommand, DeleteWishlistCommand, MoveWishlistItemCommand,
    RemoveWishlistItemCommand, RenameWishlistCommand, SetDefaultWishlistCommand,
};
use crate::wishlist::domain::wishlist::Wishlist;
use crate::wishlist::domain::wishlist_id::WishlistId;
use crate::wishlist::domain::wishlist_item::WishlistItem;
use crate::wishlist::domain::wishlist_preview::WishlistPreview;
use crate::wishlist::domain::wishlist_priority::WishlistPriority;
use crate::wishlist::domain::wishlist_status::WishlistStatus;
use serde::{Deserialize, Serialize};

#[tauri::command]
#[specta::specta]
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
#[specta::specta]
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
#[specta::specta]
pub async fn create_wishlist(
    state: tauri::State<'_, AppState>,
    input: CreateWishlistInput,
) -> Result<WishlistPreview, CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;

    let cmd = CreateWishlistCommand {
        name: input.name,
        notes: input.notes,
        is_default: input.is_default.unwrap_or(false),
    };

    let use_case = CreateWishlistUseCase;

    let preview = use_case
        .execute(&mut unit_of_work, cmd)
        .await
        .map_err(CommandError::from)?;

    unit_of_work
        .commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    Ok(preview)
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RenameWishlistInput {
    pub id: String,
    pub name: String,
}

#[tauri::command]
#[specta::specta]
pub async fn rename_wishlist(
    state: tauri::State<'_, AppState>,
    input: RenameWishlistInput,
) -> Result<(), CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;

    let cmd = RenameWishlistCommand {
        id: input.id,
        name: input.name,
    };

    let use_case = RenameWishlistUseCase;

    use_case
        .execute(&mut unit_of_work, cmd)
        .await
        .map_err(CommandError::from)?;

    unit_of_work
        .commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn delete_wishlist(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<(), CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;

    let cmd = DeleteWishlistCommand { id };

    let use_case = DeleteWishlistUseCase;

    use_case
        .execute(&mut unit_of_work, cmd)
        .await
        .map_err(CommandError::from)?;

    unit_of_work
        .commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn set_default_wishlist(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<(), CommandError> {
    let mut uow = state.unit_of_work().await?;

    let cmd = SetDefaultWishlistCommand { id };

    let use_case = SetDefaultWishlistUseCase;

    use_case.execute(&mut uow, cmd).await.map_err(CommandError::from)?;

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
#[specta::specta]
pub async fn add_to_wishlist(
    state: tauri::State<'_, AppState>,
    input: AddToWishlistInput,
) -> Result<WishlistItem, CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;

    let cmd = AddToWishlistCommand {
        wishlist_id: input.wishlist_id,
        railway_model_id: input.railway_model_id,
        priority: input.priority,
        status: input.status,
        desired_price_amount: input.desired_price_amount,
        desired_price_currency: input.desired_price_currency,
        notes: input.notes,
        added_date: input.added_date,
    };

    let use_case = AddToWishlistUseCase;

    let item = use_case
        .execute(&mut unit_of_work, cmd)
        .await
        .map_err(CommandError::from)?;

    unit_of_work
        .commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    Ok(item)
}

#[tauri::command]
#[specta::specta]
pub async fn remove_from_wishlist(
    state: tauri::State<'_, AppState>,
    item_id: String,
) -> Result<(), CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;

    let cmd = RemoveWishlistItemCommand { item_id };

    let use_case = RemoveWishlistItemUseCase;

    use_case
        .execute(&mut unit_of_work, cmd)
        .await
        .map_err(CommandError::from)?;

    unit_of_work
        .commit()
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
#[specta::specta]
pub async fn move_item_to_list(
    state: tauri::State<'_, AppState>,
    input: MoveWishlistItemInput,
) -> Result<(), CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;

    let cmd = MoveWishlistItemCommand {
        item_id: input.item_id,
        destination_wishlist_id: input.destination_wishlist_id,
    };

    let use_case = MoveWishlistItemUseCase;

    use_case
        .execute(&mut unit_of_work, cmd)
        .await
        .map_err(CommandError::from)?;

    unit_of_work
        .commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    Ok(())
}
