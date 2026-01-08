use crate::catalog::application::GetRailwayModelByIdQuery;
use crate::catalog::application::railway_model_use_case::CreateRailwayModelUseCase;
use crate::catalog::application::railway_model_use_case_input::CreateRailwayModelInput;
use crate::catalog::domain::railway_model::RailwayModel;
use crate::catalog::domain::railway_model::RailwayModelId;
use crate::core::infrastructure::error::CommandError;
use crate::state::AppState;
use log::{error, info};

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
/// * `Ok(Some(RailwayModel))` when a matching model exists,
/// * `Ok(None)` when no matching row is found
/// * `Err(CommandError)` when the ID cannot be parsed or a database error occurs.
///
/// # Errors
/// Parsing errors for the identifier and database errors are mapped to
/// `CommandError` and returned to the caller.
#[tauri::command]
#[specta::specta]
pub async fn get_railway_model_by_id(
    state: tauri::State<'_, AppState>,
    railway_model_id: RailwayModelId,
) -> Result<Option<RailwayModel>, CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;

    match GetRailwayModelByIdQuery::execute(&mut unit_of_work, railway_model_id).await {
        Ok(railway_model) => {
            // Since this is a 'get' operation, committing is technically optional,
            // but calling it ensures the transaction is closed cleanly.
            unit_of_work
                .commit()
                .await
                .map_err(|err| CommandError::DatabaseError(err.to_string()))?;

            Ok(railway_model)
        }
        Err(e) => Err(e.into()),
    }
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
    state: tauri::State<'_, AppState>,
    new_railway_model: CreateRailwayModelInput,
) -> Result<RailwayModelId, CommandError> {
    info!("Creating railway model: {:?}", new_railway_model);

    let mut unit_of_work = state.unit_of_work().await?;

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
