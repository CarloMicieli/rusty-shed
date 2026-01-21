use crate::core::infrastructure::error::CommandError;
use crate::core::infrastructure::runtime_id_provider::RuntimeIdProvider;
use crate::state::AppState;
use crate::tracks_inventory::application::{
    AddTrackPurchaseInput, NewTrackInventoryInput, RenameTrackInventoryInput,
    SetTrackItemQuantityInput,
};
use crate::tracks_inventory::application::{
    AddTrackPurchaseUseCase, CreateTrackInventoryUseCase, RenameTrackInventoryUseCase,
    SetTrackItemQuantityUseCase,
};
use log::info;

/// Create a new track inventory.
#[tauri::command]
#[specta::specta]
pub async fn create_track_inventory(
    state: tauri::State<'_, AppState>,
    input: NewTrackInventoryInput,
) -> Result<crate::tracks_inventory::domain::TrackInventoryId, CommandError> {
    info!("Creating track inventory: {:?}", input);

    let mut unit_of_work = state.unit_of_work().await?;
    let id_provider = RuntimeIdProvider::new();

    let id = CreateTrackInventoryUseCase::execute(&mut unit_of_work, id_provider, input).await?;

    unit_of_work.commit().await?;

    Ok(id)
}

/// Rename an existing track inventory.
#[tauri::command]
#[specta::specta]
pub async fn rename_track_inventory(
    state: tauri::State<'_, AppState>,
    input: RenameTrackInventoryInput,
) -> Result<(), CommandError> {
    info!("Renaming track inventory: {:?}", input);

    let mut unit_of_work = state.unit_of_work().await?;

    RenameTrackInventoryUseCase::execute(&mut unit_of_work, input).await?;

    unit_of_work.commit().await?;

    Ok(())
}

/// Add a purchase to an existing track inventory.
#[tauri::command]
#[specta::specta]
pub async fn add_track_purchase(
    state: tauri::State<'_, AppState>,
    input: AddTrackPurchaseInput,
) -> Result<crate::tracks_inventory::domain::TrackPurchaseId, CommandError> {
    info!("Adding track purchase: {:?}", input);

    let mut unit_of_work = state.unit_of_work().await?;
    let id_provider = RuntimeIdProvider::new();

    let id = AddTrackPurchaseUseCase::execute(&mut unit_of_work, id_provider, input).await?;

    unit_of_work.commit().await?;

    Ok(id)
}

/// Set quantity for a track item in an inventory.
#[tauri::command]
#[specta::specta]
pub async fn set_track_item_quantity(
    state: tauri::State<'_, AppState>,
    input: SetTrackItemQuantityInput,
) -> Result<(), CommandError> {
    info!("Setting track item quantity: {:?}", input);

    let mut unit_of_work = state.unit_of_work().await?;

    SetTrackItemQuantityUseCase::execute(&mut unit_of_work, input).await?;

    unit_of_work.commit().await?;

    Ok(())
}
