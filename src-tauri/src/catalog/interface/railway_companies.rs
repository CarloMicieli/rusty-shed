use crate::catalog::application::{GetRailwayCompaniesQuery, GetRailwayCompanyByIdQuery};
use crate::catalog::domain::railway_company::{RailwayCompany, RailwayCompanyId};
use crate::core::infrastructure::error::CommandError;
use crate::state::AppState;
use log::info;

/// Retrieve all railway companies from the database.
///
/// # Arguments
/// * `state` - Tauri-managed application `AppState` (provides DB pool).
///
/// # Returns
/// - `Ok(Vec<Manufacturer>)` when railway companies exist, the vector is empty when no railway companies are found.
/// - `Err(CommandError)` when an error occurs.
#[tauri::command]
#[specta::specta]
pub async fn get_railway_companies(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<RailwayCompany>, CommandError> {
    info!("Fetching all railway companies from the database.");

    let mut unit_of_work = state.unit_of_work().await?;

    let railway_companies = GetRailwayCompaniesQuery::execute(&mut unit_of_work).await?;
    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(railway_companies)
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
/// - `Ok(Some(RailwayCompany))` when a matching company exists
/// - `Ok(None)` when no matching row is found
/// - `Err(CommandError)` when the ID cannot be parsed or a database error occurs.
///
/// # Errors
/// Parsing errors for the identifier and database errors are mapped to
/// `CommandError` and returned to the caller.
#[tauri::command]
#[specta::specta]
pub async fn get_railway_company_by_id(
    state: tauri::State<'_, AppState>,
    railway_company_id: RailwayCompanyId,
) -> Result<Option<RailwayCompany>, CommandError> {
    info!(
        "Fetching railway company {} from the database.",
        railway_company_id
    );
    let mut unit_of_work = state.unit_of_work().await?;

    let railway_company =
        GetRailwayCompanyByIdQuery::execute(&mut unit_of_work, railway_company_id).await?;
    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(railway_company)
}
