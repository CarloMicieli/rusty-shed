use crate::catalog::application::{GetRailwayCompaniesQuery, GetRailwayCompanyByIdQuery};
use crate::catalog::domain::railway_company::{
    RailwayCompany, RailwayCompanyId,
};
use crate::core::infrastructure::error::CommandError;
use crate::state::AppState;
use tauri::State;

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
    state: State<'_, AppState>,
) -> Result<Vec<RailwayCompany>, CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;

    match GetRailwayCompaniesQuery::execute(&mut unit_of_work).await {
        Ok(railway_companies) => {
            // Since this is a 'get' operation, committing is technically optional,
            // but calling it ensures the transaction is closed cleanly.
            unit_of_work
                .commit()
                .await
                .map_err(|err| CommandError::DatabaseError(err.to_string()))?;

            Ok(railway_companies)
        }
        Err(e) => Err(e.into()),
    }
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
    railway_company_id: RailwayCompanyId,
) -> Result<Option<RailwayCompany>, CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;

    match GetRailwayCompanyByIdQuery::execute(&mut unit_of_work, railway_company_id).await {
        Ok(railway_company) => {
            // Since this is a 'get' operation, committing is technically optional,
            // but calling it ensures the transaction is closed cleanly.
            unit_of_work
                .commit()
                .await
                .map_err(|err| CommandError::DatabaseError(err.to_string()))?;

            Ok(railway_company)
        }
        Err(e) => Err(e.into()),
    }
}
