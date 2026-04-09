use crate::catalog::application::{SaveRailwayModel, SaveRailwayModelInput};
use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::error::CommandError;
use crate::core::infrastructure::runtime_id_provider::RuntimeIdProvider;
use crate::state::AppState;
use crate::wishlist::application::CreateWishlistUseCase;
use crate::wishlist::application::DeleteWishlistUseCase;
use crate::wishlist::application::GetWishlistByIdQuery;
use crate::wishlist::application::GetWishlistsQuery;
use crate::wishlist::application::MoveWishlistItemUseCase;
use crate::wishlist::application::RemoveWishlistItemUseCase;
use crate::wishlist::application::RenameWishlistUseCase;
use crate::wishlist::application::SetDefaultWishlistUseCase;
use crate::wishlist::application::UpdateWishlistItemUseCase;
use crate::wishlist::application::inputs::{
    AddToWishlistInput, CreateWishlistInput, DeleteWishlistInput, MoveWishlistItemInput,
    RemoveWishlistItemInput, RenameWishlistInput, SetDefaultWishlistInput, UpdateWishlistItemInput,
};
use crate::wishlist::application::purchase_wishlist_item::PurchaseWishlistItemCommand;
use crate::wishlist::application::queries::WishlistView;
use crate::wishlist::application::{AddToWishlistUseCase, PurchaseWishlistItemService};
use crate::wishlist::domain::wishlist_id::WishlistId;
use crate::wishlist::domain::wishlist_item::WishlistItem;
use crate::wishlist::domain::wishlist_preview::WishlistPreview;
use crate::wishlist::interface::PurchaseWishlistArgs;
use crate::wishlist::interface::command_args::{
    AddRailwayModelToWishListArgs, UpdateWishlistItemArgs,
};
use crate::wishlist::interface::{
    AddToWishlistArgs, CreateWishlistArgs, MoveWishlistItemArgs, RenameWishlistArgs,
};
use garde::Validate;
use log::info;
// SimplifiedRailwayModelArgs is referenced via the command args; no direct import needed here.
use crate::core::domain::{Currency, MonetaryAmount};

// ---------------------------------------------------------------------------
// Inner (testable) implementations – take &AppState directly
// ---------------------------------------------------------------------------

pub async fn get_wishlist_by_id_inner(
    state: &AppState,
    id: &WishlistId,
) -> Result<Option<WishlistView>, CommandError> {
    info!("Fetching wishlist with ID: {}", id);
    let mut unit_of_work = state.unit_of_work().await?;
    let result = GetWishlistByIdQuery::execute(&mut unit_of_work, id).await?;
    unit_of_work.commit().await?;
    Ok(result)
}

pub async fn get_wishlists_inner(state: &AppState) -> Result<Vec<WishlistView>, CommandError> {
    info!("Fetching all wishlists");
    let mut unit_of_work = state.unit_of_work().await?;
    let result = GetWishlistsQuery::execute(&mut unit_of_work).await?;
    unit_of_work.commit().await?;
    Ok(result)
}

pub async fn create_wishlist_inner(
    state: &AppState,
    input: CreateWishlistArgs,
) -> Result<WishlistPreview, CommandError> {
    info!("Creating wishlist: {:?}", input);
    let mut unit_of_work = state.unit_of_work().await?;
    let id_provider = RuntimeIdProvider::new();
    let cmd = CreateWishlistInput::try_from(input).map_err(CommandError::from)?;
    let preview = CreateWishlistUseCase::execute(&mut unit_of_work, id_provider, cmd).await?;
    unit_of_work.commit().await?;
    Ok(preview)
}

pub async fn rename_wishlist_inner(
    state: &AppState,
    input: RenameWishlistArgs,
) -> Result<(), CommandError> {
    info!("Renaming wishlist: {:?}", input);
    let mut unit_of_work = state.unit_of_work().await?;
    let cmd = RenameWishlistInput::try_from(input).map_err(CommandError::from)?;
    RenameWishlistUseCase::execute(&mut unit_of_work, cmd).await?;
    unit_of_work.commit().await?;
    Ok(())
}

pub async fn delete_wishlist_inner(state: &AppState, id: String) -> Result<(), CommandError> {
    info!("Deleting wishlist with ID: {}", id);
    let mut unit_of_work = state.unit_of_work().await?;
    let cmd = DeleteWishlistInput::try_from(id).map_err(CommandError::from)?;
    DeleteWishlistUseCase::execute(&mut unit_of_work, cmd).await?;
    unit_of_work.commit().await?;
    Ok(())
}

