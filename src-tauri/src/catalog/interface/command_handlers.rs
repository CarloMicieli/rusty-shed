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

/// Create a new railway model with rolling stocks.
///
/// This command follows the Unit of Work + Use Case pattern:
/// 1. Creates a SqliteUnitOfWork (transaction)
/// 2. Instantiates the CreateRailwayModelUseCase
/// 3. Executes the use case with validated input
/// 4. Commits the transaction on success
/// 5. Returns the created railway model ID
///
/// # Arguments
///
/// * `state` - Tauri-managed application `AppState` (provides DB pool).
/// * `input` - The railway model creation input data.
///
/// # Returns
///
/// Returns `Ok(String)` with the newly created railway model ID on success,
/// or `Err(CommandError)` if validation, database, or business logic fails.
#[tauri::command]
#[specta::specta]
pub async fn create_railway_model(
    state: State<'_, AppState>,
    input: CreateRailwayModelInput,
) -> Result<String, CommandError> {
    // Create Unit of Work (transaction)
    let mut uow = SqliteUnitOfWork::new(&state.db_pool())
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    // Execute use case
    match CreateRailwayModelUseCase::execute(&mut uow, input).await {
        Ok(model_id) => {
            // Commit transaction
            uow.commit()
                .await
                .map_err(|e| CommandError::DatabaseError(e.to_string()))?;
            Ok(model_id)
        }
        Err(e) => Err(CommandError::Unknown(e)),
    }
}
