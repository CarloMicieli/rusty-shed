use crate::catalog::application::create_railway_model::CreateRailwayModelUseCase;
use crate::catalog::application::create_railway_model_input::CreateRailwayModelInput;
use crate::catalog::domain::manufacturer::Manufacturer;
use crate::catalog::domain::manufacturer_id::ManufacturerId;
use crate::catalog::domain::railway_company::RailwayCompany;
use crate::catalog::domain::railway_company_id::RailwayCompanyId;
use crate::catalog::domain::railway_model::RailwayModel;
use crate::catalog::domain::railway_model_id::RailwayModelId;
use crate::core::infrastructure::error::CommandError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::state::AppState;
use log::{error, info};
use tauri::State;

/// Retrieve a manufacturer by its identifier.
///
/// Parses the provided `manufacturer_id` into a domain `ManufacturerId`,
/// acquires a database connection from the application state, and queries the
/// repository for the matching `Manufacturer`.
///
/// # Arguments
///
/// * `state` - Tauri-managed application `AppState` (provides DB pool).
/// * `manufacturer_id` - The manufacturer identifier as a `String`.
///
/// # Returns
///
/// Returns `Ok(Some(Manufacturer))` when a matching manufacturer exists,
/// `Ok(None)` when no matching row is found, or `Err(CommandError)` when the
/// ID cannot be parsed or a database error occurs.
#[tauri::command]
#[specta::specta]
pub async fn get_manufacturer_by_id(
    state: State<'_, AppState>,
    manufacturer_id: String,
) -> Result<Option<Manufacturer>, CommandError> {
    // parse id
    let id = ManufacturerId::try_from(manufacturer_id)
        .map_err(|e| CommandError::Unknown(format!("invalid manufacturer id: {}", e)))?;

    // acquire connection from pool
    let pool = state.db_pool();
    let mut conn = pool
        .acquire()
        .await
        .map_err(|e| CommandError::DatabaseError(format!("db acquire failed: {}", e)))?;

    let result = crate::catalog::infrastructure::repository::get_manufacturer_by_id(&mut conn, &id)
        .await
        .map_err(|e| CommandError::DatabaseError(format!("query failed: {}", e)))?;

    Ok(result)
}

/// Retrieve a railway company by its identifier.
///
/// Parses the provided `railway_company_id` into a domain `RailwayCompanyId`,
/// acquires a database connection from the application state, and queries the
/// repository for the matching `RailwayCompany`.
///
/// # Arguments
///
/// * `state` - Tauri-managed application `AppState` (provides DB pool).
/// * `railway_company_id` - The railway company identifier as a `String`.
///
/// # Returns
///
/// Returns `Ok(Some(RailwayCompany))` when a matching company exists,
/// `Ok(None)` when no matching row is found, or `Err(CommandError)` when the
/// ID cannot be parsed or a database error occurs.
///
/// # Errors
///
/// Parsing errors for the identifier and database errors are mapped to
/// `CommandError` and returned to the caller.
#[tauri::command]
#[specta::specta]
pub async fn get_railway_company_by_id(
    state: State<'_, AppState>,
    railway_company_id: String,
) -> Result<Option<RailwayCompany>, CommandError> {
    let id = RailwayCompanyId::try_from(railway_company_id)
        .map_err(|e| CommandError::Unknown(format!("invalid railway company id: {}", e)))?;

    let pool = state.db_pool();
    let mut conn = pool
        .acquire()
        .await
        .map_err(|e| CommandError::DatabaseError(format!("db acquire failed: {}", e)))?;

    let result =
        crate::catalog::infrastructure::repository::get_railway_company_by_id(&mut conn, &id)
            .await
            .map_err(|e| CommandError::DatabaseError(format!("query failed: {}", e)))?;

    Ok(result)
}

/// Retrieve a railway model by id. Returns the model even if it has no rolling stocks.
#[tauri::command]
#[specta::specta]
pub async fn get_railway_model_by_id(
    state: State<'_, AppState>,
    railway_model_id: String,
) -> Result<Option<RailwayModel>, CommandError> {
    let id = RailwayModelId::try_from(railway_model_id)
        .map_err(|e| CommandError::Unknown(format!("invalid railway model id: {}", e)))?;

    let pool = state.db_pool();
    let mut conn = pool
        .acquire()
        .await
        .map_err(|e| CommandError::DatabaseError(format!("db acquire failed: {}", e)))?;

    let result =
        crate::catalog::infrastructure::repository::get_railway_model_by_id(&mut conn, &id)
            .await
            .map_err(|e| CommandError::DatabaseError(format!("query failed: {}", e)))?;

    Ok(result)
}

/// Retrieve multiple railway models by their identifiers.
#[tauri::command]
#[specta::specta]
pub async fn get_railway_models_by_ids(
    state: State<'_, AppState>,
    railway_model_ids: Vec<String>,
) -> Result<Vec<RailwayModel>, CommandError> {
    let ids: Vec<RailwayModelId> = railway_model_ids
        .into_iter()
        .map(RailwayModelId::try_from)
        .collect::<Result<_, _>>()
        .map_err(|e| CommandError::Unknown(format!("invalid railway model id: {}", e)))?;

    let pool = state.db_pool();
    let mut conn = pool
        .acquire()
        .await
        .map_err(|e| CommandError::DatabaseError(format!("db acquire failed: {}", e)))?;

    let result =
        crate::catalog::infrastructure::repository::get_railway_models_by_ids(&mut conn, &ids)
            .await
            .map_err(|e| CommandError::DatabaseError(format!("query failed: {}", e)))?;

    Ok(result)
}

/// Create a new railway model together with its rolling stocks.
///
/// Performs the creation inside a database transaction using a `SqliteUnitOfWork`.
/// The handler:
/// 1. Opens a `SqliteUnitOfWork` (begins a transaction).
/// 2. Instantiates and executes the `CreateRailwayModelUseCase` with validated input.
/// 3. Commits the transaction on success; on error the transaction is rolled back when the unit of work is dropped.
/// 4. Returns the created railway model identifier.
///
/// # Arguments
///
/// * `state` - Tauri-managed application `AppState` providing the database pool.
/// * `new_railway_model` - The validated input for creating the railway model (`CreateRailwayModelInput`).
///
/// # Returns
///
/// Returns `Result<RailwayModelId, CommandError>`:
/// * `Ok(RailwayModelId)` — the identifier of the newly created railway model on success.
/// * `Err(CommandError)` — when validation fails, a database error occurs, or business logic rejects the operation.
///
/// # Errors
///
/// Errors are mapped to `CommandError` and may represent validation, repository, or unit-of-work failures.
#[tauri::command]
#[specta::specta]
pub async fn create_railway_model(
    state: State<'_, AppState>,
    new_railway_model: CreateRailwayModelInput,
) -> Result<RailwayModelId, CommandError> {
    info!("Creating railway model: {:?}", new_railway_model);

    let mut unit_of_work = SqliteUnitOfWork::new(&state.db_pool())
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    match CreateRailwayModelUseCase::execute(&mut unit_of_work, new_railway_model).await {
        Ok(model_id) => {
            // Commit transaction
            unit_of_work
                .commit()
                .await
                .map_err(|e| CommandError::DatabaseError(e.to_string()))?;
            Ok(model_id)
        }
        Err(e) => {
            error!("Failed to create railway model: {:?}", e);
            Err(e.into())
        }
    }
}
