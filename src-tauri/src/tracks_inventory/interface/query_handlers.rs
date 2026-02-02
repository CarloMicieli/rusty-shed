//! Query handlers for track inventory read operations.

use crate::core::infrastructure::error::CommandError;
use crate::state::AppState;
use crate::tracks_inventory::application::{
    GetTrackInventoriesQuery, GetTrackInventoryQuery, GetTrackProductsQuery,
    TrackInventoryListItem, TrackInventoryView, TrackProductView,
};
use crate::tracks_inventory::domain::TrackInventoryId;
use log::info;

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
    info!("Fetching all track inventories");

    let mut unit_of_work = state.unit_of_work().await?;

    let inventories = GetTrackInventoriesQuery::execute(&mut unit_of_work).await?;

    Ok(inventories)
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
    info!("Fetching track inventory: {:?}", id);

    let mut unit_of_work = state.unit_of_work().await?;

    let inventory = GetTrackInventoryQuery::execute(&mut unit_of_work, &id).await?;

    Ok(inventory)
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
    info!("Fetching all track products");

    let mut unit_of_work = state.unit_of_work().await?;

    let products = GetTrackProductsQuery::execute(&mut unit_of_work).await?;

    Ok(products)
}
