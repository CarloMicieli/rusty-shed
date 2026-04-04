use crate::catalog::application::{
    AddRailwayModel, AddRollingStockToModel, GetCouplerTypes, GetCouplerTypesInput,
    GetRailwayModelTranslations, GetRailwayModelViewById, SearchRailwayModels,
    SetRollingStockCoupler, UpdateRailwayModelClassification, UpdateRailwayModelDeliveryDate,
    UpdateRailwayModelText, UpdateRollingStockCategory, UpdateRollingStockDcc,
    UpdateRollingStockIdentification, UpdateRollingStockRailwayCompany,
    UpdateRollingStockServiceLevel, UpdateRollingStockSpecifications,
    UpdateRollingStockSubcategory, UpsertRailwayModelTranslation, parse_add_rolling_stock_args,
};
use crate::catalog::domain::railway_model::RailwayModelId;
use crate::catalog::domain::railway_model::RailwayModelView;
use crate::catalog::domain::railway_model::railway_model_translation::RailwayModelTranslations;
use crate::catalog::domain::railway_model::{CouplerType, CouplingSocket};
use crate::catalog::interface::{
    AddRollingStockResult, AddRollingStockToModelArgs, CreateRailwayModelArgs,
    SearchRailwayModelsArgs, SetRollingStockCouplerArgs, UpdateRailwayModelClassificationArgs,
    UpdateRailwayModelDeliveryDateArgs, UpdateRailwayModelTextArgs, UpdateRollingStockCategoryArgs,
    UpdateRollingStockDccArgs, UpdateRollingStockIdentificationArgs,
    UpdateRollingStockRailwayCompanyArgs, UpdateRollingStockServiceLevelArgs,
    UpdateRollingStockSpecificationsArgs, UpdateRollingStockSubcategoryArgs,
    UpsertRailwayModelTranslationArgs,
};
use crate::collecting::domain::CollectionUowExt;
use crate::core::domain::Language;
use crate::core::infrastructure::error::CommandError;
use crate::state::AppState;
use garde::Validate;
use log::info;

/// Retrieve a railway model by its identifier.
///
/// Parses the provided `railway_model_id` into a domain `RailwayModelId`,
/// acquires a database connection from the application state, and queries the
/// repository for the matching `RailwayModel`.
///
/// # Arguments
/// * `state` - Tauri-managed application `AppState` (provides DB pool).
/// * `railway_model_id` - The railway model identifier as a `String`.
///
/// # Returns
/// - `Ok(Some(RailwayModel))` when a matching model exists,
/// - `Ok(None)` when no matching row is found
/// - `Err(CommandError)` when the ID cannot be parsed or a database error occurs.
///
/// # Errors
/// Parsing errors for the identifier and database errors are mapped to
/// `CommandError` and returned to the caller.
#[tauri::command]
#[specta::specta]
pub async fn get_railway_model_by_id(
    state: tauri::State<'_, AppState>,
    railway_model_id: RailwayModelId,
    lang: Language,
) -> Result<Option<RailwayModelView>, CommandError> {
    info!("Fetching railway model with ID: {}", railway_model_id);

    let lang_str = match lang {
        Language::English => "en",
        Language::Italian => "it",
    };
    let mut unit_of_work = state.unit_of_work().await?;

    let railway_model =
        GetRailwayModelViewById::execute(&mut unit_of_work, &railway_model_id, lang_str).await?;
    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(railway_model)
}

/// Create a new railway model along with its associated rolling stocks.
///
/// # Arguments
/// * `state` - Tauri-managed application `AppState` providing the database pool.
/// * `args` - The arguments required to create the railway model and its rolling stocks.
///
/// # Returns
/// - `Ok(RailwayModelId)` — the identifier of the newly created railway model on success.
/// - `Err(CommandError)` — when validation fails, a database error occurs, or business logic rejects the operation.
#[tauri::command]
#[specta::specta]
pub async fn create_railway_model(
    state: tauri::State<'_, AppState>,
    args: CreateRailwayModelArgs,
) -> Result<RailwayModelId, CommandError> {
    info!("Creating railway model: {:?}", args);

    let mut unit_of_work = state.unit_of_work().await?;

    let railway_model_input = args.try_into()?;
    let railway_model_id = AddRailwayModel::execute(&mut unit_of_work, railway_model_input).await?;
    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(railway_model_id)
}

