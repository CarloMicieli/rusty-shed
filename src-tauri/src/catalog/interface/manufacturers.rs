use crate::catalog::application::{GetManufacturerByIdQuery, GetManufacturersQuery};
use crate::catalog::domain::manufacturer::{Manufacturer, ManufacturerId};
use crate::core::infrastructure::error::CommandError;
use crate::state::AppState;
use log::info;

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
    state: tauri::State<'_, AppState>,
) -> Result<Vec<Manufacturer>, CommandError> {
    info!("Fetching all manufacturers from the database.");

    let mut unit_of_work = state.unit_of_work().await?;

    let manufacturers = GetManufacturersQuery::execute(&mut unit_of_work).await?;
    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(manufacturers)
}

/// Retrieve a manufacturer by its identifier.
///
/// Parses the provided `manufacturer_id` into a domain `ManufacturerId`,
/// acquires a database connection from the application state, and queries the
/// repository for the matching `Manufacturer`.
///
/// # Arguments
/// * `state` - Tauri-managed application `AppState` (provides DB pool).
/// * `manufacturer_id` - The manufacturer identifier as a `String`.
///
/// # Returns
/// - `Ok(Some(Manufacturer))` when a matching manufacturer exists,
/// - `Ok(None)` when no matching row is found
/// - `Err(CommandError)` when the ID cannot be parsed or a database error occurs.
///
/// # Errors
/// Parsing errors for the identifier and database errors are mapped to
/// `CommandError` and returned to the caller.
#[tauri::command]
#[specta::specta]
pub async fn get_manufacturer_by_id(
    state: tauri::State<'_, AppState>,
    manufacturer_id: ManufacturerId,
) -> Result<Option<Manufacturer>, CommandError> {
    info!(
        "Fetching manufacturer {} from the database.",
        manufacturer_id
    );

    let mut unit_of_work = state.unit_of_work().await?;

    let manufacturer =
        GetManufacturerByIdQuery::execute(&mut unit_of_work, manufacturer_id).await?;
    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(manufacturer)
}
