use crate::catalog::application::{GetManufacturerByIdQuery, GetManufacturersQuery};
use crate::catalog::domain::manufacturer::{Manufacturer, ManufacturerId};
use crate::core::infrastructure::error::CommandError;
use crate::state::AppState;
use tauri::State;

/// Retrieve all manufacturers from the database.
///
/// # Arguments
/// * `state` - Tauri-managed application `AppState` (provides DB pool).
///
/// # Returns
/// - `Ok(Vec<Manufacturer>)` when manufacturers exist, the vector is empty when no manufacturers are found.
/// - `Err(CommandError)` when an error occurs.
#[tauri::command]
#[specta::specta]
pub async fn get_manufacturers(
    state: State<'_, AppState>,
) -> Result<Vec<Manufacturer>, CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;

    match GetManufacturersQuery::execute(&mut unit_of_work).await {
        Ok(manufacturers) => {
            // Since this is a 'get' operation, committing is technically optional,
            // but calling it ensures the transaction is closed cleanly.
            unit_of_work
                .commit()
                .await
                .map_err(|err| CommandError::DatabaseError(err.to_string()))?;

            Ok(manufacturers)
        }
        Err(e) => Err(e.into()),
    }
}

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
    manufacturer_id: ManufacturerId,
) -> Result<Option<Manufacturer>, CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;

    match GetManufacturerByIdQuery::execute(&mut unit_of_work, manufacturer_id).await {
        Ok(manufacturer) => {
            // Since this is a 'get' operation, committing is technically optional,
            // but calling it ensures the transaction is closed cleanly.
            unit_of_work
                .commit()
                .await
                .map_err(|err| CommandError::DatabaseError(err.to_string()))?;

            Ok(manufacturer)
        }
        Err(e) => Err(e.into()),
    }
}
