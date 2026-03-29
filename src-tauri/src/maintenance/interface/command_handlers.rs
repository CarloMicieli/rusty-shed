use crate::collecting::domain::OwnedRollingStockId;
use crate::core::domain::validation::ValidationError;
use crate::core::infrastructure::error::CommandError;
use crate::core::infrastructure::runtime_id_provider::RuntimeIdProvider;
use crate::maintenance::application::AddMaintenanceCard;
use crate::maintenance::application::AddMaintenanceEvent;
use crate::maintenance::application::DeleteMaintenanceEvent;
use crate::maintenance::application::add_maintenance_card::AddMaintenanceCardInput;
use crate::maintenance::application::add_maintenance_event::AddMaintenanceEventInput;
use crate::maintenance::domain::MaintenanceCardId;
use crate::maintenance::domain::MaintenanceEventId;
use crate::maintenance::domain::MaintenanceType;
use crate::maintenance::domain::MaintenanceUowExt;
use crate::maintenance::interface::{
    AddMaintenanceArgs, AddMaintenanceEventArgs, MaintenanceCardView,
};
use crate::state::AppState;
use garde::{Report, Validate};
use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::TryInto;

fn map_garde_report_to_command_error(report: Report) -> CommandError {
    let mut fields: HashMap<String, Vec<ValidationError>> = HashMap::new();

    for (path, error) in report.into_inner() {
        fields
            .entry(path.to_string())
            .or_default()
            .push(ValidationError {
                code: Cow::Borrowed("invalid"),
                message: Some(Cow::Owned(error.to_string())),
                params: HashMap::new(),
            });
    }

    CommandError::ValidationError(fields)
}

/// Command handler to retrieve a single maintenance card by its ID.
///
/// # Arguments
/// - `state`: The application state.
/// - `card_id`: The ID of the maintenance card to retrieve.
///
/// # Returns
/// The `MaintenanceCardView` if found, or `None`.
#[tauri::command]
#[specta::specta]
pub async fn get_maintenance_card(
    state: tauri::State<'_, AppState>,
    card_id: MaintenanceCardId,
) -> Result<Option<MaintenanceCardView>, CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;
    let mut repo = unit_of_work.maintenance_repository();
    let view = repo
        .find_view_by_id(&card_id)
        .await
        .map_err(CommandError::from)?;
    drop(repo);
    unit_of_work.commit().await.map_err(CommandError::from)?;
    Ok(view)
}

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

/// Command handler to delete a single maintenance event.
#[tauri::command]
#[specta::specta]
pub async fn delete_maintenance_event(
    state: tauri::State<'_, AppState>,
    event_id: uuid::Uuid,
) -> Result<(), CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;

    let id = MaintenanceEventId::from_uuid(&event_id);

    DeleteMaintenanceEvent::execute(&mut unit_of_work, id)
        .await
        .map_err(CommandError::from)?;

    unit_of_work.commit().await.map_err(CommandError::from)?;
    Ok(())
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

    args.validate().map_err(map_garde_report_to_command_error)?;

    let app_input: AddMaintenanceEventInput = args.try_into()?;

    let id_provider = RuntimeIdProvider::new();

    AddMaintenanceEvent::execute(&mut unit_of_work, id_provider, app_input)
        .await
        .map_err(CommandError::from)?;
    unit_of_work.commit().await.map_err(CommandError::from)?;
    Ok(())
}
