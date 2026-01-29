use crate::core::infrastructure::error::CommandError;
use crate::core::infrastructure::runtime_id_provider::RuntimeIdProvider;
use crate::state::AppState;
use crate::tracks_inventory::application::{
    AddTrackPurchaseInput, AddTrackPurchaseUseCase, CreateTrackInventoryUseCase,
    NewTrackInventoryInput, RenameTrackInventoryInput, RenameTrackInventoryUseCase,
    SetTrackItemQuantityInput, SetTrackItemQuantityUseCase,
};
use crate::tracks_inventory::domain::{TrackInventoryId, TrackPurchaseId};
use crate::tracks_inventory::interface::command_args::{
    AddTrackPurchaseArgs, NewTrackInventoryArgs, RenameTrackInventoryArgs, SetTrackItemQuantityArgs,
};
use log::info;
use std::convert::TryInto;

/// Command handler to create a new track inventory.
///
/// # Arguments
/// - `state`: The application state.
/// - `input`: The arguments required to create a new track inventory.
///
/// # Returns
/// the ID of the newly created track inventory.
#[tauri::command]
#[specta::specta]
pub async fn create_track_inventory(
    state: tauri::State<'_, AppState>,
    input: NewTrackInventoryArgs,
) -> Result<TrackInventoryId, CommandError> {
    info!("Creating track inventory: {:?}", input);

    let mut unit_of_work = state.unit_of_work().await?;
    let id_provider = RuntimeIdProvider::new();

    let input: NewTrackInventoryInput = input.try_into()?;

    let id = CreateTrackInventoryUseCase::execute(&mut unit_of_work, id_provider, input).await?;

    unit_of_work.commit().await?;

    Ok(id)
}

/// Command handler to rename an existing track inventory.
///
/// # Arguments
/// - `state`: The application state.
/// - `input`: The arguments required to rename a track inventory.
///
/// # Returns
/// nothing on success.
#[tauri::command]
#[specta::specta]
pub async fn rename_track_inventory(
    state: tauri::State<'_, AppState>,
    input: RenameTrackInventoryArgs,
) -> Result<(), CommandError> {
    info!("Renaming track inventory: {:?}", input);

    let mut unit_of_work = state.unit_of_work().await?;

    let input: RenameTrackInventoryInput = input.try_into()?;

    RenameTrackInventoryUseCase::execute(&mut unit_of_work, input).await?;

    unit_of_work.commit().await?;

    Ok(())
}

/// Command handler to add a purchase to an existing track inventory.
///
/// # Arguments
/// - `state`: The application state.
/// - `input`: The arguments required to add a track purchase.
///
/// # Returns
/// the ID of the newly added track purchase.
#[tauri::command]
#[specta::specta]
pub async fn add_track_purchase(
    state: tauri::State<'_, AppState>,
    input: AddTrackPurchaseArgs,
) -> Result<TrackPurchaseId, CommandError> {
    info!("Adding track purchase: {:?}", input);

    let mut unit_of_work = state.unit_of_work().await?;
    let id_provider = RuntimeIdProvider::new();

    let input: AddTrackPurchaseInput = input.try_into()?;

    let id = AddTrackPurchaseUseCase::execute(&mut unit_of_work, id_provider, input).await?;

    unit_of_work.commit().await?;

    Ok(id)
}

/// Command handler to set quantity for a track item in an inventory.
///
/// # Arguments
/// - `state`: The application state.
/// - `input`: The arguments required to set the track item quantity.
///
/// # Returns
/// nothing on success.
#[tauri::command]
#[specta::specta]
pub async fn set_track_item_quantity(
    state: tauri::State<'_, AppState>,
    input: SetTrackItemQuantityArgs,
) -> Result<(), CommandError> {
    info!("Setting track item quantity: {:?}", input);

    let mut unit_of_work = state.unit_of_work().await?;

    let input: SetTrackItemQuantityInput = input.try_into()?;

    SetTrackItemQuantityUseCase::execute(&mut unit_of_work, input).await?;

    unit_of_work.commit().await?;

    Ok(())
}
