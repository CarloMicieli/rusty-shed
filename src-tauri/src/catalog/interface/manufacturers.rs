use crate::catalog::application::{GetManufacturerById, GetManufacturers};
use crate::catalog::domain::manufacturer::{Manufacturer, ManufacturerId};
use crate::core::infrastructure::error::CommandError;
use crate::state::AppState;
use log::info;

// ---------------------------------------------------------------------------
// Inner (testable) implementations – take &AppState directly
// ---------------------------------------------------------------------------

/// Retrieve all manufacturers from the database.
pub async fn get_manufacturers_inner(state: &AppState) -> Result<Vec<Manufacturer>, CommandError> {
    info!("Fetching all manufacturers from the database.");
    let mut uow = state.unit_of_work().await?;
    let manufacturers = GetManufacturers::execute(&mut uow).await?;
    uow.commit().await?;
    Ok(manufacturers)
}

/// Tauri command to retrieve all manufacturers.
#[tauri::command]
#[specta::specta]
pub async fn get_manufacturers(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<Manufacturer>, CommandError> {
    get_manufacturers_inner(&state).await
}

/// Retrieve a manufacturer by its identifier.
pub async fn get_manufacturer_by_id_inner(
    state: &AppState,
    manufacturer_id: ManufacturerId,
) -> Result<Option<Manufacturer>, CommandError> {
    info!(
        "Fetching manufacturer {} from the database.",
        manufacturer_id
    );
    let mut uow = state.unit_of_work().await?;
    let manufacturer = GetManufacturerById::execute(&mut uow, manufacturer_id).await?;
    uow.commit().await?;
    Ok(manufacturer)
}

/// Tauri command to retrieve a manufacturer by its identifier.
#[tauri::command]
#[specta::specta]
pub async fn get_manufacturer_by_id(
    state: tauri::State<'_, AppState>,
    manufacturer_id: ManufacturerId,
) -> Result<Option<Manufacturer>, CommandError> {
    get_manufacturer_by_id_inner(&state, manufacturer_id).await
}
