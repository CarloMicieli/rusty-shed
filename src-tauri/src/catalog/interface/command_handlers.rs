use crate::catalog::application::{
    AddRailwayModel, AddRollingStockToModel, DeleteRollingStock, GetCouplerTypes,
    GetCouplerTypesInput, GetRailwayModelTranslations, GetRailwayModelViewById,
    SearchRailwayModels, SetRollingStockCoupler, UpdateRailwayModelClassification,
    UpdateRailwayModelDeliveryDate, UpdateRailwayModelText, UpdateRollingStockCategory,
    UpdateRollingStockDcc, UpdateRollingStockIdentification, UpdateRollingStockRailwayCompany,
    UpdateRollingStockServiceLevel, UpdateRollingStockSpecifications,
    UpdateRollingStockSubcategory, UpsertRailwayModelTranslation, parse_add_rolling_stock_args,
};
use crate::catalog::domain::railway_model::RailwayModelId;
use crate::catalog::domain::railway_model::RailwayModelView;
use crate::catalog::domain::railway_model::railway_model_translation::RailwayModelTranslations;
use crate::catalog::domain::railway_model::{CouplerType, CouplingSocket};
use crate::catalog::interface::{
    AddRollingStockResult, AddRollingStockToModelArgs, CreateRailwayModelArgs,
    DeleteRollingStockArgs, SearchRailwayModelsArgs, SetRollingStockCouplerArgs,
    UpdateRailwayModelClassificationArgs, UpdateRailwayModelDeliveryDateArgs,
    UpdateRailwayModelTextArgs, UpdateRollingStockCategoryArgs, UpdateRollingStockDccArgs,
    UpdateRollingStockIdentificationArgs, UpdateRollingStockRailwayCompanyArgs,
    UpdateRollingStockServiceLevelArgs, UpdateRollingStockSpecificationsArgs,
    UpdateRollingStockSubcategoryArgs, UpsertRailwayModelTranslationArgs,
};
use crate::collecting::domain::CollectionUowExt;
use crate::core::domain::Language;
use crate::core::infrastructure::error::CommandError;
use crate::state::AppState;
use garde::Validate;
use tracing::info;

// ---------------------------------------------------------------------------
// Inner (testable) implementations – take &AppState directly
// ---------------------------------------------------------------------------

/// Retrieve a railway model by its identifier.
pub async fn get_railway_model_by_id_inner(
    state: &AppState,
    railway_model_id: RailwayModelId,
    lang: Language,
) -> Result<Option<RailwayModelView>, CommandError> {
    info!("Fetching railway model with ID: {}", railway_model_id);
    let mut uow = state.unit_of_work().await?;
    let railway_model = GetRailwayModelViewById::execute(&mut uow, &railway_model_id, lang).await?;
    uow.commit().await?;
    Ok(railway_model)
}

/// Tauri command to retrieve a railway model by its identifier.
#[tauri::command]
#[specta::specta]
pub async fn get_railway_model_by_id(
    state: tauri::State<'_, AppState>,
    railway_model_id: RailwayModelId,
    lang: Language,
) -> Result<Option<RailwayModelView>, CommandError> {
    get_railway_model_by_id_inner(&state, railway_model_id, lang).await
}

/// Create a new railway model along with its associated rolling stocks.
pub async fn create_railway_model_inner(
    state: &AppState,
    args: CreateRailwayModelArgs,
) -> Result<RailwayModelId, CommandError> {
    info!("Creating railway model: {:?}", args);
    args.validate().map_err(CommandError::from)?;
    let mut uow = state.unit_of_work().await?;
    let railway_model_input = args.try_into()?;
    let railway_model_id = AddRailwayModel::execute(&mut uow, railway_model_input).await?;
    uow.commit().await?;
    Ok(railway_model_id)
}

/// Tauri command to create a new railway model.
#[tauri::command]
#[specta::specta]
pub async fn create_railway_model(
    state: tauri::State<'_, AppState>,
    args: CreateRailwayModelArgs,
) -> Result<RailwayModelId, CommandError> {
    create_railway_model_inner(&state, args).await
}

