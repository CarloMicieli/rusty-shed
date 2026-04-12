//! Query handlers for track inventory read operations.

use crate::core::infrastructure::error::CommandError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::state::AppState;
use crate::tracks_inventory::application::{
    GetTrackInventoriesQuery, GetTrackInventoryQuery, GetTrackProductsQuery,
    TrackInventoryListItem, TrackInventoryView, TrackProductView,
};
use crate::tracks_inventory::domain::TrackInventoryId;
use tracing::info;

// ---------------------------------------------------------------------------
// Inner (testable) implementations – take &AppState directly
// ---------------------------------------------------------------------------

pub async fn get_track_inventories_inner(
    state: &AppState,
) -> Result<Vec<TrackInventoryListItem>, CommandError> {
    info!("Fetching all track inventories");

    let pool = state.db_pool();
    let mut uow = SqliteUnitOfWork::new(&pool)
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    let inventories = GetTrackInventoriesQuery::execute(&mut uow).await?;

    Ok(inventories)
}

/// Query handler to fetch all track inventories with summary information.
///
/// # Arguments
/// - `state`: The application state.
///
/// # Returns
/// A list of track inventory summaries.
#[tauri::command]
#[specta::specta]
pub async fn get_track_inventories(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<TrackInventoryListItem>, CommandError> {
    get_track_inventories_inner(&state).await
}

pub async fn get_track_inventory_inner(
    state: &AppState,
    id: TrackInventoryId,
) -> Result<TrackInventoryView, CommandError> {
    info!("Fetching track inventory: {:?}", id);

    let pool = state.db_pool();
    let mut uow = SqliteUnitOfWork::new(&pool)
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    let inventory = GetTrackInventoryQuery::execute(&mut uow, &id).await?;

    Ok(inventory)
}

/// Query handler to fetch a single track inventory with full details.
///
/// # Arguments
/// - `state`: The application state.
/// - `id`: The ID of the inventory to fetch.
///
/// # Returns
/// The complete inventory view with items and purchases.
#[tauri::command]
#[specta::specta]
pub async fn get_track_inventory(
    state: tauri::State<'_, AppState>,
    id: TrackInventoryId,
) -> Result<TrackInventoryView, CommandError> {
    get_track_inventory_inner(&state, id).await
}

pub async fn get_track_products_inner(
    state: &AppState,
) -> Result<Vec<TrackProductView>, CommandError> {
    info!("Fetching all track products");

    let pool = state.db_pool();
    let mut uow = SqliteUnitOfWork::new(&pool)
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    let products = GetTrackProductsQuery::execute(&mut uow).await?;

    Ok(products)
}

/// Query handler to fetch all track products.
///
/// # Arguments
/// - `state`: The application state.
///
/// # Returns
/// A list of all track products available.
#[tauri::command]
#[specta::specta]
pub async fn get_track_products(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<TrackProductView>, CommandError> {
    get_track_products_inner(&state).await
}
