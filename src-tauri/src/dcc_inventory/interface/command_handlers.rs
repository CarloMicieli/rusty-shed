use crate::core::domain::IdProvider;
use crate::core::infrastructure::error::CommandError;
use crate::dcc_inventory::application::DigitalRollingStockView;
use crate::dcc_inventory::application::NewDigitalRollingStockUseCase;
use crate::dcc_inventory::application::change_dcc_address::ChangeDccAddressInput;
use crate::dcc_inventory::application::change_decoder::ChangeDecoderInput;
use crate::dcc_inventory::application::new_digital_rolling_stock::NewDigitalRollingStockInput;
use crate::dcc_inventory::application::{
    ChangeDccAddressUseCase, ChangeDecoderUseCase, GetDigitalRollingStocksUseCase,
};
use crate::dcc_inventory::domain::DigitalRollingStockId;
use crate::dcc_inventory::interface::command_args::{
    ChangeDccAddressArgs, ChangeDecoderArgs, NewDigitalRollingStockArgs,
    ResponseNewDigitalRollingStock,
};
use crate::state::AppState;
use log::info;
use uuid::Uuid;

/// An ID provider for digital rolling stocks.
struct DigitalRollingStockIdProvider;

impl IdProvider<DigitalRollingStockId> for DigitalRollingStockIdProvider {
    fn next_id(&self) -> DigitalRollingStockId {
        DigitalRollingStockId::from_uuid(Uuid::new_v4())
    }
}

/// A command handler to create a new digital rolling stock.
///
/// # Arguments
/// - `state`: The application state.
/// - `args`: The command arguments.
///
/// # Returns
/// A result containing the response with the new digital rolling stock ID or a command error.
#[tauri::command]
#[specta::specta]
pub async fn new_digital_rolling_stock(
    state: tauri::State<'_, AppState>,
    args: NewDigitalRollingStockArgs,
) -> Result<ResponseNewDigitalRollingStock, CommandError> {
    info!("Creating digital rolling stock: {:?}", args);

    let domain_cmd = NewDigitalRollingStockInput::try_from(args).map_err(CommandError::from)?;

    let mut unit_of_work = state.unit_of_work().await?;

    let id_provider = DigitalRollingStockIdProvider;

    let id =
        NewDigitalRollingStockUseCase::execute(&mut unit_of_work, id_provider, domain_cmd).await?;

    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(ResponseNewDigitalRollingStock { id })
}

/// A command handler to change the DCC address of a digital rolling stock.
///
/// # Arguments
/// - `state`: The application state.
/// - `args`: The command arguments.
///
/// # Returns
/// A result indicating success or a command error.
#[tauri::command]
#[specta::specta]
pub async fn change_dcc_address(
    state: tauri::State<'_, AppState>,
    args: ChangeDccAddressArgs,
) -> Result<(), CommandError> {
    info!("Change DCC address: {:?}", args);

    let domain_cmd = ChangeDccAddressInput::try_from(args).map_err(CommandError::from)?;

    let mut unit_of_work = state.unit_of_work().await?;

    ChangeDccAddressUseCase::execute(&mut unit_of_work, domain_cmd).await?;
    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(())
}

/// A command handler to change the decoder of a digital rolling stock.
///
/// # Arguments
/// - `state`: The application state.
/// - `args`: The command arguments.
///
/// # Returns
/// A result indicating success or a command error.
#[tauri::command]
#[specta::specta]
pub async fn change_decoder(
    state: tauri::State<'_, AppState>,
    args: ChangeDecoderArgs,
) -> Result<(), CommandError> {
    info!("Change decoder: {:?}", args);

    let domain_cmd = ChangeDecoderInput::try_from(args).map_err(CommandError::from)?;

    let mut unit_of_work = state.unit_of_work().await?;

    ChangeDecoderUseCase::execute(&mut unit_of_work, domain_cmd).await?;
    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(())
}

/// A command handler to retrieve all digital rolling stocks.
///
/// # Arguments
/// - `state`: The application state.
///
/// # Returns
/// A result containing a list of digital rolling stock views or a command error.
#[tauri::command]
#[specta::specta]
pub async fn get_digital_rolling_stocks(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<DigitalRollingStockView>, CommandError> {
    info!("Getting digital rolling stocks");

    let mut unit_of_work = state.unit_of_work().await?;

    let list = GetDigitalRollingStocksUseCase::execute(&mut unit_of_work).await?;
    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(list)
}
