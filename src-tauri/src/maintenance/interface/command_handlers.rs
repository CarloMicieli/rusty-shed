use crate::core::infrastructure::error::CommandError;
use crate::core::infrastructure::runtime_id_provider::RuntimeIdProvider;
use crate::maintenance::application::{AddMaintenanceRecord, GetMaintenanceDashboard};
use crate::maintenance::domain::MaintenanceCard;
use crate::maintenance::domain::MaintenanceType;
use crate::maintenance::interface::AddMaintenanceRecordArgs;
use crate::state::AppState;

/// Retrieve maintenance cards that are due or overdue.
#[tauri::command]
#[specta::specta]
pub async fn get_maintenance_dashboard(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<MaintenanceCard>, CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;

    let cards = GetMaintenanceDashboard::execute(&mut unit_of_work)
        .await
        .map_err(CommandError::from)?;
    unit_of_work.commit().await.map_err(CommandError::from)?;
    Ok(cards)
}

use crate::maintenance::interface::AddMaintenanceArgs;

/// Add a maintenance record and update the card.
#[tauri::command]
#[specta::specta]
pub async fn add_maintenance_record(
    state: tauri::State<'_, AppState>,
    input: AddMaintenanceArgs,
) -> Result<(), CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;

    // Inputs already validated/deserialized by Tauri; pass the maintenance_card_id string
    // through to the use-case and let the use-case parse it.
    let date = input.date_performed;

    let maintenance_type = match input.maintenance_type {
        Some(s) => s.parse::<MaintenanceType>().ok(),
        None => None,
    };

    let use_case_input = AddMaintenanceRecordArgs {
        maintenance_card_id: input.maintenance_card_id.clone(),
        date_performed: date,
        maintenance_type,
        notes: input.notes,
    };

    let id_provider = RuntimeIdProvider::new();

    AddMaintenanceRecord::execute(&mut unit_of_work, id_provider, use_case_input)
        .await
        .map_err(CommandError::from)?;
    unit_of_work.commit().await.map_err(CommandError::from)?;
    Ok(())
}
