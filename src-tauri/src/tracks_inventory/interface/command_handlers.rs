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
    use crate::catalog::domain::manufacturer::ManufacturerId;
    use crate::core::domain::Language;
    use crate::core::domain::currency::Currency;
    use crate::core::domain::monetary_amount::MonetaryAmount;
    use chrono::NaiveDate;
    use sqlx::SqlitePool;

    fn app_state(pool: SqlitePool) -> AppState {
        AppState::for_test(pool)
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn create_track_inventory_inner_persists_inventory_and_returns_id(pool: SqlitePool) {
        let state = app_state(pool.clone());
        let input = NewTrackInventoryArgs {
            name: "Command Handler Inventory".to_string(),
            description: Some("Created from command handler test".to_string()),
        };

        let created_id = create_track_inventory_inner(&state, input)
            .await
            .expect("create should succeed");

        let row = sqlx::query_as::<_, (String, Option<String>)>(
            "SELECT name, description FROM track_inventories WHERE id = ?1",
        )
        .bind(created_id.to_string())
        .fetch_one(&pool)
        .await
        .expect("created inventory should be persisted");

        assert_eq!(row.0, "Command Handler Inventory");
        assert_eq!(row.1.as_deref(), Some("Created from command handler test"));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn rename_track_inventory_inner_updates_persisted_name(pool: SqlitePool) {
        let state = app_state(pool.clone());

        let created_id = create_track_inventory_inner(
            &state,
            NewTrackInventoryArgs {
                name: "Initial Name".to_string(),
                description: Some("Rename flow".to_string()),
            },
        )
        .await
        .expect("create should succeed");

        rename_track_inventory_inner(
            &state,
            RenameTrackInventoryArgs {
                id: created_id.clone(),
                new_name: "Renamed By Handler".to_string(),
            },
        )
        .await
        .expect("rename should succeed");

        let persisted_name: String =
            sqlx::query_scalar("SELECT name FROM track_inventories WHERE id = ?1")
                .bind(created_id.to_string())
                .fetch_one(&pool)
                .await
                .expect("renamed inventory should be persisted");

        assert_eq!(persisted_name, "Renamed By Handler");
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_tracks_inventory.sql")
    )]
    async fn add_track_purchase_inner_persists_purchase_and_returns_id(pool: SqlitePool) {
        let state = app_state(pool.clone());
        let input = AddTrackPurchaseArgs {
            id: TrackInventoryId::try_from(
                "trn:track-inventory:00000000-0000-0000-0000-000000000001",
            )
            .expect("valid inventory id"),
            track_id: TrackId::try_from("trn:track:acme:60100").expect("valid track id"),
            quantity: 2,
            price: MonetaryAmount::new(1234, Currency::EUR),
            seller_id: None,
            purchase_date: NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
        };

        let purchase_id = add_track_purchase_inner(&state, input)
            .await
            .expect("add purchase should succeed");

        let row =
            sqlx::query_as::<_, (String, i64, i64, String)>(
                "SELECT track_id, quantity, price_amount, price_currency FROM track_purchases WHERE id = ?1",
            )
            .bind(purchase_id.to_string())
            .fetch_one(&pool)
            .await
            .expect("created purchase should be persisted");

        assert_eq!(row.0, "trn:track:acme:60100");
        assert_eq!(row.1, 2);
        assert_eq!(row.2, 1234);
        assert_eq!(row.3, "EUR");
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_tracks_inventory.sql")
    )]
    async fn set_track_item_quantity_inner_updates_quantity(pool: SqlitePool) {
        let state = app_state(pool.clone());
        let inventory_id =
            TrackInventoryId::try_from("trn:track-inventory:00000000-0000-0000-0000-000000000001")
                .expect("valid inventory id");
        let track_id = TrackId::try_from("trn:track:acme:60100").expect("valid track id");

        set_track_item_quantity_inner(
            &state,
            SetTrackItemQuantityArgs {
                inventory_id: inventory_id.clone(),
                track_id: track_id.clone(),
                quantity: 9,
            },
        )
        .await
        .expect("set quantity should succeed");

        let quantity: i64 = sqlx::query_scalar(
            "SELECT quantity FROM track_inventory_items WHERE inventory_id = ?1 AND track_id = ?2",
        )
        .bind(inventory_id.to_string())
        .bind(track_id.to_string())
        .fetch_one(&pool)
        .await
        .expect("updated quantity should be queryable");

        assert_eq!(quantity, 9);
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_tracks_inventory.sql")
    )]
    async fn create_track_product_inner_persists_track_and_returns_id(pool: SqlitePool) {
        let state = app_state(pool.clone());
        let args = CreateTrackProductArgs {
            lang: Language::English,
            manufacturer_id: ManufacturerId::try_from("trn:manufacturer:acme")
                .expect("valid manufacturer id"),
            product_code: "60199".to_string(),
            description: Some("Command handler track".to_string()),
            details: Some("Created in command handler test".to_string()),
            track_type: crate::tracks_inventory::domain::TrackType::Straight,
            track_code: crate::tracks_inventory::domain::TrackCode::Code100,
            with_roadbed: false,
            length: None,
            radius: None,
        };

        let track_id = create_track_product_inner(&state, args)
            .await
            .expect("create track product should succeed");

        let row = sqlx::query_as::<_, (String, String, String)>(
            "SELECT track_id, manufacturer_id, product_code FROM track_products WHERE track_id = ?1",
        )
        .bind(track_id.to_string())
        .fetch_one(&pool)
        .await
        .expect("created track product should be persisted");

        assert_eq!(row.0, track_id.to_string());
        assert_eq!(row.1, "trn:manufacturer:acme");
        assert_eq!(row.2, "60199");
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_tracks_inventory.sql")
    )]
    async fn update_track_product_inner_updates_existing_track(pool: SqlitePool) {
        let state = app_state(pool.clone());

        update_track_product_inner(
            &state,
            UpdateTrackProductArgs {
                track_id: TrackId::try_from("trn:track:acme:60100").expect("valid track id"),
                manufacturer_id: ManufacturerId::try_from("trn:manufacturer:acme")
                    .expect("valid manufacturer id"),
                product_code: "60100-UPDATED".to_string(),
                track_type: crate::tracks_inventory::domain::TrackType::Curve,
                track_code: crate::tracks_inventory::domain::TrackCode::Code83,
                with_roadbed: true,
                length: None,
                radius: None,
            },
        )
        .await
        .expect("update track product should succeed");

        let row = sqlx::query_as::<_, (String, i64)>(
            "SELECT product_code, with_roadbed FROM track_products WHERE track_id = ?1",
        )
        .bind("trn:track:acme:60100")
        .fetch_one(&pool)
        .await
        .expect("updated track product should be persisted");

        assert_eq!(row.0, "60100-UPDATED");
        assert_eq!(row.1, 1);
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
