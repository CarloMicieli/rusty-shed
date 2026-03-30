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

/// Tauri command to get a wishlist by its ID.
///
/// This handler retrieves a wishlist using the provided ID. It constructs the necessary
/// repository and query handler, executes the query asynchronously, and returns the
/// `Wishlist` on success. On failure, it converts the error into a `CommandError
/// preserving the error message for logging/debugging.
///
/// Parameters:
/// * `state`: Tauri-managed application state which provides a database pool.
/// * `id`: The identifier of the wishlist to retrieve.
///
/// Returns:
/// - `Ok(Some(WishlistView))` when a matching wishlist exists,
/// - `Ok(None)` when no matching row is found
/// - `Err(CommandError)` when the ID cannot be parsed or a database error occurs.
#[tauri::command]
#[specta::specta]
pub async fn get_wishlist_by_id(
    state: tauri::State<'_, AppState>,
    id: WishlistId,
) -> Result<Option<WishlistView>, CommandError> {
    get_wishlist_by_id_ref(state, &id).await
}

/// Helper that accepts a reference to `WishlistId`.
pub async fn get_wishlist_by_id_ref(
    state: tauri::State<'_, AppState>,
    id: &WishlistId,
) -> Result<Option<WishlistView>, CommandError> {
    info!("Fetching wishlist with ID: {}", id);

    let mut unit_of_work = state.unit_of_work().await?;

    let result = GetWishlistByIdQuery::execute(&mut unit_of_work, id).await?;
    unit_of_work.commit().await?;

    Ok(result)
}

