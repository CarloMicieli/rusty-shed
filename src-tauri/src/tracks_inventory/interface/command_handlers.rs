use crate::core::infrastructure::error::CommandError;
use crate::core::infrastructure::runtime_id_provider::RuntimeIdProvider;
use crate::state::AppState;
use crate::tracks_inventory::application::{
    AddTrackPurchaseInput, AddTrackPurchaseUseCase, CreateTrackInventoryUseCase,
    CreateTrackProductInput, CreateTrackProductUseCase, DeleteTrackInventoryUseCase,
    NewTrackInventoryInput, RenameTrackInventoryInput, RenameTrackInventoryUseCase,
    SetTrackItemQuantityInput, SetTrackItemQuantityUseCase,
};
use crate::tracks_inventory::domain::TracksInventoryUowExt;
use crate::tracks_inventory::domain::{TrackId, TrackInventoryId, TrackPurchaseId};
use crate::tracks_inventory::interface::command_args::{
    AddTrackPurchaseArgs, CreateTrackProductArgs, NewTrackInventoryArgs, RenameTrackInventoryArgs,
    SetItemRequiredArgs, SetTrackItemQuantityArgs,
};
use std::convert::TryInto;
use tracing::info;

// ---------------------------------------------------------------------------
// Inner (testable) implementations – take &AppState directly
// ---------------------------------------------------------------------------

pub async fn create_track_inventory_inner(
    state: &AppState,
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
    create_track_inventory_inner(&state, input).await
}

pub async fn rename_track_inventory_inner(
    state: &AppState,
    input: RenameTrackInventoryArgs,
) -> Result<(), CommandError> {
    info!("Renaming track inventory: {:?}", input);

    let mut unit_of_work = state.unit_of_work().await?;

    let input: RenameTrackInventoryInput = input.try_into()?;

    RenameTrackInventoryUseCase::execute(&mut unit_of_work, input).await?;

    unit_of_work.commit().await?;

    Ok(())
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
    rename_track_inventory_inner(&state, input).await
}

pub async fn add_track_purchase_inner(
    state: &AppState,
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
    add_track_purchase_inner(&state, input).await
}

pub async fn set_track_item_quantity_inner(
    state: &AppState,
    input: SetTrackItemQuantityArgs,
) -> Result<(), CommandError> {
    info!("Setting track item quantity: {:?}", input);

    let mut unit_of_work = state.unit_of_work().await?;

    let input: SetTrackItemQuantityInput = input.try_into()?;

    SetTrackItemQuantityUseCase::execute(&mut unit_of_work, input).await?;

    unit_of_work.commit().await?;

    Ok(())
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
    set_track_item_quantity_inner(&state, input).await
}

pub async fn delete_track_inventory_inner(
    state: &AppState,
    id: TrackInventoryId,
) -> Result<(), CommandError> {
    info!("Deleting track inventory: {:?}", id);

    let mut unit_of_work = state.unit_of_work().await?;

    DeleteTrackInventoryUseCase::execute(&mut unit_of_work, &id).await?;

    unit_of_work.commit().await?;

    Ok(())
}

/// Command handler to delete a track inventory.
///
/// # Arguments
/// - `state`: The application state.
/// - `id`: The ID of the inventory to delete.
///
/// # Returns
/// nothing on success.
#[tauri::command]
#[specta::specta]
pub async fn delete_track_inventory(
    state: tauri::State<'_, AppState>,
    id: TrackInventoryId,
) -> Result<(), CommandError> {
    delete_track_inventory_inner(&state, id).await
}

pub async fn create_track_product_inner(
    state: &AppState,
    input: CreateTrackProductArgs,
) -> Result<TrackId, CommandError> {
    info!("Creating track product: {:?}", input);

    let mut unit_of_work = state.unit_of_work().await?;
    let id_provider = RuntimeIdProvider::new();

    let input: CreateTrackProductInput = input.try_into()?;

    let id = CreateTrackProductUseCase::execute(&mut unit_of_work, id_provider, input).await?;

    unit_of_work.commit().await?;

    Ok(id)
}

/// Command handler to create a new track product.
///
/// # Arguments
/// - `state`: The application state.
/// - `input`: The arguments required to create a new track product.
///
/// # Returns
/// the ID of the newly created track product.
#[tauri::command]
#[specta::specta]
pub async fn create_track_product(
    state: tauri::State<'_, AppState>,
    input: CreateTrackProductArgs,
) -> Result<TrackId, CommandError> {
    create_track_product_inner(&state, input).await
}

pub async fn set_item_required_inner(
    state: &AppState,
    input: SetItemRequiredArgs,
) -> Result<(), CommandError> {
    info!("Setting required quantity: {:?}", input);

    let mut unit_of_work = state.unit_of_work().await?;

    // Validate required quantity
    if input.required < 0 {
        return Err(CommandError::validation_field(
            "required",
            "Required quantity cannot be negative",
        ));
    }

    let updated = {
        let mut repo = unit_of_work.track_inventories_repo();
        repo.set_item_required(&input.inventory_id, &input.track_id, input.required)
            .await
            .map_err(CommandError::from)?
    };

    if !updated {
        return Err(CommandError::NotFound(format!(
            "Track item {} not found in inventory {}",
            input.track_id, input.inventory_id
        )));
    }

    unit_of_work.commit().await?;

    Ok(())
}

/// Command handler to set the required quantity for a track item.
///
/// # Arguments
/// - `state`: The application state.
/// - `input`: The arguments specifying inventory, track, and required quantity.
///
/// # Returns
/// Unit type on success.
#[tauri::command]
#[specta::specta]
pub async fn set_item_required(
    state: tauri::State<'_, AppState>,
    input: SetItemRequiredArgs,
) -> Result<(), CommandError> {
    set_item_required_inner(&state, input).await
}