/// Update a single free-text field (description or details) of a railway model.
pub async fn update_railway_model_text_inner(
    state: &AppState,
    args: UpdateRailwayModelTextArgs,
) -> Result<(), CommandError> {
    info!(
        "Updating railway model text field {:?} for {}",
        args.field, args.railway_model_id
    );
    args.validate().map_err(CommandError::from)?;
    let mut uow = state.unit_of_work().await?;
    UpdateRailwayModelText::execute(&mut uow, args.into()).await?;
    uow.commit().await?;
    Ok(())
}

/// Tauri command to update a text field of a railway model.
#[tauri::command]
#[specta::specta]
pub async fn update_railway_model_text(
    state: tauri::State<'_, AppState>,
    args: UpdateRailwayModelTextArgs,
) -> Result<(), CommandError> {
    update_railway_model_text_inner(&state, args).await
}

/// Update the identification fields of a single rolling stock unit.
pub async fn update_rolling_stock_identification_inner(
    state: &AppState,
    args: UpdateRollingStockIdentificationArgs,
) -> Result<(), CommandError> {
    info!(
        "Updating rolling stock identification for {} / {}",
        args.railway_model_id, args.rolling_stock_id
    );
    args.validate().map_err(CommandError::from)?;
    let mut uow = state.unit_of_work().await?;
    UpdateRollingStockIdentification::execute(&mut uow, args.into()).await?;
    uow.commit().await?;
    Ok(())
}

/// Tauri command to update the identification fields of a rolling stock unit.
#[tauri::command]
#[specta::specta]
pub async fn update_rolling_stock_identification(
    state: tauri::State<'_, AppState>,
    args: UpdateRollingStockIdentificationArgs,
) -> Result<(), CommandError> {
    update_rolling_stock_identification_inner(&state, args).await
}

/// Update the constrained classification fields (scale and/or epoch) of a railway model.
pub async fn update_railway_model_classification_inner(
    state: &AppState,
    args: UpdateRailwayModelClassificationArgs,
) -> Result<(), CommandError> {
    info!(
        "Updating railway model classification for {}",
        args.railway_model_id
    );
    args.validate().map_err(CommandError::from)?;
    let mut uow = state.unit_of_work().await?;
    UpdateRailwayModelClassification::execute(&mut uow, args.into()).await?;
    uow.commit().await?;
    Ok(())
}

/// Tauri command to update the classification of a railway model.
#[tauri::command]
#[specta::specta]
pub async fn update_railway_model_classification(
    state: tauri::State<'_, AppState>,
    args: UpdateRailwayModelClassificationArgs,
) -> Result<(), CommandError> {
    update_railway_model_classification_inner(&state, args).await
}

/// Update the delivery date of a railway model.
pub async fn update_railway_model_delivery_date_inner(
    state: &AppState,
    args: UpdateRailwayModelDeliveryDateArgs,
) -> Result<(), CommandError> {
    info!(
        "Updating delivery date for railway model {}",
        args.railway_model_id
    );
    args.validate().map_err(CommandError::from)?;
    let mut uow = state.unit_of_work().await?;
    UpdateRailwayModelDeliveryDate::execute(&mut uow, args.try_into()?).await?;
    uow.commit().await?;
    Ok(())
}

/// Tauri command to update the delivery date of a railway model.
#[tauri::command]
#[specta::specta]
pub async fn update_railway_model_delivery_date(
    state: tauri::State<'_, AppState>,
    args: UpdateRailwayModelDeliveryDateArgs,
) -> Result<(), CommandError> {
    update_railway_model_delivery_date_inner(&state, args).await
}

/// Update the railway company of a single rolling stock unit.
pub async fn update_rolling_stock_railway_company_inner(
    state: &AppState,
    args: UpdateRollingStockRailwayCompanyArgs,
) -> Result<(), CommandError> {
    info!(
        "Updating railway company for rolling stock {} / {}",
        args.railway_model_id, args.rolling_stock_id
    );
    args.validate().map_err(CommandError::from)?;
    let mut uow = state.unit_of_work().await?;
    UpdateRollingStockRailwayCompany::execute(&mut uow, args.into()).await?;
    uow.commit().await?;
    Ok(())
}

