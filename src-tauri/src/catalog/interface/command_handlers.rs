use crate::catalog::application::{
    AddRailwayModel, GetRailwayModelTranslations, GetRailwayModelViewById, SearchRailwayModels,
    UpdateRailwayModelClassification, UpdateRailwayModelText, UpdateRollingStockIdentification,
    UpdateRollingStockRailwayCompany, UpdateRollingStockSpecifications,
    UpsertRailwayModelTranslation,
};
use crate::catalog::domain::railway_model::railway_model_translation::RailwayModelTranslations;
use crate::catalog::domain::railway_model::RailwayModelId;
use crate::catalog::domain::railway_model::RailwayModelView;
use crate::catalog::interface::{
    CreateRailwayModelArgs, SearchRailwayModelsArgs, UpdateRailwayModelClassificationArgs,
    UpdateRailwayModelTextArgs, UpdateRollingStockIdentificationArgs,
    UpdateRollingStockRailwayCompanyArgs, UpdateRollingStockSpecificationsArgs,
    UpsertRailwayModelTranslationArgs,
};
use crate::core::infrastructure::error::CommandError;
use crate::state::AppState;
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
    lang: String,
) -> Result<Option<RailwayModelView>, CommandError> {
    info!("Fetching railway model with ID: {}", railway_model_id);

    let lang = if lang == "it" { "it" } else { "en" };
    let mut unit_of_work = state.unit_of_work().await?;

    let railway_model =
        GetRailwayModelViewById::execute(&mut unit_of_work, &railway_model_id, lang).await?;
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
    info!("Fetching translations for railway model {}", railway_model_id);

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

    args.validate()?;
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

    args.validate()?;
    let mut unit_of_work = state.unit_of_work().await?;
    let ids = SearchRailwayModels::execute(&mut unit_of_work, args.into()).await?;
    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(ids)
}
