use specta::specta;
use tauri::State;

use crate::core::infrastructure::error::CommandError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::state::AppState;
use crate::wishlist::application::get_wishlist_by_id::GetWishlistUseCase;
use crate::wishlist::domain::wishlist::Wishlist;

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

    let result = use_case
        .execute(&mut uow, id)
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    uow.commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    Ok(result)
}