/// Tauri command to update the railway company of a rolling stock unit.
#[tauri::command]
#[specta::specta]
pub async fn update_rolling_stock_railway_company(
    state: tauri::State<'_, AppState>,
    args: UpdateRollingStockRailwayCompanyArgs,
) -> Result<(), CommandError> {
    update_rolling_stock_railway_company_inner(&state, args).await
}

/// Change the category (variant) of a single rolling stock unit.
pub async fn update_rolling_stock_category_inner(
    state: &AppState,
    args: UpdateRollingStockCategoryArgs,
) -> Result<(), CommandError> {
    info!(
        "Updating category for rolling stock {} / {}",
        args.railway_model_id, args.rolling_stock_id
    );
    args.validate().map_err(CommandError::from)?;
    let mut uow = state.unit_of_work().await?;
    UpdateRollingStockCategory::execute(&mut uow, args.into()).await?;
    uow.commit().await?;
    Ok(())
}

/// Tauri command to update the category of a rolling stock unit.
#[tauri::command]
#[specta::specta]
pub async fn update_rolling_stock_category(
    state: tauri::State<'_, AppState>,
    args: UpdateRollingStockCategoryArgs,
) -> Result<(), CommandError> {
    update_rolling_stock_category_inner(&state, args).await
}

/// Update the control type, DCC interface, and length of a single rolling stock unit.
pub async fn update_rolling_stock_dcc_inner(
    state: &AppState,
    args: UpdateRollingStockDccArgs,
) -> Result<(), CommandError> {
    info!(
        "Updating DCC/length for rolling stock {} / {}",
        args.railway_model_id, args.rolling_stock_id
    );
    let mut uow = state.unit_of_work().await?;
    UpdateRollingStockDcc::execute(&mut uow, args.into()).await?;
    uow.commit().await?;
    Ok(())
}

/// Tauri command to update DCC/length fields of a rolling stock unit.
#[tauri::command]
#[specta::specta]
pub async fn update_rolling_stock_dcc(
    state: tauri::State<'_, AppState>,
    args: UpdateRollingStockDccArgs,
) -> Result<(), CommandError> {
    update_rolling_stock_dcc_inner(&state, args).await
}

/// Update the full technical specifications of a single rolling stock unit (drawer save).
pub async fn update_rolling_stock_specifications_inner(
    state: &AppState,
    args: UpdateRollingStockSpecificationsArgs,
) -> Result<(), CommandError> {
    info!(
        "Updating rolling stock specifications for {} / {}",
        args.railway_model_id, args.rolling_stock_id
    );
    args.validate().map_err(CommandError::from)?;
    let mut uow = state.unit_of_work().await?;
    UpdateRollingStockSpecifications::execute(&mut uow, args.try_into()?).await?;
    uow.commit().await?;
    Ok(())
}

/// Tauri command to update the full technical specifications of a rolling stock unit.
#[tauri::command]
#[specta::specta]
pub async fn update_rolling_stock_specifications(
    state: tauri::State<'_, AppState>,
    args: UpdateRollingStockSpecificationsArgs,
) -> Result<(), CommandError> {
    update_rolling_stock_specifications_inner(&state, args).await
}

/// Retrieve all stored translations for a railway model.
pub async fn get_railway_model_translations_inner(
    state: &AppState,
    railway_model_id: RailwayModelId,
) -> Result<Option<RailwayModelTranslations>, CommandError> {
    info!(
        "Fetching translations for railway model {}",
        railway_model_id
    );
    let mut uow = state.unit_of_work().await?;
    let translations = GetRailwayModelTranslations::execute(&mut uow, &railway_model_id).await?;
    uow.commit().await?;
    Ok(translations)
}

/// Tauri command to retrieve all translations for a railway model.
#[tauri::command]
#[specta::specta]
pub async fn get_railway_model_translations(
    state: tauri::State<'_, AppState>,
    railway_model_id: RailwayModelId,
) -> Result<Option<RailwayModelTranslations>, CommandError> {
    get_railway_model_translations_inner(&state, railway_model_id).await
}

