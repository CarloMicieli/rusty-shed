use crate::core::infrastructure::error::CommandError;
use crate::state::AppState;
use tauri::State;

use crate::catalog::domain::manufacturer::Manufacturer;
use crate::catalog::domain::manufacturer_id::ManufacturerId;

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