/// Update a single free-text field (description or details) of a railway model.
///
/// # Arguments
/// * `state` - Tauri-managed application `AppState` providing the database pool.
/// * `args` - The target model, field, and new value.
///
/// # Returns
/// - `Ok(())` on success.
/// - `Err(CommandError::NotFound)` when the railway model does not exist.
/// - `Err(CommandError::ValidationError)` when the value is invalid (e.g., empty description).
/// - `Err(CommandError::DatabaseError)` on persistence failure.
#[tauri::command]
#[specta::specta]
pub async fn update_railway_model_text(
    state: tauri::State<'_, AppState>,
    args: UpdateRailwayModelTextArgs,
) -> Result<(), CommandError> {
    info!(
        "Updating railway model text field {:?} for {}",
        args.field, args.railway_model_id
    );

    let mut unit_of_work = state.unit_of_work().await?;
    UpdateRailwayModelText::execute(&mut unit_of_work, args.into()).await?;
    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(())
}

/// Update the identification fields (series_code, road_number, livery, depot) of a single
/// rolling stock unit within a railway model.
///
/// # Arguments
/// * `state` - Tauri-managed application `AppState` providing the database pool.
/// * `args` - The target model, rolling stock, and new identification values.
///
/// # Returns
/// - `Ok(())` on success.
/// - `Err(CommandError::NotFound)` when the railway model or rolling stock does not exist.
/// - `Err(CommandError::ValidationError)` when `series_code` is empty.
/// - `Err(CommandError::DatabaseError)` on persistence failure.
#[tauri::command]
#[specta::specta]
pub async fn update_rolling_stock_identification(
    state: tauri::State<'_, AppState>,
    args: UpdateRollingStockIdentificationArgs,
) -> Result<(), CommandError> {
    info!(
        "Updating rolling stock identification for {} / {}",
        args.railway_model_id, args.rolling_stock_id
    );

    let mut unit_of_work = state.unit_of_work().await?;
    UpdateRollingStockIdentification::execute(&mut unit_of_work, args.into()).await?;
    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(())
}

/// Update the constrained classification fields (scale and/or epoch) of a railway model.
///
/// # Arguments
/// * `state` - Tauri-managed application `AppState` providing the database pool.
/// * `args` - The target model with optional scale and epoch values.
///
/// # Returns
/// - `Ok(())` on success.
/// - `Err(CommandError::NotFound)` when the railway model does not exist.
/// - `Err(CommandError::ValidationError)` when neither scale nor epoch is provided.
/// - `Err(CommandError::DatabaseError)` on persistence failure.
#[tauri::command]
#[specta::specta]
pub async fn update_railway_model_classification(
    state: tauri::State<'_, AppState>,
    args: UpdateRailwayModelClassificationArgs,
) -> Result<(), CommandError> {
    info!(
        "Updating railway model classification for {}",
        args.railway_model_id
    );

    let mut unit_of_work = state.unit_of_work().await?;
    UpdateRailwayModelClassification::execute(&mut unit_of_work, args.into()).await?;
    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(())
}

/// Update the delivery date of a railway model.
///
/// # Arguments
/// * `state` - Tauri-managed application `AppState` providing the database pool.
/// * `args` - The target model and the new delivery date string (or `None`/`""` to clear).
///
/// # Returns
/// - `Ok(())` on success.
/// - `Err(CommandError::NotFound)` when the railway model does not exist.
/// - `Err(CommandError::ValidationError)` when the delivery date string cannot be parsed.
/// - `Err(CommandError::DatabaseError)` on persistence failure.
#[tauri::command]
#[specta::specta]
pub async fn update_railway_model_delivery_date(
    state: tauri::State<'_, AppState>,
    args: UpdateRailwayModelDeliveryDateArgs,
) -> Result<(), CommandError> {
    info!(
        "Updating delivery date for railway model {}",
        args.railway_model_id
    );

    let mut unit_of_work = state.unit_of_work().await?;
    UpdateRailwayModelDeliveryDate::execute(&mut unit_of_work, args.try_into()?).await?;
    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(())
}

