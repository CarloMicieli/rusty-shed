use crate::catalog::application::{GetRailwayCompanies, GetRailwayCompanyById};
use crate::catalog::domain::railway_company::{RailwayCompany, RailwayCompanyId};
use crate::core::infrastructure::error::CommandError;
use crate::state::AppState;
use tracing::info;

/// Retrieve all railway companies from the database.
pub async fn get_railway_companies_inner(
    state: &AppState,
) -> Result<Vec<RailwayCompany>, CommandError> {
    info!("Fetching all railway companies from the database.");
    let mut uow = state.unit_of_work().await?;
    let railway_companies = GetRailwayCompanies::execute(&mut uow).await?;
    uow.commit().await?;
    Ok(railway_companies)
}

/// Tauri command to retrieve all railway companies.
#[tauri::command]
#[specta::specta]
pub async fn get_railway_companies(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<RailwayCompany>, CommandError> {
    get_railway_companies_inner(&state).await
}

/// Retrieve a railway company by its identifier.
pub async fn get_railway_company_by_id_inner(
    state: &AppState,
    railway_company_id: RailwayCompanyId,
) -> Result<Option<RailwayCompany>, CommandError> {
    info!(
        "Fetching railway company {} from the database.",
        railway_company_id
    );
    let mut uow = state.unit_of_work().await?;
    let railway_company = GetRailwayCompanyById::execute(&mut uow, railway_company_id).await?;
    uow.commit().await?;
    Ok(railway_company)
}

/// Tauri command to retrieve a railway company by its identifier.
#[tauri::command]
#[specta::specta]
pub async fn get_railway_company_by_id(
    state: tauri::State<'_, AppState>,
    railway_company_id: RailwayCompanyId,
) -> Result<Option<RailwayCompany>, CommandError> {
    get_railway_company_by_id_inner(&state, railway_company_id).await
}
