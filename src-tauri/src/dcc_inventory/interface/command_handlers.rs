use crate::core::domain::IdProvider;
use crate::core::infrastructure::error::CommandError;
use crate::dcc_inventory::application::change_dcc_address::ChangeDccAddressInput;
use crate::dcc_inventory::application::change_decoder::ChangeDecoderInput;
use crate::dcc_inventory::application::new_digital_rolling_stock::NewDigitalRollingStockInput;
use crate::dcc_inventory::application::{
    ChangeDccAddressUseCase, ChangeDecoderUseCase, CheckDuplicateAddressResult,
    CheckDuplicateAddressUseCase, DigitalRollingStockView, DigitalSummary, GetDecodersUseCase,
    GetDigitalRollingStocksUseCase, GetDigitalSummaryUseCase, GetInstallableRollingStocksUseCase,
    InstallableRollingStockView, NewDigitalRollingStockUseCase,
};
use crate::dcc_inventory::domain::{DccAddress, Decoder, DigitalRollingStockId};
use crate::dcc_inventory::interface::command_args::{
    ChangeDccAddressArgs, ChangeDecoderArgs, CheckDccAddressDuplicateArgs,
    NewDigitalRollingStockArgs, ResponseNewDigitalRollingStock,
};
use crate::state::AppState;
use tracing::info;
use uuid::Uuid;

/// An ID provider for digital rolling stocks.
struct DigitalRollingStockIdProvider;

impl IdProvider<DigitalRollingStockId> for DigitalRollingStockIdProvider {
    fn next_id(&self) -> DigitalRollingStockId {
        DigitalRollingStockId::from_uuid(Uuid::new_v4())
    }
}

pub async fn new_digital_rolling_stock_inner(
    state: &AppState,
    args: NewDigitalRollingStockArgs,
) -> Result<ResponseNewDigitalRollingStock, CommandError> {
    let domain_cmd = NewDigitalRollingStockInput::try_from(args).map_err(CommandError::from)?;
    let mut unit_of_work = state.unit_of_work().await?;
    let id_provider = DigitalRollingStockIdProvider;
    let id =
        NewDigitalRollingStockUseCase::execute(&mut unit_of_work, id_provider, domain_cmd).await?;
    unit_of_work.commit().await?;
    Ok(ResponseNewDigitalRollingStock { id })
}

pub async fn change_dcc_address_inner(
    state: &AppState,
    args: ChangeDccAddressArgs,
) -> Result<(), CommandError> {
    let domain_cmd = ChangeDccAddressInput::try_from(args).map_err(CommandError::from)?;
    let mut unit_of_work = state.unit_of_work().await?;
    ChangeDccAddressUseCase::execute(&mut unit_of_work, domain_cmd).await?;
    unit_of_work.commit().await?;
    Ok(())
}

pub async fn change_decoder_inner(
    state: &AppState,
    args: ChangeDecoderArgs,
) -> Result<(), CommandError> {
    let domain_cmd = ChangeDecoderInput::try_from(args).map_err(CommandError::from)?;
    let mut unit_of_work = state.unit_of_work().await?;
    ChangeDecoderUseCase::execute(&mut unit_of_work, domain_cmd).await?;
    unit_of_work.commit().await?;
    Ok(())
}

pub async fn get_digital_rolling_stocks_inner(
    state: &AppState,
) -> Result<Vec<DigitalRollingStockView>, CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;
    let list = GetDigitalRollingStocksUseCase::execute(&mut unit_of_work).await?;
    unit_of_work.commit().await?;
    Ok(list)
}

pub async fn get_digital_summary_inner(state: &AppState) -> Result<DigitalSummary, CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;
    let summary = GetDigitalSummaryUseCase::execute(&mut unit_of_work).await?;
    unit_of_work.commit().await?;
    Ok(summary)
}

pub async fn get_decoders_inner(state: &AppState) -> Result<Vec<Decoder>, CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;
    let decoders = GetDecodersUseCase::execute(&mut unit_of_work).await?;
    unit_of_work.commit().await?;
    Ok(decoders)
}

pub async fn check_dcc_address_duplicate_inner(
    state: &AppState,
    args: CheckDccAddressDuplicateArgs,
) -> Result<CheckDuplicateAddressResult, CommandError> {
    let dcc_address = DccAddress::new(args.dcc_address).map_err(CommandError::from)?;
    let exclude_id = if let Some(id_str) = args.exclude_id {
        let id = DigitalRollingStockId::try_from(id_str.as_str())
            .map_err(|e| CommandError::unknown(e.to_string()))?;
        Some(id)
    } else {
        None
    };
    let mut unit_of_work = state.unit_of_work().await?;
    let result =
        CheckDuplicateAddressUseCase::execute(&mut unit_of_work, dcc_address, exclude_id).await?;
    unit_of_work.commit().await?;
    Ok(result)
}

