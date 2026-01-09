use crate::core::infrastructure::error::CommandError;
use crate::maintenance::application::add_maintenance_record::AddMaintenanceRecordInput;
use crate::maintenance::application::{
    AddMaintenanceRecordUseCase, GetMaintenanceDashboardUseCase,
};
use crate::maintenance::domain::maintenance_card::MaintenanceCard;
use crate::maintenance::domain::maintenance_type::MaintenanceType;
use crate::state::AppState;
use chrono::NaiveDate;
use uuid::Uuid;

/// Retrieve maintenance cards that are due or overdue.
#[tauri::command]
#[specta::specta]
pub async fn get_maintenance_dashboard(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<MaintenanceCard>, CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;

    let cards = GetMaintenanceDashboardUseCase::execute(&mut unit_of_work)
        .await
        .map_err(CommandError::from)?;
    unit_of_work.commit().await.map_err(CommandError::from)?;
    Ok(cards)
}

/// Input DTO for adding a maintenance record over IPC.
#[derive(Debug, serde::Deserialize, specta::Type)]
pub struct AddMaintenanceInput {
    pub id: String,
    pub maintenance_card_id: String,
    pub date_performed: String, // YYYY-MM-DD
    pub maintenance_type: Option<String>,
    pub notes: Option<String>,
}

/// Add a maintenance record and update the card.
#[tauri::command]
#[specta::specta]
pub async fn add_maintenance_record(
    state: tauri::State<'_, AppState>,
    input: AddMaintenanceInput,
) -> Result<(), CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;

    // Parse inputs
    let id = Uuid::parse_str(&input.id)
        .map_err(|e| CommandError::validation_field("id", e.to_string()))?;
    let card_id = Uuid::parse_str(&input.maintenance_card_id)
        .map_err(|e| CommandError::validation_field("maintenance_card_id", e.to_string()))?;
    let date = NaiveDate::parse_from_str(&input.date_performed, "%Y-%m-%d")
        .map_err(|e| CommandError::validation_field("date_performed", e.to_string()))?;

    let maintenance_type = match input.maintenance_type {
        Some(s) => s.parse::<MaintenanceType>().ok(),
        None => None,
    };

    let use_case_input = AddMaintenanceRecordInput {
        id,
        maintenance_card_id: card_id,
        date_performed: date,
        maintenance_type,
        notes: input.notes,
    };

    AddMaintenanceRecordUseCase::execute(&mut unit_of_work, use_case_input)
        .await
        .map_err(CommandError::from)?;
    unit_of_work.commit().await.map_err(CommandError::from)?;
    Ok(())
}
