//! Tauri command handlers for the `maintenance` module.
//!
//! These functions follow the project's Unit of Work + Use Case pattern: they
//! create a `SqliteUnitOfWork`, instantiate a use-case, execute it, commit the
//! transaction, and map errors to `CommandError` for transmission over IPC.

use crate::core::infrastructure::error::CommandError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
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
    let mut uow = SqliteUnitOfWork::new(&state.db_pool())
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    let use_case = GetMaintenanceDashboardUseCase::new();

    let cards = use_case
        .execute(&mut uow)
        .await
        .map_err(CommandError::from)?;
    uow.commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;
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
    let mut uow = SqliteUnitOfWork::new(&state.db_pool())
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    // Parse inputs
    let id = Uuid::parse_str(&input.id)
        .map_err(|e| CommandError::validation_field("id", e.to_string()))?;
    let card_id = Uuid::parse_str(&input.maintenance_card_id)
        .map_err(|e| CommandError::validation_field("maintenance_card_id", e.to_string()))?;
    let date = NaiveDate::parse_from_str(&input.date_performed, "%Y-%m-%d")
        .map_err(|e| CommandError::validation_field("date_performed", e.to_string()))?;

    let use_case = AddMaintenanceRecordUseCase::new();

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

    use_case
        .execute(&mut uow, use_case_input)
        .await
        .map_err(CommandError::from)?;
    uow.commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;
    Ok(())
}