/// Create or replace a translation for one language on a railway model.
pub async fn upsert_railway_model_translation_inner(
    state: &AppState,
    args: UpsertRailwayModelTranslationArgs,
) -> Result<(), CommandError> {
    info!(
        "Upserting {} translation for railway model {}",
        args.lang, args.railway_model_id
    );
    args.validate().map_err(CommandError::from)?;
    let mut uow = state.unit_of_work().await?;
    UpsertRailwayModelTranslation::execute(&mut uow, args.into()).await?;
    uow.commit().await?;

    Ok(())
}

/// Tauri command to upsert a language translation for a railway model.
#[tauri::command]
#[specta::specta]
pub async fn upsert_railway_model_translation(
    state: tauri::State<'_, AppState>,
    args: UpsertRailwayModelTranslationArgs,
) -> Result<(), CommandError> {
    upsert_railway_model_translation_inner(&state, args).await
}

/// Search railway models using FTS5 full-text search across all language translations.
pub async fn search_railway_models_inner(
    state: &AppState,
    args: SearchRailwayModelsArgs,
) -> Result<Vec<RailwayModelId>, CommandError> {
    info!("Searching railway models with query: {}", args.query);
    args.validate().map_err(CommandError::from)?;
    let mut uow = state.unit_of_work().await?;
    let ids = SearchRailwayModels::execute(&mut uow, args.into()).await?;
    uow.commit().await?;
    Ok(ids)
}

/// Tauri command to search railway models.
#[tauri::command]
#[specta::specta]
pub async fn search_railway_models(
    state: tauri::State<'_, AppState>,
    args: SearchRailwayModelsArgs,
) -> Result<Vec<RailwayModelId>, CommandError> {
    search_railway_models_inner(&state, args).await
}

/// Add a new rolling stock variant to an existing Railway Model.
pub async fn add_rolling_stock_to_model_inner(
    state: &AppState,
    args: AddRollingStockToModelArgs,
) -> Result<AddRollingStockResult, CommandError> {
    info!(
        "Adding rolling stock to model {} (category: {})",
        args.railway_model_id, args.category
    );
    args.validate().map_err(CommandError::from)?;
    let input = parse_add_rolling_stock_args(
        args.railway_model_id,
        args.railway_company_id,
        args.category,
        args.series_code,
        args.road_number,
        args.livery,
        args.depot,
        args.control,
        args.dcc_interface,
        args.coupling_socket,
        args.close_couplers,
        args.sub_type,
        args.friendly_name,
        args.prototype_id,
        args.is_dummy,
    )?;
    let railway_model_id = input.railway_model_id.clone();
    let mut uow = state.unit_of_work().await?;
    let rs_id = AddRollingStockToModel::execute(&mut uow, input).await?;

    let owned_ids = uow
        .collections_repository()
        .add_owned_rolling_stock_for_collection_items(&railway_model_id, &rs_id)
        .await
        .map_err(CommandError::from)?;

    uow.commit().await?;

    let owned_rolling_stock_id = owned_ids.into_iter().next();

    Ok(AddRollingStockResult {
        rolling_stock_id: rs_id,
        owned_rolling_stock_id,
    })
}

/// Tauri command to add a rolling stock variant to a railway model.
#[tauri::command]
#[specta::specta]
pub async fn add_rolling_stock_to_model(
    state: tauri::State<'_, AppState>,
    args: AddRollingStockToModelArgs,
) -> Result<AddRollingStockResult, CommandError> {
    add_rolling_stock_to_model_inner(&state, args).await
}

/// Delete a rolling stock variant from an existing Railway Model.
pub async fn delete_rolling_stock_inner(
    state: &AppState,
    args: DeleteRollingStockArgs,
) -> Result<(), CommandError> {
    info!(
        "Deleting rolling stock {} from model {}",
        args.rolling_stock_id, args.railway_model_id
    );
    args.validate().map_err(CommandError::from)?;
    let mut uow = state.unit_of_work().await?;
    DeleteRollingStock::execute(&mut uow, args.into()).await?;
    uow.commit().await?;
    Ok(())
}