pub async fn set_default_wishlist_inner(state: &AppState, id: String) -> Result<(), CommandError> {
    info!("Setting default wishlist with ID: {}", id);
    let mut unit_of_work = state.unit_of_work().await?;
    let cmd = SetDefaultWishlistInput::try_from(id).map_err(CommandError::from)?;
    SetDefaultWishlistUseCase::execute(&mut unit_of_work, cmd).await?;
    unit_of_work.commit().await?;
    Ok(())
}

pub async fn add_to_wishlist_inner(
    state: &AppState,
    input: AddToWishlistArgs,
) -> Result<WishlistItem, CommandError> {
    info!("Adding item to wishlist: {:?}", input);
    let mut unit_of_work = state.unit_of_work().await?;
    let id_provider = RuntimeIdProvider::new();
    let cmd = AddToWishlistInput::try_from(input).map_err(CommandError::from)?;
    let item = AddToWishlistUseCase::execute(&mut unit_of_work, id_provider, cmd).await?;
    unit_of_work.commit().await?;
    Ok(item)
}

pub async fn remove_from_wishlist_inner(
    state: &AppState,
    item_id: String,
) -> Result<(), CommandError> {
    info!("Removing item from wishlist with ID: {}", item_id);
    let mut unit_of_work = state.unit_of_work().await?;
    let cmd = RemoveWishlistItemInput::try_from(item_id).map_err(CommandError::from)?;
    RemoveWishlistItemUseCase::execute(&mut unit_of_work, cmd).await?;
    unit_of_work.commit().await?;
    Ok(())
}

pub async fn move_item_to_list_inner(
    state: &AppState,
    input: MoveWishlistItemArgs,
) -> Result<(), CommandError> {
    info!("Moving wishlist item: {:?}", input);
    let mut unit_of_work = state.unit_of_work().await?;
    let cmd = MoveWishlistItemInput::try_from(input).map_err(CommandError::from)?;
    MoveWishlistItemUseCase::execute(&mut unit_of_work, cmd).await?;
    unit_of_work.commit().await?;
    Ok(())
}

pub async fn purchase_wishlist_item_inner(
    state: &AppState,
    input: PurchaseWishlistArgs,
) -> Result<(), CommandError> {
    info!("Purchasing wishlist item: {:?}", input);
    let mut unit_of_work = state.unit_of_work().await?;
    let collection_item_id_provider = RuntimeIdProvider::new();
    let purchase_info_id_provider = RuntimeIdProvider::new();
    let cmd = PurchaseWishlistItemCommand::try_from(input).map_err(CommandError::from)?;
    PurchaseWishlistItemService::execute(
        &mut unit_of_work,
        collection_item_id_provider,
        purchase_info_id_provider,
        cmd,
    )
    .await?;
    unit_of_work.commit().await?;
    Ok(())
}

pub async fn add_railway_model_to_wish_list_inner(
    state: &AppState,
    args: AddRailwayModelToWishListArgs,
) -> Result<(), CommandError> {
    info!("add_railway_model_to_wish_list (wishlist): {:?}", args);
    let mut unit_of_work = state.unit_of_work().await?;
    let save_input: SaveRailwayModelInput = args.railway_model.try_into()?;
    let railway_model_id = SaveRailwayModel::execute(&mut unit_of_work, save_input).await?;
    let target_wishlist_id = WishlistId::try_from(args.wishlist_id.as_str())
        .map_err(|e| CommandError::from(DomainError::Validation(e.to_string())))?;
    let id_provider = RuntimeIdProvider::new();
    let desired_price: Option<MonetaryAmount> = match (
        args.desired_price_amount,
        args.desired_price_currency.clone(),
    ) {
        (Some(amount), Some(code)) => {
            let currency = Currency::from_code(&code)
                .map_err(|e| CommandError::from(DomainError::Validation(e.to_string())))?;
            Some(MonetaryAmount::new(amount, currency))
        }
        _ => None,
    };
    let add_input = AddToWishlistInput {
        wishlist_id: target_wishlist_id,
        railway_model_id,
        priority: args.priority.unwrap_or_default(),
        status: args.status.unwrap_or_default(),
        desired_price,
        notes: args.notes,
        added_date: args.added_date.unwrap_or(chrono::Utc::now().date_naive()),
    };
    AddToWishlistUseCase::execute(&mut unit_of_work, id_provider, add_input).await?;
    unit_of_work.commit().await?;
    Ok(())
}

