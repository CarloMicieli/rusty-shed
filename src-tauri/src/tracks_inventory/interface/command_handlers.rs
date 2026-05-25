use crate::core::infrastructure::error::CommandError;
use crate::core::infrastructure::runtime_id_provider::RuntimeIdProvider;
use crate::state::AppState;
use crate::tracks_inventory::application::{
    AddTrackPurchaseInput, AddTrackPurchaseUseCase, CreateTrackInventoryUseCase,
    CreateTrackProductInput, CreateTrackProductUseCase, DeleteTrackInventoryUseCase,
    DeleteTrackProductInput, DeleteTrackProductTranslationInput,
    DeleteTrackProductTranslationUseCase, DeleteTrackProductUseCase, NewTrackInventoryInput,
    RenameTrackInventoryInput, RenameTrackInventoryUseCase, SetTrackItemQuantityInput,
    SetTrackItemQuantityUseCase, UpdateTrackProductInput, UpdateTrackProductUseCase,
    UpsertTrackProductTranslationInput, UpsertTrackProductTranslationUseCase,
};
use crate::tracks_inventory::domain::TracksInventoryUowExt;
use crate::tracks_inventory::domain::{TrackId, TrackInventoryId, TrackPurchaseId};
use crate::tracks_inventory::interface::command_args::{
    AddTrackPurchaseArgs, CreateTrackProductArgs, DeleteTrackProductArgs,
    DeleteTrackProductTranslationArgs, NewTrackInventoryArgs, RenameTrackInventoryArgs,
    SetItemRequiredArgs, SetTrackItemQuantityArgs, UpdateTrackProductArgs,
    UpsertTrackProductTranslationArgs,
};
use std::convert::TryInto;
use tracing::info;

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

pub async fn update_track_product_inner(
    state: &AppState,
    input: UpdateTrackProductArgs,
) -> Result<(), CommandError> {
    info!("Updating track product: {:?}", input);

    let mut unit_of_work = state.unit_of_work().await?;

    let input: UpdateTrackProductInput = input.try_into()?;

    UpdateTrackProductUseCase::execute(&mut unit_of_work, input).await?;

    unit_of_work.commit().await?;

    Ok(())
}

/// Command handler to update an existing track product.
#[tauri::command]
#[specta::specta]
pub async fn update_track_product(
    state: tauri::State<'_, AppState>,
    input: UpdateTrackProductArgs,
) -> Result<(), CommandError> {
    update_track_product_inner(&state, input).await
}

pub async fn delete_track_product_inner(
    state: &AppState,
    input: DeleteTrackProductArgs,
) -> Result<(), CommandError> {
    info!("Deleting track product: {:?}", input);

    let mut unit_of_work = state.unit_of_work().await?;

    let input: DeleteTrackProductInput = input.into();

    DeleteTrackProductUseCase::execute(&mut unit_of_work, input).await?;

    unit_of_work.commit().await?;

    Ok(())
}

/// Command handler to delete an existing track product.
#[tauri::command]
#[specta::specta]
pub async fn delete_track_product(
    state: tauri::State<'_, AppState>,
    input: DeleteTrackProductArgs,
) -> Result<(), CommandError> {
    delete_track_product_inner(&state, input).await
}

pub async fn upsert_track_product_translation_inner(
    state: &AppState,
    input: UpsertTrackProductTranslationArgs,
) -> Result<(), CommandError> {
    info!("Upserting track product translation: {:?}", input);

    let mut unit_of_work = state.unit_of_work().await?;

    let input: UpsertTrackProductTranslationInput = input.into();

    UpsertTrackProductTranslationUseCase::execute(&mut unit_of_work, input).await?;

    unit_of_work.commit().await?;

    Ok(())
}

/// Command handler to upsert one translation for a track product.
#[tauri::command]
#[specta::specta]
pub async fn upsert_track_product_translation(
    state: tauri::State<'_, AppState>,
    input: UpsertTrackProductTranslationArgs,
) -> Result<(), CommandError> {
    upsert_track_product_translation_inner(&state, input).await
}

pub async fn delete_track_product_translation_inner(
    state: &AppState,
    input: DeleteTrackProductTranslationArgs,
) -> Result<(), CommandError> {
    info!("Deleting track product translation: {:?}", input);

    let mut unit_of_work = state.unit_of_work().await?;

    let input: DeleteTrackProductTranslationInput = input.into();

    DeleteTrackProductTranslationUseCase::execute(&mut unit_of_work, input).await?;

    unit_of_work.commit().await?;

    Ok(())
}

/// Command handler to delete one translation for a track product.
#[tauri::command]
#[specta::specta]
pub async fn delete_track_product_translation(
    state: tauri::State<'_, AppState>,
    input: DeleteTrackProductTranslationArgs,
) -> Result<(), CommandError> {
    delete_track_product_translation_inner(&state, input).await
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

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    fn app_state(pool: SqlitePool) -> AppState {
        AppState::for_test(pool)
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn set_item_required_negative_required_returns_validation_error(pool: SqlitePool) {
        let state = app_state(pool);
        let input = SetItemRequiredArgs {
            inventory_id: TrackInventoryId::try_from(
                "trn:track-inventory:00000000-0000-0000-0000-000000000001",
            )
            .expect("valid inventory id"),
            track_id: TrackId::try_from("trn:track:acme:60100").expect("valid track id"),
            required: -1,
        };

        let result = set_item_required_inner(&state, input).await;

        assert!(
            matches!(result, Err(CommandError::ValidationError(_))),
            "Expected ValidationError, got: {:?}",
            result
        );
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_tracks_inventory.sql")
    )]
    async fn set_item_required_missing_item_returns_not_found(pool: SqlitePool) {
        let state = app_state(pool);
        let input = SetItemRequiredArgs {
            inventory_id: TrackInventoryId::try_from(
                "trn:track-inventory:00000000-0000-0000-0000-000000000001",
            )
            .expect("valid inventory id"),
            track_id: TrackId::try_from("trn:track:acme:99999").expect("valid track id"),
            required: 3,
        };

        let result = set_item_required_inner(&state, input).await;

        assert!(
            matches!(result, Err(CommandError::NotFound(_))),
            "Expected NotFound, got: {:?}",
            result
        );
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_tracks_inventory.sql")
    )]
    async fn set_item_required_updates_existing_row(pool: SqlitePool) {
        let state = app_state(pool.clone());
        let inventory_id =
            TrackInventoryId::try_from("trn:track-inventory:00000000-0000-0000-0000-000000000001")
                .expect("valid inventory id");
        let track_id = TrackId::try_from("trn:track:acme:60100").expect("valid track id");

        let input = SetItemRequiredArgs {
            inventory_id: inventory_id.clone(),
            track_id: track_id.clone(),
            required: 7,
        };

        set_item_required_inner(&state, input)
            .await
            .expect("update should succeed");

        let required: i64 = sqlx::query_scalar(
            "SELECT required FROM track_inventory_items WHERE inventory_id = ?1 AND track_id = ?2",
        )
        .bind(inventory_id.to_string())
        .bind(track_id.to_string())
        .fetch_one(&pool)
        .await
        .expect("required should be queryable");

        assert_eq!(required, 7);
    }
}