/// Tauri command to delete a rolling stock variant from a railway model.
#[tauri::command]
#[specta::specta]
pub async fn delete_rolling_stock(
    state: tauri::State<'_, AppState>,
    args: DeleteRollingStockArgs,
) -> Result<(), CommandError> {
    delete_rolling_stock_inner(&state, args).await
}

/// Update the subcategory (type field) of a single rolling stock unit.
pub async fn update_rolling_stock_subcategory_inner(
    state: &AppState,
    args: UpdateRollingStockSubcategoryArgs,
) -> Result<(), CommandError> {
    info!(
        "Updating subcategory for rolling stock {} / {}",
        args.railway_model_id, args.rolling_stock_id
    );
    args.validate().map_err(CommandError::from)?;
    let mut uow = state.unit_of_work().await?;
    UpdateRollingStockSubcategory::execute(&mut uow, args.into()).await?;
    uow.commit().await?;
    Ok(())
}

/// Tauri command to update the subcategory of a rolling stock unit.
#[tauri::command]
#[specta::specta]
pub async fn update_rolling_stock_subcategory(
    state: tauri::State<'_, AppState>,
    args: UpdateRollingStockSubcategoryArgs,
) -> Result<(), CommandError> {
    update_rolling_stock_subcategory_inner(&state, args).await
}

/// Update the service level of a single rolling stock unit.
pub async fn update_rolling_stock_service_level_inner(
    state: &AppState,
    args: UpdateRollingStockServiceLevelArgs,
) -> Result<(), CommandError> {
    info!(
        "Updating service level for rolling stock {} / {}",
        args.railway_model_id, args.rolling_stock_id
    );
    args.validate().map_err(CommandError::from)?;
    let mut uow = state.unit_of_work().await?;
    UpdateRollingStockServiceLevel::execute(&mut uow, args.into()).await?;
    uow.commit().await?;
    Ok(())
}

/// Tauri command to update the service level of a rolling stock unit.
#[tauri::command]
#[specta::specta]
pub async fn update_rolling_stock_service_level(
    state: tauri::State<'_, AppState>,
    args: UpdateRollingStockServiceLevelArgs,
) -> Result<(), CommandError> {
    update_rolling_stock_service_level_inner(&state, args).await
}

/// Return the coupler type catalogue, optionally filtered to a specific coupling socket.
///
/// When `socket` is provided (e.g. `"NEM_362"`), only couplers compatible with that
/// socket are returned. When omitted, the full catalogue is returned.
///
/// # Arguments
/// * `state` - Tauri-managed application `AppState` providing the database pool.
/// * `socket` - Optional coupling socket filter string.
///
/// # Returns
/// - `Ok(Vec<CouplerType>)` on success.
/// - `Err(CommandError::DatabaseError)` on persistence failure.
pub async fn get_coupler_types_inner(
    state: &AppState,
    socket: Option<String>,
) -> Result<Vec<CouplerType>, CommandError> {
    let socket_filter = socket
        .as_deref()
        .map(|s| s.parse::<CouplingSocket>())
        .transpose()
        .map_err(|_| CommandError::validation_field("socket", "Invalid coupling socket value"))?;

    let mut unit_of_work = state.unit_of_work().await?;
    let result = GetCouplerTypes::execute(
        &mut unit_of_work,
        GetCouplerTypesInput {
            socket: socket_filter,
        },
    )
    .await?;
    unit_of_work.commit().await?;
    Ok(result)
}

/// # Returns
/// - `Ok(Vec<CouplerType>)` on success.
/// - `Err(CommandError::DatabaseError)` on persistence failure.
#[tauri::command]
#[specta::specta]
pub async fn get_coupler_types(
    state: tauri::State<'_, AppState>,
    socket: Option<String>,
) -> Result<Vec<CouplerType>, CommandError> {
    get_coupler_types_inner(&state, socket).await
}