pub async fn update_wishlist_item_inner(
    state: &AppState,
    args: UpdateWishlistItemArgs,
) -> Result<WishlistItem, CommandError> {
    info!("Updating wishlist item: {:?}", args);
    args.validate()
        .map_err(|e| CommandError::BusinessRule(format!("Invalid update args: {e}")))?;
    let mut unit_of_work = state.unit_of_work().await?;
    let input = UpdateWishlistItemInput::try_from(args).map_err(CommandError::from)?;
    let item = UpdateWishlistItemUseCase::execute(&mut unit_of_work, input).await?;
    unit_of_work.commit().await?;
    Ok(item)
}

// ---------------------------------------------------------------------------
// Tauri command wrappers
// ---------------------------------------------------------------------------

/// Tauri command to get a wishlist by its ID.
#[tauri::command]
#[specta::specta]
pub async fn get_wishlist_by_id(
    state: tauri::State<'_, AppState>,
    id: WishlistId,
) -> Result<Option<WishlistView>, CommandError> {
    get_wishlist_by_id_inner(&state, &id).await
}

/// Tauri command to retrieve all wishlists.
#[tauri::command]
#[specta::specta]
pub async fn get_wishlists(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<WishlistView>, CommandError> {
    get_wishlists_inner(&state).await
}

/// Tauri command to create a new wishlist.
#[tauri::command]
#[specta::specta]
pub async fn create_wishlist(
    state: tauri::State<'_, AppState>,
    input: CreateWishlistArgs,
) -> Result<WishlistPreview, CommandError> {
    create_wishlist_inner(&state, input).await
}

/// Tauri command to rename an existing wishlist.
#[tauri::command]
#[specta::specta]
pub async fn rename_wishlist(
    state: tauri::State<'_, AppState>,
    input: RenameWishlistArgs,
) -> Result<(), CommandError> {
    rename_wishlist_inner(&state, input).await
}

/// Tauri command to delete a wishlist by its ID.
#[tauri::command]
#[specta::specta]
pub async fn delete_wishlist(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<(), CommandError> {
    delete_wishlist_inner(&state, id).await
}

/// Tauri command to set a wishlist as the default wishlist.
#[tauri::command]
#[specta::specta]
pub async fn set_default_wishlist(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<(), CommandError> {
    set_default_wishlist_inner(&state, id).await
}

/// Tauri command to add an item to a wishlist.
#[tauri::command]
#[specta::specta]
pub async fn add_to_wishlist(
    state: tauri::State<'_, AppState>,
    input: AddToWishlistArgs,
) -> Result<WishlistItem, CommandError> {
    add_to_wishlist_inner(&state, input).await
}

/// Tauri command to remove an item from a wishlist.
#[tauri::command]
#[specta::specta]
pub async fn remove_from_wishlist(
    state: tauri::State<'_, AppState>,
    item_id: String,
) -> Result<(), CommandError> {
    remove_from_wishlist_inner(&state, item_id).await
}

/// Tauri command to move an item from one wishlist to another.
#[tauri::command]
#[specta::specta]
pub async fn move_item_to_list(
    state: tauri::State<'_, AppState>,
    input: MoveWishlistItemArgs,
) -> Result<(), CommandError> {
    move_item_to_list_inner(&state, input).await
}

/// Tauri command to purchase a wishlist item and move it into the collection.
#[tauri::command]
#[specta::specta]
pub async fn purchase_wishlist_item(
    state: tauri::State<'_, AppState>,
    input: PurchaseWishlistArgs,
) -> Result<(), CommandError> {
    purchase_wishlist_item_inner(&state, input).await
}

/// Simplified flow: save (merge) the railway model and add it to the default wishlist.
#[tauri::command]
#[specta::specta]
pub async fn add_railway_model_to_wish_list(
    state: tauri::State<'_, AppState>,
    args: AddRailwayModelToWishListArgs,
) -> Result<(), CommandError> {
    add_railway_model_to_wish_list_inner(&state, args).await
}

/// Tauri command to update one or more editable fields on a wishlist item.
#[tauri::command]
#[specta::specta]
pub async fn update_wishlist_item(
    state: tauri::State<'_, AppState>,
    args: UpdateWishlistItemArgs,
) -> Result<WishlistItem, CommandError> {
    update_wishlist_item_inner(&state, args).await
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use sqlx::SqlitePool;

    fn app_state(pool: SqlitePool) -> AppState {
        AppState::for_test(pool)
    }

    // --- get_wishlists ---

    #[sqlx::test(migrations = "./migrations")]
    async fn get_wishlists_returns_empty(pool: SqlitePool) -> Result<()> {
        let state = app_state(pool);
        let result = get_wishlists_inner(&state).await?;
        assert!(result.is_empty());
        Ok(())
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_wishlists.sql")
    )]
    async fn get_wishlists_returns_all(pool: SqlitePool) -> Result<()> {
        let state = app_state(pool);
        let result = get_wishlists_inner(&state).await?;
        assert_eq!(result.len(), 2);
        Ok(())
    }

    // --- get_wishlist_by_id ---

    #[sqlx::test(migrations = "./migrations")]
    async fn get_wishlist_by_id_returns_none(pool: SqlitePool) -> Result<()> {
        let state = app_state(pool);
        let id = WishlistId::default();
        let result = get_wishlist_by_id_inner(&state, &id).await?;
        assert!(result.is_none());
        Ok(())
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_wishlist.sql")
    )]
    async fn get_wishlist_by_id_returns_some(pool: SqlitePool) -> Result<()> {
        let state = app_state(pool);
        let id = WishlistId::try_from("trn:wishlist:58fb6f1d-d838-44b5-b65c-21e5388ca4c9")?;
        let result = get_wishlist_by_id_inner(&state, &id).await?;
        assert!(result.is_some());
        let view = result.unwrap();
        assert_eq!(view.name, "Test Wishlist");
        let items = view.items.expect("items should be present");
        assert_eq!(items.len(), 1);
        Ok(())
    }

    // --- create_wishlist ---

    #[sqlx::test(migrations = "./migrations")]
    async fn create_wishlist_persists_and_returns_preview(pool: SqlitePool) -> Result<()> {
        let state = app_state(pool);
        let args = CreateWishlistArgs {
            name: "New List".to_string(),
            notes: Some("my notes".to_string()),
            is_default: Some(true),
        };
        let preview = create_wishlist_inner(&state, args).await?;
        assert_eq!(preview.name, "New List");
        assert_eq!(preview.notes, Some("my notes".to_string()));
        assert!(preview.is_default);

        // Verify it's retrievable
        let lists = get_wishlists_inner(&state).await?;
        assert_eq!(lists.len(), 1);
        Ok(())
    }

    // --- rename_wishlist ---

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_wishlist.sql")
    )]
    async fn rename_wishlist_updates_name(pool: SqlitePool) -> Result<()> {
        let state = app_state(pool);
        let args = RenameWishlistArgs {
            wishlist_id: "trn:wishlist:58fb6f1d-d838-44b5-b65c-21e5388ca4c9".to_string(),
            name: "Renamed List".to_string(),
        };
        rename_wishlist_inner(&state, args).await?;

        let id = WishlistId::try_from("trn:wishlist:58fb6f1d-d838-44b5-b65c-21e5388ca4c9")?;
        let view = get_wishlist_by_id_inner(&state, &id).await?.unwrap();
        assert_eq!(view.name, "Renamed List");
        Ok(())
    }

    // --- delete_wishlist ---

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_wishlist.sql")
    )]
    async fn delete_wishlist_removes_it(pool: SqlitePool) -> Result<()> {
        let state = app_state(pool);
        let id = "trn:wishlist:58fb6f1d-d838-44b5-b65c-21e5388ca4c9".to_string();
        delete_wishlist_inner(&state, id.clone()).await?;

        let wid = WishlistId::try_from(id.as_str())?;
        let result = get_wishlist_by_id_inner(&state, &wid).await?;
        assert!(result.is_none());
        Ok(())
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn delete_wishlist_not_found_returns_error(pool: SqlitePool) -> Result<()> {
        let state = app_state(pool);
        let id = WishlistId::default().to_string();
        let result = delete_wishlist_inner(&state, id).await;
        assert!(result.is_err());
        Ok(())
    }

    // --- set_default_wishlist ---

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_wishlists.sql")
    )]
    async fn set_default_wishlist_marks_correct_list(pool: SqlitePool) -> Result<()> {
        let state = app_state(pool);
        // test_wishlists.sql: wishlist 1 is not default, wishlist 2 is default
        let id = "trn:wishlist:58fb6f1d-d838-44b5-b65c-21e5388ca4c9".to_string();
        set_default_wishlist_inner(&state, id.clone()).await?;

        let lists = get_wishlists_inner(&state).await?;
        let target = lists.iter().find(|w| w.id.to_string() == id).unwrap();
        assert!(target.is_default);
        Ok(())
    }

    // --- add_to_wishlist ---

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_wishlist.sql")
    )]
    async fn add_to_wishlist_appends_item(pool: SqlitePool) -> Result<()> {
        let state = app_state(pool);
        let args = AddToWishlistArgs {
            wishlist_id: "trn:wishlist:58fb6f1d-d838-44b5-b65c-21e5388ca4c9".to_string(),
            railway_model_id: "trn:railway-model:acme:60100".to_string(),
            priority: None,
            status: None,
            desired_price_amount: Some(9900),
            desired_price_currency: Some("EUR".to_string()),
            notes: None,
            added_date: None,
        };
        let item = add_to_wishlist_inner(&state, args).await?;
        assert_eq!(
            item.railway_model_id.to_string(),
            "trn:railway-model:acme:60100"
        );

        // Verify count increased
        let id = WishlistId::try_from("trn:wishlist:58fb6f1d-d838-44b5-b65c-21e5388ca4c9")?;
        let view = get_wishlist_by_id_inner(&state, &id).await?.unwrap();
        assert_eq!(view.items.unwrap().len(), 2);
        Ok(())
    }

    // --- remove_from_wishlist ---

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_wishlist.sql")
    )]
    async fn remove_from_wishlist_removes_item(pool: SqlitePool) -> Result<()> {
        let state = app_state(pool);
        let item_id = "trn:wishlist-item:2af7578c-8857-4894-8c93-0be4b579ff25".to_string();
        remove_from_wishlist_inner(&state, item_id).await?;

        let id = WishlistId::try_from("trn:wishlist:58fb6f1d-d838-44b5-b65c-21e5388ca4c9")?;
        let view = get_wishlist_by_id_inner(&state, &id).await?.unwrap();
        assert!(view.items.unwrap().is_empty());
        Ok(())
    }

    // --- move_item_to_list ---

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_wishlists.sql")
    )]
    async fn move_item_to_list_moves_item(pool: SqlitePool) -> Result<()> {
        let state = app_state(pool);
        let args = MoveWishlistItemArgs {
            item_id: "trn:wishlist-item:2af7578c-8857-4894-8c93-0be4b579ff25".to_string(),
            destination_wishlist_id: "trn:wishlist:c9950910-96e1-47ae-8097-cd0ebbaa83f5"
                .to_string(),
            wishlist_id: "trn:wishlist:58fb6f1d-d838-44b5-b65c-21e5388ca4c9".to_string(),
        };
        move_item_to_list_inner(&state, args).await?;

        let dest_id = WishlistId::try_from("trn:wishlist:c9950910-96e1-47ae-8097-cd0ebbaa83f5")?;
        let dest = get_wishlist_by_id_inner(&state, &dest_id).await?.unwrap();
        // destination had 2 items, now has 3
        assert_eq!(dest.items.unwrap().len(), 3);
        Ok(())
    }

    // --- update_wishlist_item ---

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_wishlist.sql")
    )]
    async fn update_wishlist_item_changes_priority(pool: SqlitePool) -> Result<()> {
        use crate::wishlist::domain::wishlist_priority::WishlistPriority;

        let state = app_state(pool);
        let args = UpdateWishlistItemArgs {
            wishlist_id: "trn:wishlist:58fb6f1d-d838-44b5-b65c-21e5388ca4c9".to_string(),
            item_id: "trn:wishlist-item:2af7578c-8857-4894-8c93-0be4b579ff25".to_string(),
            priority: Some(WishlistPriority::High),
            status: None,
            desired_price_amount: None,
            desired_price_currency: None,
            added_date: None,
        };
        let item = update_wishlist_item_inner(&state, args).await?;
        assert_eq!(item.priority, WishlistPriority::High);
        Ok(())
    }
}