/// Update the railway company of a single rolling stock unit.
///
/// # Arguments
/// * `state` - Tauri-managed application `AppState` providing the database pool.
/// * `args` - The target model, rolling stock, and new railway company id.
///
/// # Returns
/// - `Ok(())` on success.
/// - `Err(CommandError::NotFound)` when the railway company, model, or rolling stock does not exist.
/// - `Err(CommandError::DatabaseError)` on persistence failure.
#[tauri::command]
#[specta::specta]
pub async fn update_rolling_stock_railway_company(
    state: tauri::State<'_, AppState>,
    args: UpdateRollingStockRailwayCompanyArgs,
) -> Result<(), CommandError> {
    info!(
        "Updating railway company for rolling stock {} / {}",
        args.railway_model_id, args.rolling_stock_id
    );

    let mut unit_of_work = state.unit_of_work().await?;
    UpdateRollingStockRailwayCompany::execute(&mut unit_of_work, args.into()).await?;
    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(())
}

/// Change the category (variant) of a single rolling stock unit.
///
/// # Arguments
/// * `state` - Tauri-managed application `AppState` providing the database pool.
/// * `args` - The target model, rolling stock, and new category.
///
/// # Returns
/// - `Ok(())` on success.
/// - `Err(CommandError::NotFound)` when the railway model or rolling stock does not exist.
/// - `Err(CommandError::DatabaseError)` on persistence failure.
#[tauri::command]
#[specta::specta]
pub async fn update_rolling_stock_category(
    state: tauri::State<'_, AppState>,
    args: UpdateRollingStockCategoryArgs,
) -> Result<(), CommandError> {
    info!(
        "Updating category for rolling stock {} / {}",
        args.railway_model_id, args.rolling_stock_id
    );

    let mut unit_of_work = state.unit_of_work().await?;
    UpdateRollingStockCategory::execute(&mut unit_of_work, args.into()).await?;
    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(())
}

/// Update only the control type, DCC interface, and length of a single rolling stock unit.
///
/// Unlike `update_rolling_stock_specifications`, this command only touches these three fields
/// and leaves all other technical specification fields (flywheel, body shell, etc.) unchanged.
///
/// # Arguments
/// * `state` - Tauri-managed application `AppState` providing the database pool.
/// * `args` - The target model, rolling stock, and new values.
///
/// # Returns
/// - `Ok(())` on success.
/// - `Err(CommandError::NotFound)` when the railway model or rolling stock does not exist.
/// - `Err(CommandError::DatabaseError)` on persistence failure.
#[tauri::command]
#[specta::specta]
pub async fn update_rolling_stock_dcc(
    state: tauri::State<'_, AppState>,
    args: UpdateRollingStockDccArgs,
) -> Result<(), CommandError> {
    log::info!(
        "Updating DCC/length for rolling stock {} / {}",
        args.railway_model_id,
        args.rolling_stock_id
    );

    let mut unit_of_work = state.unit_of_work().await?;
    UpdateRollingStockDcc::execute(&mut unit_of_work, args.into()).await?;
    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(())
}

/// Update the full technical specifications of a single rolling stock unit (drawer save).
///
/// # Arguments
/// * `state` - Tauri-managed application `AppState` providing the database pool.
/// * `args` - All four specification sections: identification, technical, control, coupling.
///
/// # Returns
/// - `Ok(())` on success.
/// - `Err(CommandError::NotFound)` when the railway model or rolling stock does not exist.
/// - `Err(CommandError::ValidationError)` when `series_code` is empty or enum values are invalid.
/// - `Err(CommandError::DatabaseError)` on persistence failure.
#[tauri::command]
#[specta::specta]
pub async fn update_rolling_stock_specifications(
    state: tauri::State<'_, AppState>,
    args: UpdateRollingStockSpecificationsArgs,
) -> Result<(), CommandError> {
    info!(
        "Updating rolling stock specifications for {} / {}",
        args.railway_model_id, args.rolling_stock_id
    );

    let mut unit_of_work = state.unit_of_work().await?;
    UpdateRollingStockSpecifications::execute(&mut unit_of_work, args.try_into()?).await?;
    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(())
}