/// Set (or clear) the installed coupler on an owned rolling stock.
///
/// After updating `current_coupler_id`, a `CouplerChange` maintenance event is
/// automatically recorded on the rolling stock's maintenance card if one exists.
///
/// # Arguments
/// * `state` - Tauri-managed application `AppState` providing the database pool.
/// * `args` - The owned rolling stock id and the coupler type id to install (or `null`).
///
/// # Returns
/// - `Ok(())` on success.
/// - `Err(CommandError::DatabaseError)` on persistence failure.
#[tauri::command]
#[specta::specta]
pub async fn set_rolling_stock_coupler(
    state: tauri::State<'_, AppState>,
    args: SetRollingStockCouplerArgs,
) -> Result<(), CommandError> {
    info!(
        "Setting coupler {:?} on owned rolling stock {}",
        args.coupler_type_id, args.owned_rolling_stock_id
    );

    let mut unit_of_work = state.unit_of_work().await?;
    SetRollingStockCoupler::execute(&mut unit_of_work, args.into()).await?;
    unit_of_work.commit().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    fn app_state(pool: SqlitePool) -> AppState {
        AppState::for_test(pool)
    }

    // ── create_railway_model_inner ─────────────────────────────────────────

    /// Validation is enforced before any database access: an empty manufacturer_id
    /// must immediately return a `ValidationError`, not a `DatabaseError`.
    #[sqlx::test(migrations = "./migrations")]
    async fn create_railway_model_empty_manufacturer_id_returns_validation_error(pool: SqlitePool) {
        let state = app_state(pool);
        let args = CreateRailwayModelArgs {
            manufacturer_id: String::new(),
            product_code: "60100".to_string(),
            description: "Some locomotive".to_string(),
            details: None,
            power_method: "DC".to_string(),
            scale: "H0".to_string(),
            epoch: "VI".to_string(),
            category: "LOCOMOTIVES".to_string(),
            delivery_date: None,
            availability_status: None,
            rolling_stocks: vec![],
        };
        let result = create_railway_model_inner(&state, args).await;
        assert!(
            matches!(result, Err(CommandError::ValidationError(_))),
            "Expected ValidationError, got: {:?}",
            result
        );
    }

    /// An invalid scale string must be caught by validation before the DB is touched.
    #[sqlx::test(migrations = "./migrations")]
    async fn create_railway_model_invalid_scale_returns_validation_error(pool: SqlitePool) {
        let state = app_state(pool);
        let args = CreateRailwayModelArgs {
            manufacturer_id: "trn:manufacturer:acme".to_string(),
            product_code: "60100".to_string(),
            description: "Some locomotive".to_string(),
            details: None,
            power_method: "DC".to_string(),
            scale: "INVALID_SCALE".to_string(),
            epoch: "VI".to_string(),
            category: "LOCOMOTIVES".to_string(),
            delivery_date: None,
            availability_status: None,
            rolling_stocks: vec![],
        };
        let result = create_railway_model_inner(&state, args).await;
        assert!(
            matches!(result, Err(CommandError::ValidationError(_))),
            "Expected ValidationError, got: {:?}",
            result
        );
    }

    /// An empty rolling_stocks list must fail validation (at least one required).
    #[sqlx::test(migrations = "./migrations")]
    async fn create_railway_model_empty_rolling_stocks_returns_validation_error(pool: SqlitePool) {
        let state = app_state(pool);
        let args = CreateRailwayModelArgs {
            manufacturer_id: "trn:manufacturer:acme".to_string(),
            product_code: "60100".to_string(),
            description: "Some locomotive".to_string(),
            details: None,
            power_method: "DC".to_string(),
            scale: "H0".to_string(),
            epoch: "VI".to_string(),
            category: "LOCOMOTIVES".to_string(),
            delivery_date: None,
            availability_status: None,
            rolling_stocks: vec![],
        };
        let result = create_railway_model_inner(&state, args).await;
        assert!(
            matches!(result, Err(CommandError::ValidationError(_))),
            "Expected ValidationError, got: {:?}",
            result
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn add_rolling_stock_to_model_invalid_railway_model_id_returns_validation_error(
        pool: SqlitePool,
    ) {
        let state = app_state(pool);
        let args = AddRollingStockToModelArgs {
            railway_model_id: "not-a-railway-model-id".to_string(),
            railway_company_id: "trn:railway-company:fs".to_string(),
            category: "LOCOMOTIVE".to_string(),
            series_code: "E444".to_string(),
            road_number: None,
            livery: None,
            depot: None,
            control: None,
            dcc_interface: None,
            coupling_socket: None,
            close_couplers: None,
            sub_type: None,
            friendly_name: None,
            prototype_id: None,
            is_dummy: None,
        };

        let result = add_rolling_stock_to_model_inner(&state, args).await;
        assert!(matches!(result, Err(CommandError::ValidationError(_))));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn update_railway_model_text_missing_model_returns_not_found(pool: SqlitePool) {
        let state = app_state(pool);
        let args = UpdateRailwayModelTextArgs {
            railway_model_id: RailwayModelId::try_from("trn:railway-model:acme:missing")
                .expect("valid railway model id"),
            field: crate::catalog::application::RailwayModelTextField::Description,
            value: "Updated description".to_string(),
            lang: Language::English,
        };

        let result = update_railway_model_text_inner(&state, args).await;
        assert!(matches!(result, Err(CommandError::NotFound(_))));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn update_rolling_stock_identification_missing_model_returns_not_found(pool: SqlitePool) {
        let state = app_state(pool);
        let args = UpdateRollingStockIdentificationArgs {
            railway_model_id: RailwayModelId::try_from("trn:railway-model:acme:missing")
                .expect("valid railway model id"),
            rolling_stock_id: crate::catalog::domain::railway_model::RollingStockId::default(),
            series_code: "E444".to_string(),
            road_number: Some("001".to_string()),
            livery: Some("XMPR".to_string()),
            depot: Some("Roma".to_string()),
        };

        let result = update_rolling_stock_identification_inner(&state, args).await;
        assert!(matches!(result, Err(CommandError::NotFound(_))));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn update_railway_model_classification_missing_model_returns_not_found(pool: SqlitePool) {
        let state = app_state(pool);
        let args = UpdateRailwayModelClassificationArgs {
            railway_model_id: RailwayModelId::try_from("trn:railway-model:acme:missing")
                .expect("valid railway model id"),
            scale: Some(crate::catalog::domain::scale::Scale::H0),
            epoch: None,
            category: None,
        };

        let result = update_railway_model_classification_inner(&state, args).await;
        assert!(matches!(result, Err(CommandError::NotFound(_))));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn update_rolling_stock_railway_company_missing_model_returns_not_found(
        pool: SqlitePool,
    ) {
        let state = app_state(pool);
        let args = UpdateRollingStockRailwayCompanyArgs {
            railway_model_id: RailwayModelId::try_from("trn:railway-model:acme:missing")
                .expect("valid railway model id"),
            rolling_stock_id: crate::catalog::domain::railway_model::RollingStockId::default(),
            railway_company_id:
                crate::catalog::domain::railway_company::RailwayCompanyId::try_from(
                    "trn:railway-company:fs",
                )
                .expect("valid railway company id"),
        };

        let result = update_rolling_stock_railway_company_inner(&state, args).await;
        assert!(matches!(result, Err(CommandError::NotFound(_))));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn update_rolling_stock_category_missing_model_returns_not_found(pool: SqlitePool) {
        let state = app_state(pool);
        let args = UpdateRollingStockCategoryArgs {
            railway_model_id: RailwayModelId::try_from("trn:railway-model:acme:missing")
                .expect("valid railway model id"),
            rolling_stock_id: crate::catalog::domain::railway_model::RollingStockId::default(),
            category: crate::catalog::domain::railway_model::RollingStockCategory::Locomotive,
        };

        let result = update_rolling_stock_category_inner(&state, args).await;
        assert!(matches!(result, Err(CommandError::NotFound(_))));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn upsert_railway_model_translation_missing_model_returns_not_found(pool: SqlitePool) {
        let state = app_state(pool);
        let args = UpsertRailwayModelTranslationArgs {
            railway_model_id: RailwayModelId::try_from("trn:railway-model:acme:missing")
                .expect("valid railway model id"),
            lang: Language::English,
            description: Some("Translated description".to_string()),
            details: Some("Translated details".to_string()),
        };

        let result = upsert_railway_model_translation_inner(&state, args).await;
        assert!(matches!(result, Err(CommandError::NotFound(_))));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn search_railway_models_query_too_short_returns_validation_error(pool: SqlitePool) {
        let state = app_state(pool);
        let args = SearchRailwayModelsArgs {
            query: "x".to_string(),
        };

        let result = search_railway_models_inner(&state, args).await;
        assert!(matches!(result, Err(CommandError::ValidationError(_))));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn delete_rolling_stock_missing_model_returns_not_found(pool: SqlitePool) {
        let state = app_state(pool);
        let args = DeleteRollingStockArgs {
            railway_model_id: RailwayModelId::try_from("trn:railway-model:acme:missing")
                .expect("valid railway model id"),
            rolling_stock_id: crate::catalog::domain::railway_model::RollingStockId::default(),
        };

        let result = delete_rolling_stock_inner(&state, args).await;
        assert!(matches!(result, Err(CommandError::NotFound(_))));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn update_rolling_stock_subcategory_missing_model_returns_not_found(pool: SqlitePool) {
        let state = app_state(pool);
        let args = UpdateRollingStockSubcategoryArgs {
            railway_model_id: RailwayModelId::try_from("trn:railway-model:acme:missing")
                .expect("valid railway model id"),
            rolling_stock_id: crate::catalog::domain::railway_model::RollingStockId::default(),
            subcategory: "ELECTRIC_LOCOMOTIVE".to_string(),
        };

        let result = update_rolling_stock_subcategory_inner(&state, args).await;
        assert!(matches!(result, Err(CommandError::NotFound(_))));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn update_rolling_stock_service_level_missing_model_returns_not_found(pool: SqlitePool) {
        let state = app_state(pool);
        let args = UpdateRollingStockServiceLevelArgs {
            railway_model_id: RailwayModelId::try_from("trn:railway-model:acme:missing")
                .expect("valid railway model id"),
            rolling_stock_id: crate::catalog::domain::railway_model::RollingStockId::default(),
            service_level: None,
        };

        let result = update_rolling_stock_service_level_inner(&state, args).await;
        assert!(matches!(result, Err(CommandError::NotFound(_))));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn get_coupler_types_inner_invalid_socket_returns_validation_error(pool: SqlitePool) {
        let state = app_state(pool);
        let result = get_coupler_types_inner(&state, Some("not-a-socket".to_string())).await;
        assert!(matches!(result, Err(CommandError::ValidationError(_))));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn get_coupler_types_inner_without_filter_succeeds(pool: SqlitePool) {
        let state = app_state(pool);
        let result = get_coupler_types_inner(&state, None).await;
        assert!(result.is_ok());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn update_railway_model_delivery_date_invalid_date_returns_validation_error(
        pool: SqlitePool,
    ) {
        let state = app_state(pool);
        let args = UpdateRailwayModelDeliveryDateArgs {
            railway_model_id: RailwayModelId::try_from("trn:railway-model:acme:60100")
                .expect("valid railway model id"),
            delivery_date: Some("not-a-delivery-date".to_string()),
        };

        let result = update_railway_model_delivery_date_inner(&state, args).await;
        assert!(matches!(result, Err(CommandError::ValidationError(_))));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn update_rolling_stock_specifications_invalid_body_shell_returns_validation_error(
        pool: SqlitePool,
    ) {
        let state = app_state(pool);
        let args = UpdateRollingStockSpecificationsArgs {
            railway_model_id: RailwayModelId::try_from("trn:railway-model:acme:60100")
                .expect("valid railway model id"),
            rolling_stock_id: crate::catalog::domain::railway_model::RollingStockId::default(),
            series_code: "E444".to_string(),
            road_number: None,
            livery: None,
            depot: None,
            series: None,
            friendly_name: None,
            flywheel_fitted: None,
            body_shell: Some("NotARealBodyShell".to_string()),
            chassis: None,
            interior_lights: None,
            lights: None,
            sprung_buffers: None,
            dcc_interface: None,
            control: None,
            coupling_socket: None,
            close_couplers: None,
            digital_shunting: None,
            is_dummy: None,
        };

        let result = update_rolling_stock_specifications_inner(&state, args).await;
        assert!(matches!(result, Err(CommandError::ValidationError(_))));
    }
}
