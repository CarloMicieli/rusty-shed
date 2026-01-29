use crate::collecting::domain::OwnedRollingStockId;
use crate::core::infrastructure::error::CommandError;
use crate::core::infrastructure::runtime_id_provider::RuntimeIdProvider;
use crate::maintenance::application::AddMaintenanceCard;
use crate::maintenance::application::AddMaintenanceEvent;
use crate::maintenance::application::add_maintenance_card::AddMaintenanceCardInput;
use crate::maintenance::application::add_maintenance_event::AddMaintenanceEventInput;
use crate::maintenance::domain::MaintenanceCardId;
use crate::maintenance::domain::MaintenanceType;
use crate::maintenance::domain::MaintenanceUowExt;
use crate::maintenance::interface::{
    AddMaintenanceArgs, AddMaintenanceEventArgs, MaintenanceCardView,
};
use crate::state::AppState;
use std::convert::TryInto;

/// Command handler to retrieve maintenance cards that are due or overdue.
///
/// # Arguments
/// - `state`: The application state.
///
/// # Returns
/// A vector of `MaintenanceCardView` representing the due maintenance cards.
#[tauri::command]
#[specta::specta]
pub async fn get_maintenance_dashboard(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<MaintenanceCardView>, CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;

    let mut repo = unit_of_work.maintenance_repository();

    let views = repo
        .list_due_card_views()
        .await
        .map_err(CommandError::from)?;

    // Drop the repo borrow before committing the unit of work.
    drop(repo);

    unit_of_work.commit().await.map_err(CommandError::from)?;
    Ok(views)
}

/// Command handler to create a new maintenance card for the given owned rolling stock.
///
/// # Arguments
/// - `state`: The application state.
/// - `owned_rolling_stock_id`: The ID of the owned rolling stock.
///
/// # Returns
/// The ID of the newly created maintenance card.
#[tauri::command]
#[specta::specta]
pub async fn add_maintenance_card(
    state: tauri::State<'_, AppState>,
    owned_rolling_stock_id: OwnedRollingStockId,
) -> Result<MaintenanceCardId, CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;

    let input = AddMaintenanceCardInput {
        owned_rolling_stock_id,
    };

    let id_provider = RuntimeIdProvider::new();

    let id = AddMaintenanceCard::execute(&mut unit_of_work, id_provider, input)
        .await
        .map_err(CommandError::from)?;

    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(id)
}

/// Command handler to add a maintenance event and update the card.
///
/// # Arguments
/// - `state`: The application state.
/// - `input`: The arguments required to add a maintenance event.
///
/// # Returns
/// nothing on success.
#[tauri::command]
#[specta::specta]
pub async fn add_maintenance_event(
    state: tauri::State<'_, AppState>,
    input: AddMaintenanceArgs,
) -> Result<(), CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;

    // Inputs already validated/deserialized by Tauri; create an Args instance
    // and convert it into the application input type.
    let date = input.date_performed;

    let maintenance_type = match input.maintenance_type {
        Some(s) => s.parse::<MaintenanceType>().ok(),
        None => None,
    };

    let args = AddMaintenanceEventArgs {
        maintenance_card_id: input.maintenance_card_id.clone(),
        date_performed: date,
        maintenance_type,
        notes: input.notes,
    };

    let app_input: AddMaintenanceEventInput = args.try_into()?;

    let id_provider = RuntimeIdProvider::new();

    AddMaintenanceEvent::execute(&mut unit_of_work, id_provider, app_input)
        .await
        .map_err(CommandError::from)?;
    unit_of_work.commit().await.map_err(CommandError::from)?;
    Ok(())
}