/// Retrieve all stored translations for a railway model (used to pre-populate the edit form).
#[tauri::command]
#[specta::specta]
pub async fn get_railway_model_translations(
    state: tauri::State<'_, AppState>,
    railway_model_id: RailwayModelId,
) -> Result<Option<RailwayModelTranslations>, CommandError> {
    info!(
        "Fetching translations for railway model {}",
        railway_model_id
    );

    let mut unit_of_work = state.unit_of_work().await?;
    let translations =
        GetRailwayModelTranslations::execute(&mut unit_of_work, &railway_model_id).await?;
    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(translations)
}

/// Create or replace a translation for one language on a railway model.
#[tauri::command]
#[specta::specta]
pub async fn upsert_railway_model_translation(
    state: tauri::State<'_, AppState>,
    args: UpsertRailwayModelTranslationArgs,
) -> Result<(), CommandError> {
    info!(
        "Upserting {} translation for railway model {}",
        args.lang, args.railway_model_id
    );

    args.validate().map_err(CommandError::from)?;
    let mut unit_of_work = state.unit_of_work().await?;
    UpsertRailwayModelTranslation::execute(&mut unit_of_work, args.into()).await?;
    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(())
}

/// Search railway models using FTS5 full-text search across all language translations.
#[tauri::command]
#[specta::specta]
pub async fn search_railway_models(
    state: tauri::State<'_, AppState>,
    args: SearchRailwayModelsArgs,
) -> Result<Vec<RailwayModelId>, CommandError> {
    info!("Searching railway models with query: {}", args.query);

    args.validate().map_err(CommandError::from)?;
    let mut unit_of_work = state.unit_of_work().await?;
    let ids = SearchRailwayModels::execute(&mut unit_of_work, args.into()).await?;
    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(ids)
}

/// Add a new rolling stock variant to an existing Railway Model.
///
/// # Arguments
/// * `state` - Tauri-managed application `AppState` providing the database pool.
/// * `args` - The rolling stock identification data and category.
///
/// # Returns
/// - `Ok(RollingStockId)` — the identifier of the newly created rolling stock on success.
/// - `Err(CommandError)` — when validation fails, the model is not found, or a database error occurs.
#[tauri::command]
#[specta::specta]
pub async fn add_rolling_stock_to_model(
    state: tauri::State<'_, AppState>,
    args: AddRollingStockToModelArgs,
) -> Result<AddRollingStockResult, CommandError> {
    info!(
        "Adding rolling stock to model {} (category: {})",
        args.railway_model_id, args.category
    );

    args.validate().map_err(CommandError::from)?;

    let input = parse_add_rolling_stock_args(
        args.railway_model_id,
        args.railway_company_id,
        args.category,
        args.series_code,
        args.road_number,
        args.livery,
        args.depot,
        args.control,
        args.dcc_interface,
        args.coupling_socket,
        args.close_couplers,
        args.sub_type,
        args.friendly_name,
        args.prototype_id,
    )?;

    let railway_model_id = input.railway_model_id.clone();
    let mut unit_of_work = state.unit_of_work().await?;
    let rs_id = AddRollingStockToModel::execute(&mut unit_of_work, input).await?;

    let owned_ids = unit_of_work
        .collections_repository()
        .add_owned_rolling_stock_for_collection_items(&railway_model_id, &rs_id)
        .await
        .map_err(CommandError::from)?;

    unit_of_work.commit().await.map_err(CommandError::from)?;

    let owned_rolling_stock_id = owned_ids
        .into_iter()
        .next()
        .ok_or_else(|| CommandError::NotFound("owned rolling stock".to_string()))?;

    Ok(AddRollingStockResult {
        rolling_stock_id: rs_id,
        owned_rolling_stock_id,
    })
}