/// Tauri command to retrieve all wishlists.
///
/// This handler constructs the repository and query handler, executes the query
/// asynchronously and returns the list of `WishlistPreview` on success. On failure, it
/// converts the error into a `CommandError` preserving the error
/// message for logging/debugging.
///
/// Parameters:
/// * `state`: Tauri-managed application state which provides a database pool.
///
/// Returns:
/// - `Ok(Vec<WishlistView>)` when retrieval succeeds.
/// - `Err(CommandError)` when the use-case returns an error.
#[tauri::command]
#[specta::specta]
pub async fn get_wishlists(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<WishlistView>, CommandError> {
    info!("Fetching all wishlists");

    let mut unit_of_work = state.unit_of_work().await?;

    let result = GetWishlistsQuery::execute(&mut unit_of_work).await?;
    unit_of_work.commit().await?;

    Ok(result)
}

/// Tauri command to create a new wishlist.
///
/// This handler constructs the repository and command handler, executes the command
/// asynchronously and returns the created `WishlistPreview` on success. On failure, it
/// converts the error into a `CommandError` preserving the error message for logging/debugging.
///
/// Parameters:
/// * `state`: Tauri-managed application state which provides a database pool.
/// * `input`: The input data required to create a new wishlist (`CreateWishlistInput`).
///
/// Returns:
/// - `Ok(WishlistPreview)` when creation succeeds.
/// - `Err(CommandError)` when validation fails, a database error occurs, or business logic rejects the operation.
#[tauri::command]
#[specta::specta]
pub async fn create_wishlist(
    state: tauri::State<'_, AppState>,
    input: CreateWishlistArgs,
) -> Result<WishlistPreview, CommandError> {
    info!("Creating wishlist: {:?}", input);

    input.validate().map_err(CommandError::from)?;

    let mut unit_of_work = state.unit_of_work().await?;
    let id_provider = RuntimeIdProvider::new();

    let cmd = CreateWishlistInput::try_from(input).map_err(CommandError::from)?;

    let preview = CreateWishlistUseCase::execute(&mut unit_of_work, id_provider, cmd).await?;

    unit_of_work.commit().await?;

    Ok(preview)
}

/// Tauri command to rename an existing wishlist.
///
/// This handler constructs the repository and command handler, executes the command
/// asynchronously and returns nothing on success. On failure, it converts the error
/// into a `CommandError` preserving the error message for logging/debugging.
///
/// Parameters:
/// * `state`: Tauri-managed application state which provides a database pool.
/// * `input`: The input data required to rename a wishlist (`RenameWishlistInput`).
///
/// Returns:
/// - `Ok(())` when renaming succeeds.
/// - `Err(CommandError)` when validation fails, a database error occurs, or business logic
#[tauri::command]
#[specta::specta]
pub async fn rename_wishlist(
    state: tauri::State<'_, AppState>,
    input: RenameWishlistArgs,
) -> Result<(), CommandError> {
    info!("Renaming wishlist: {:?}", input);

    input.validate().map_err(CommandError::from)?;

    let mut unit_of_work = state.unit_of_work().await?;

    let cmd = RenameWishlistInput::try_from(input).map_err(CommandError::from)?;

    RenameWishlistUseCase::execute(&mut unit_of_work, cmd).await?;

    unit_of_work.commit().await?;

    Ok(())
}

/// Tauri command to delete a wishlist by its ID.
///
/// This handler constructs the repository and command handler, executes the command
/// asynchronously and returns nothing on success. On failure, it converts the error
/// into a `CommandError` preserving the error message for logging/debugging.
///
/// Parameters:
/// * `state`: Tauri-managed application state which provides a database pool.
/// * `id`: The identifier of the wishlist to delete.
///
/// Returns:
/// - `Ok(())` when the deletion succeeds.
/// - `Err(CommandError)` when the use-case returns an error.
#[tauri::command]
#[specta::specta]
pub async fn delete_wishlist(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<(), CommandError> {
    info!("Deleting wishlist with ID: {}", id);

    let mut unit_of_work = state.unit_of_work().await?;

    let cmd = DeleteWishlistInput::try_from(id).map_err(CommandError::from)?;

    DeleteWishlistUseCase::execute(&mut unit_of_work, cmd).await?;

    unit_of_work.commit().await?;

    Ok(())
}

/// Tauri command to set a wishlist as the default wishlist.
///
/// This handler constructs the repository and command handler, executes the command
/// asynchronously and returns nothing on success. On failure, it converts the error
/// into a `CommandError` preserving the error message for logging/debugging.
///
/// Parameters:
/// * `state`: Tauri-managed application state which provides a database pool.
/// * `id`: The identifier of the wishlist to set as default.
///
/// Returns:
/// - `Ok(())` when the operation succeeds.
/// - `Err(CommandError)` when the use-case returns an error.
#[tauri::command]
#[specta::specta]
pub async fn set_default_wishlist(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<(), CommandError> {
    info!("Setting default wishlist with ID: {}", id);

    let mut unit_of_work = state.unit_of_work().await?;

    let cmd = SetDefaultWishlistInput::try_from(id).map_err(CommandError::from)?;

    SetDefaultWishlistUseCase::execute(&mut unit_of_work, cmd).await?;

    unit_of_work.commit().await?;

    Ok(())
}

/// Tauri command to add an item to a wishlist.
///
/// This handler constructs the repository and command handler, executes the command
/// asynchronously and returns the added `WishlistItem` on success. On failure, it
/// converts the error into a `CommandError` preserving the error message for logging/debugging.
///
/// Parameters:
/// * `state`: Tauri-managed application state which provides a database pool.
/// * `input`: The input data required to add an item to a wishlist (`AddToWishlistInput`).
///
/// Returns:
/// - `Ok(WishlistItem)` when the addition succeeds.
/// - `Err(CommandError)` when validation fails, a database error occurs, or business logic rejects the operation.
#[tauri::command]
#[specta::specta]
pub async fn add_to_wishlist(
    state: tauri::State<'_, AppState>,
    input: AddToWishlistArgs,
) -> Result<WishlistItem, CommandError> {
    info!("Adding item to wishlist: {:?}", input);

    input.validate().map_err(CommandError::from)?;

    let mut unit_of_work = state.unit_of_work().await?;
    let id_provider = RuntimeIdProvider::new();

    let cmd = AddToWishlistInput::try_from(input).map_err(CommandError::from)?;

    let item = AddToWishlistUseCase::execute(&mut unit_of_work, id_provider, cmd).await?;

    unit_of_work.commit().await?;

    Ok(item)
}

/// Tauri command to remove an item from a wishlist.
///
/// This handler constructs the repository and command handler, executes the command
/// asynchronously and returns nothing on success. On failure, it converts the error
/// into a `CommandError` preserving the error message for logging/debugging.
///
/// Parameters:
/// * `state`: Tauri-managed application state which provides a database pool.
/// * `item_id`: The identifier of the wishlist item to remove.
///
/// Returns:
/// - `Ok(())` when removal succeeds.   
/// - `Err(CommandError)` when validation fails, a database error occurs, or business logic rejects the operation.
#[tauri::command]
#[specta::specta]
pub async fn remove_from_wishlist(
    state: tauri::State<'_, AppState>,
    item_id: String,
) -> Result<(), CommandError> {
    info!("Removing item from wishlist with ID: {}", item_id);

    let mut unit_of_work = state.unit_of_work().await?;

    let cmd = RemoveWishlistItemInput::try_from(item_id).map_err(CommandError::from)?;

    RemoveWishlistItemUseCase::execute(&mut unit_of_work, cmd).await?;

    unit_of_work.commit().await?;

    Ok(())
}

/// Tauri command to move an item from one wishlist to another.
///
/// This handler constructs the repository and command handler, executes the command
/// asynchronously and returns nothing on success. On failure, it converts the error
/// into a `CommandError` preserving the error message for logging/debugging.
///
/// Parameters:
/// * `state`: Tauri-managed application state which provides a database pool.
/// * `input`: The input data required to move a wishlist item (`MoveWishlistItemInput`).
///
/// Returns:
/// - `Ok(())` when the move succeeds.
/// - `Err(CommandError)` when validation fails, a database error occurs, or business logic rejects the operation.
#[tauri::command]
#[specta::specta]
pub async fn move_item_to_list(
    state: tauri::State<'_, AppState>,
    input: MoveWishlistItemArgs,
) -> Result<(), CommandError> {
    info!("Moving wishlist item: {:?}", input);

    input.validate().map_err(CommandError::from)?;

    let mut unit_of_work = state.unit_of_work().await?;

    let cmd = MoveWishlistItemInput::try_from(input).map_err(CommandError::from)?;

    MoveWishlistItemUseCase::execute(&mut unit_of_work, cmd).await?;

    unit_of_work.commit().await?;

    Ok(())
}

/// Tauri command to purchase a wishlist item and move it into the collection.
///
/// # Arguments
/// * `state`: Tauri-managed application state which provides a database pool.
/// * `input`: The input data required to purchase a wishlist item (`PurchaseWishlistArgs`).
///
/// # Returns
/// - `Ok(())` when the purchase and move succeeds.
/// - `Err(CommandError)` when validation fails, a database error occurs, or business logic rejects the operation.
#[tauri::command]
#[specta::specta]
pub async fn purchase_wishlist_item(
    state: tauri::State<'_, AppState>,
    input: PurchaseWishlistArgs,
) -> Result<(), CommandError> {
    info!("Purchasing wishlist item: {:?}", input);

    input.validate().map_err(CommandError::from)?;

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

/// Simplified flow: save (merge) the railway model and add it to the default wishlist.
#[tauri::command]
#[specta::specta]
pub async fn add_railway_model_to_wish_list(
    state: tauri::State<'_, AppState>,
    args: AddRailwayModelToWishListArgs,
) -> Result<(), CommandError> {
    info!("add_railway_model_to_wish_list (wishlist): {:?}", args);

    args.validate().map_err(CommandError::from)?;

    let mut unit_of_work = state.unit_of_work().await?;

    let save_input: SaveRailwayModelInput = args.railway_model.try_into()?;

    let railway_model_id = SaveRailwayModel::execute(&mut unit_of_work, save_input).await?;

    // Use the provided wishlist id (required by the args).
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

/// Tauri command to update one or more editable fields on a wishlist item.
///
/// # Arguments
/// * `state`: Tauri-managed application state which provides a database pool.
/// * `args`: Transport DTO carrying the patch fields (all optional except wishlist/item IDs).
///
/// # Returns
/// - `Ok(WishlistItem)` — the full updated item — on success.
/// - `Err(CommandError)` when validation, domain, or database errors occur.
#[tauri::command]
#[specta::specta]
pub async fn update_wishlist_item(
    state: tauri::State<'_, AppState>,
    args: UpdateWishlistItemArgs,
) -> Result<WishlistItem, CommandError> {
    info!("Updating wishlist item: {:?}", args);

    args.validate().map_err(CommandError::from)?;

    let mut unit_of_work = state.unit_of_work().await?;

    let input = UpdateWishlistItemInput::try_from(args).map_err(CommandError::from)?;

    let item = UpdateWishlistItemUseCase::execute(&mut unit_of_work, input).await?;

    unit_of_work.commit().await?;

    Ok(item)
}