pub async fn get_installable_rolling_stocks_inner(
    state: &AppState,
) -> Result<Vec<InstallableRollingStockView>, CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;
    let list = GetInstallableRollingStocksUseCase::execute(&mut unit_of_work).await?;
    unit_of_work.commit().await?;
    Ok(list)
}

// ---------------------------------------------------------------------------
// Tauri command wrappers (thin delegation to the inner functions above)
// ---------------------------------------------------------------------------

/// A command handler to create a new digital rolling stock.
#[tauri::command]
#[specta::specta]
pub async fn new_digital_rolling_stock(
    state: tauri::State<'_, AppState>,
    args: NewDigitalRollingStockArgs,
) -> Result<ResponseNewDigitalRollingStock, CommandError> {
    info!("Creating digital rolling stock: {:?}", args);
    new_digital_rolling_stock_inner(&state, args).await
}

/// A command handler to change the DCC address of a digital rolling stock.
#[tauri::command]
#[specta::specta]
pub async fn change_dcc_address(
    state: tauri::State<'_, AppState>,
    args: ChangeDccAddressArgs,
) -> Result<(), CommandError> {
    info!("Change DCC address: {:?}", args);
    change_dcc_address_inner(&state, args).await
}

/// A command handler to change the decoder of a digital rolling stock.
#[tauri::command]
#[specta::specta]
pub async fn change_decoder(
    state: tauri::State<'_, AppState>,
    args: ChangeDecoderArgs,
) -> Result<(), CommandError> {
    info!("Change decoder: {:?}", args);
    change_decoder_inner(&state, args).await
}

/// A command handler to retrieve all digital rolling stocks.
#[tauri::command]
#[specta::specta]
pub async fn get_digital_rolling_stocks(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<DigitalRollingStockView>, CommandError> {
    info!("Getting digital rolling stocks");
    get_digital_rolling_stocks_inner(&state).await
}

/// A command handler to get the digital rolling stock summary.
#[tauri::command]
#[specta::specta]
pub async fn get_digital_summary(
    state: tauri::State<'_, AppState>,
) -> Result<DigitalSummary, CommandError> {
    info!("Getting digital summary");
    get_digital_summary_inner(&state).await
}

/// A command handler to get all available decoders.
#[tauri::command]
#[specta::specta]
pub async fn get_decoders(state: tauri::State<'_, AppState>) -> Result<Vec<Decoder>, CommandError> {
    info!("Getting decoders");
    get_decoders_inner(&state).await
}

/// A command handler to check if a DCC address is a duplicate.
#[tauri::command]
#[specta::specta]
pub async fn check_dcc_address_duplicate(
    state: tauri::State<'_, AppState>,
    args: CheckDccAddressDuplicateArgs,
) -> Result<CheckDuplicateAddressResult, CommandError> {
    info!("Checking DCC address duplicate: {:?}", args);
    check_dcc_address_duplicate_inner(&state, args).await
}

/// A command handler to get all installable rolling stocks.
#[tauri::command]
#[specta::specta]
pub async fn get_installable_rolling_stocks(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<InstallableRollingStockView>, CommandError> {
    info!("Getting installable rolling stocks");
    get_installable_rolling_stocks_inner(&state).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_uow::testing::{MockAppUow, OneShotFactory};
    use crate::dcc_inventory::domain::MockDigitalRollingStockRepository;
    use sqlx::SqlitePool;
    use std::sync::Arc;

    /// Helper: build an `AppState` backed by an in-memory pool (not used for
    /// UoW) with the given pre-configured mock unit of work.
    async fn state_with_uow(uow: MockAppUow) -> AppState {
        let pool = SqlitePool::connect(":memory:")
            .await
            .expect("in-memory pool");
        AppState::new_with_factory(pool, Arc::new(OneShotFactory::new(uow)))
    }

    #[tokio::test]
    async fn get_decoders_returns_empty_list() {
        let mut repo = MockDigitalRollingStockRepository::new();
        repo.expect_find_all_decoders()
            .times(1)
            .returning(|| Ok(vec![]));

        let uow = MockAppUow::new().with_dcc_inventory(repo);
        let state = state_with_uow(uow).await;

        let result = get_decoders_inner(&state).await.expect("should succeed");
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn get_decoders_propagates_repository_error() {
        let mut repo = MockDigitalRollingStockRepository::new();
        repo.expect_find_all_decoders().times(1).returning(|| {
            Err(
                crate::core::domain::domain_error::DomainError::Infrastructure(
                    "db down".to_string(),
                ),
            )
        });

        let uow = MockAppUow::new().with_dcc_inventory(repo);
        let state = state_with_uow(uow).await;

        let result = get_decoders_inner(&state).await;
        assert!(result.is_err());
    }
}
