use crate::catalog::application::{AddRailwayModel, GetRailwayModelViewById};
use crate::catalog::domain::railway_model::RailwayModelId;
use crate::catalog::domain::railway_model::RailwayModelView;
use crate::catalog::interface::CreateRailwayModelArgs;
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
) -> Result<Option<RailwayModelView>, CommandError> {
    info!("Fetching railway model with ID: {}", railway_model_id);

    let mut unit_of_work = state.unit_of_work().await?;

    let railway_model =
        GetRailwayModelViewById::execute(&mut unit_of_work, &railway_model_id).await?;
    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(railway_model)
}

/// Create a new railway model together with its rolling stocks.
///
/// # Arguments
/// * `state` - Tauri-managed application `AppState` providing the database pool.
/// * `args` - The arguments required to create the railway model and its rolling stocks.
///
/// # Returns
/// - `Ok(RailwayModelId)` — the identifier of the newly created railway model on success.
/// - `Err(CommandError)` — when validation fails, a database error occurs, or business logic rejects the operation.
///
/// # Errors
/// Errors are mapped to `CommandError` and may represent validation, repository, or unit-of-work failures.
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