/// Update the subcategory (type field) of a single rolling stock unit.
///
/// # Arguments
/// * `state` - Tauri-managed application `AppState` providing the database pool.
/// * `args` - The target model, rolling stock, and new subcategory string.
///
/// # Returns
/// - `Ok(())` on success.
/// - `Err(CommandError::NotFound)` when the railway model or rolling stock does not exist.
/// - `Err(CommandError::ValidationError)` when the subcategory is invalid for the current category.
/// - `Err(CommandError::DatabaseError)` on persistence failure.
#[tauri::command]
#[specta::specta]
pub async fn update_rolling_stock_subcategory(
    state: tauri::State<'_, AppState>,
    args: UpdateRollingStockSubcategoryArgs,
) -> Result<(), CommandError> {
    info!(
        "Updating subcategory for rolling stock {} / {}",
        args.railway_model_id, args.rolling_stock_id
    );

    let mut unit_of_work = state.unit_of_work().await?;
    UpdateRollingStockSubcategory::execute(&mut unit_of_work, args.into()).await?;
    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(())
}

/// Update the service level of a single rolling stock unit.
///
/// # Arguments
/// * `state` - Tauri-managed application `AppState` providing the database pool.
/// * `args` - The target model, rolling stock, and new service level (or None to clear).
///
/// # Returns
/// - `Ok(())` on success.
/// - `Err(CommandError::NotFound)` when the railway model or rolling stock does not exist.
/// - `Err(CommandError::DatabaseError)` on persistence failure.
#[tauri::command]
#[specta::specta]
pub async fn update_rolling_stock_service_level(
    state: tauri::State<'_, AppState>,
    args: UpdateRollingStockServiceLevelArgs,
) -> Result<(), CommandError> {
    info!(
        "Updating service level for rolling stock {} / {}",
        args.railway_model_id, args.rolling_stock_id
    );

    let mut unit_of_work = state.unit_of_work().await?;
    UpdateRollingStockServiceLevel::execute(&mut unit_of_work, args.into()).await?;
    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(())
}

/// Return the coupler type catalogue, optionally filtered to a specific coupling socket.
///
/// When `socket` is provided (e.g. `"NEM_362"`), only couplers compatible with that
/// socket are returned. When omitted, the full catalogue is returned.
///
/// # Arguments
/// * `state` - Tauri-managed application `AppState` providing the database pool.
/// * `socket` - Optional coupling socket filter string.
///
/// # Returns
/// - `Ok(Vec<CouplerType>)` on success.
/// - `Err(CommandError::DatabaseError)` on persistence failure.
#[tauri::command]
#[specta::specta]
pub async fn get_coupler_types(
    state: tauri::State<'_, AppState>,
    socket: Option<String>,
) -> Result<Vec<CouplerType>, CommandError> {
    let socket_filter = socket
        .as_deref()
        .map(|s| s.parse::<CouplingSocket>())
        .transpose()
        .map_err(|_| CommandError::validation_field("socket", "Invalid coupling socket value"))?;

    let mut unit_of_work = state.unit_of_work().await?;
    let result = GetCouplerTypes::execute(
        &mut unit_of_work,
        GetCouplerTypesInput {
            socket: socket_filter,
        },
    )
    .await?;
    unit_of_work.commit().await.map_err(CommandError::from)?;
    Ok(result)
}

/// Set (or clear) the installed coupler on an owned rolling stock.
///
/// After updating `current_coupler_id`, a `CouplerChange` maintenance event is
/// automatically recorded on the rolling stock's maintenance card if one exists.
///
/// # Arguments
/// * `state` - Tauri-managed application `AppState` providing the database pool.
/// * `args` - The owned rolling stock id and the coupler type id to install (or `null`).
///
/// # Returns
/// - `Ok(())` on success.
/// - `Err(CommandError::DatabaseError)` on persistence failure.
#[tauri::command]
#[specta::specta]
pub async fn set_rolling_stock_coupler(
    state: tauri::State<'_, AppState>,
    args: SetRollingStockCouplerArgs,
) -> Result<(), CommandError> {
    info!(
        "Setting coupler {:?} on owned rolling stock {}",
        args.coupler_type_id, args.owned_rolling_stock_id
    );

    let mut unit_of_work = state.unit_of_work().await?;
    SetRollingStockCoupler::execute(&mut unit_of_work, args.into()).await?;
    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(())
}
