use specta::specta;
use tauri::State;

use crate::core::infrastructure::error::CommandError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::state::AppState;
use crate::wishlist::application::get_wishlist_by_id::GetWishlistUseCase;
use crate::wishlist::application::get_wishlists::GetWishlistsUseCase;
use crate::wishlist::domain::wishlist::Wishlist;
use crate::wishlist::domain::wishlist_id::WishlistId;
use crate::wishlist::domain::wishlist_preview::WishlistPreview;

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
