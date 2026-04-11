//! Tauri command handlers for the train-formations feature.
//!
//! Each handler follows the "Inner-Shim" pattern:
//!  1. A public `*_inner(state: &AppState, ...)` function holds the testable logic.
//!  2. A thin `#[tauri::command]` wrapper validates args and delegates to `*_inner`.

use crate::core::infrastructure::error::CommandError;
use crate::state::AppState;
use crate::trains::application::{
    add_formation_element::AddFormationElementUseCase,
    assign_rolling_stock_to_element::AssignRollingStockToElementUseCase,
    create_custom_prototype::CreateCustomPrototypeUseCase,
    create_formation_category::CreateFormationCategoryUseCase,
    create_train_formation::CreateTrainFormationUseCase,
    delete_train_formation::DeleteTrainFormationUseCase,
    get_formation_categories::GetFormationCategoriesUseCase, get_prototypes::GetPrototypesUseCase,
    get_train_formation::GetTrainFormationUseCase, get_train_formations::GetTrainFormationsUseCase,
    remove_formation_element::RemoveFormationElementUseCase,
    reorder_formation_elements::ReorderFormationElementsUseCase,
    set_traction_override::SetTractionOverrideUseCase,
    update_train_formation::UpdateTrainFormationUseCase,
};
use crate::trains::domain::{
    FormationCategoryView, FormationElementView, PrototypeGroupView, PrototypeView,
    TrainFormationDetail, TrainFormationSummary, TrainFormationView,
};
use crate::trains::interface::command_args::{
    AddFormationElementArgs, AssignRollingStockToElementArgs, CreateCustomPrototypeArgs,
    CreateFormationCategoryArgs, CreateTrainFormationArgs, ReorderFormationElementsArgs,
    SetTractionOverrideArgs, UpdateTrainFormationArgs,
};
use garde::Validate;
use log::info;

// ── Formation CRUD ────────────────────────────────────────────────────────────

pub async fn create_train_formation_inner(
    state: &AppState,
    args: CreateTrainFormationArgs,
) -> Result<TrainFormationView, CommandError> {
    info!("Creating train formation: {:?}", args);

    let mut uow = state.unit_of_work().await?;
    let result = CreateTrainFormationUseCase::execute(
        &mut uow,
        args.name,
        args.category_id,
        args.start_year.map(|y| y.value()),
        args.end_year.map(|y| y.value()),
        args.epoch,
        args.notes,
    )
    .await?;
    uow.commit().await?;
    Ok(result)
}

pub async fn update_train_formation_inner(
    state: &AppState,
    id: String,
    args: UpdateTrainFormationArgs,
) -> Result<TrainFormationView, CommandError> {
    info!("Updating train formation {}: {:?}", id, args);

    let mut uow = state.unit_of_work().await?;
    let result = UpdateTrainFormationUseCase::execute(
        &mut uow,
        id,
        args.name,
        args.category_id,
        args.start_year.map(|y| y.value()),
        args.end_year.map(|y| y.value()),
        args.epoch,
        args.notes,
    )
    .await?;
    uow.commit().await?;
    Ok(result)
}

pub async fn delete_train_formation_inner(
    state: &AppState,
    id: String,
) -> Result<(), CommandError> {
    info!("Deleting train formation {}", id);

    let mut uow = state.unit_of_work().await?;
    DeleteTrainFormationUseCase::execute(&mut uow, id).await?;
    uow.commit().await?;
    Ok(())
}

pub async fn get_train_formation_inner(
    state: &AppState,
    id: String,
) -> Result<TrainFormationDetail, CommandError> {
    info!("Getting train formation {}", id);

    let mut uow = state.unit_of_work().await?;
    let result = GetTrainFormationUseCase::execute(&mut uow, id).await?;
    uow.commit().await?;
    Ok(result)
}

pub async fn get_train_formations_inner(
    state: &AppState,
) -> Result<Vec<TrainFormationSummary>, CommandError> {
    info!("Listing train formations");

    let mut uow = state.unit_of_work().await?;
    let result = GetTrainFormationsUseCase::execute(&mut uow).await?;
    uow.commit().await?;
    Ok(result)
}

// ── Element composition ───────────────────────────────────────────────────────

pub async fn add_formation_element_inner(
    state: &AppState,
    formation_id: String,
    args: AddFormationElementArgs,
) -> Result<FormationElementView, CommandError> {
    info!("Adding element to formation {}: {:?}", formation_id, args);

    let mut uow = state.unit_of_work().await?;
    let result = AddFormationElementUseCase::execute(
        &mut uow,
        formation_id,
        args.prototype_id,
        args.owned_rolling_stock_id,
    )
    .await?;
    uow.commit().await?;
    Ok(result)
}

pub async fn remove_formation_element_inner(
    state: &AppState,
    element_id: String,
) -> Result<(), CommandError> {
    info!("Removing formation element {}", element_id);

    let mut uow = state.unit_of_work().await?;
    RemoveFormationElementUseCase::execute(&mut uow, element_id).await?;
    uow.commit().await?;
    Ok(())
}

pub async fn reorder_formation_elements_inner(
    state: &AppState,
    formation_id: String,
    args: ReorderFormationElementsArgs,
) -> Result<TrainFormationDetail, CommandError> {
    info!(
        "Reordering elements in formation {}: {:?}",
        formation_id, args
    );

    let mut uow = state.unit_of_work().await?;
    let result =
        ReorderFormationElementsUseCase::execute(&mut uow, formation_id, args.element_ids).await?;
    uow.commit().await?;
    Ok(result)
}

// ── Ownership ─────────────────────────────────────────────────────────────────

pub async fn assign_rolling_stock_to_element_inner(
    state: &AppState,
    element_id: String,
    args: AssignRollingStockToElementArgs,
) -> Result<FormationElementView, CommandError> {
    info!(
        "Assigning rolling stock to element {}: {:?}",
        element_id, args
    );

    let mut uow = state.unit_of_work().await?;
    let result = AssignRollingStockToElementUseCase::execute(
        &mut uow,
        element_id,
        args.owned_rolling_stock_id,
    )
    .await?;
    uow.commit().await?;
    Ok(result)
}

// ── Traction ──────────────────────────────────────────────────────────────────

pub async fn set_traction_override_inner(
    state: &AppState,
    element_id: String,
    args: SetTractionOverrideArgs,
) -> Result<FormationElementView, CommandError> {
    info!(
        "Setting traction override for element {}: {:?}",
        element_id, args
    );

    let mut uow = state.unit_of_work().await?;
    let result =
        SetTractionOverrideUseCase::execute(&mut uow, element_id, args.traction_override).await?;
    uow.commit().await?;
    Ok(result)
}

// ── Prototypes ────────────────────────────────────────────────────────────────

pub async fn get_prototypes_inner(
    state: &AppState,
    query: Option<String>,
) -> Result<Vec<PrototypeGroupView>, CommandError> {
    info!("Searching prototypes with query: {:?}", query);

    let mut uow = state.unit_of_work().await?;
    let result = GetPrototypesUseCase::execute(&mut uow, query).await?;
    uow.commit().await?;
    Ok(result)
}

pub async fn create_custom_prototype_inner(
    state: &AppState,
    args: CreateCustomPrototypeArgs,
) -> Result<PrototypeView, CommandError> {
    info!("Creating custom prototype: {:?}", args);

    let mut uow = state.unit_of_work().await?;
    let result = CreateCustomPrototypeUseCase::execute(
        &mut uow,
        args.railway_company_id,
        args.series_code,
        args.friendly_name,
        args.is_motorized,
        args.default_is_dummy,
        args.notes,
        args.specification_type,
        args.locomotive_type,
        args.locomotive_series,
        args.service_level,
        args.passenger_car_type,
        args.freight_car_type,
        args.railcar_type,
        args.electric_multiple_unit_type,
        args.elements_count,
        args.is_permanently_coupled,
    )
    .await?;
    uow.commit().await?;
    Ok(result)
}

// ── Categories ────────────────────────────────────────────────────────────────

pub async fn get_formation_categories_inner(
    state: &AppState,
) -> Result<Vec<FormationCategoryView>, CommandError> {
    info!("Getting formation categories");

    let mut uow = state.unit_of_work().await?;
    let result = GetFormationCategoriesUseCase::execute(&mut uow).await?;
    uow.commit().await?;
    Ok(result)
}

pub async fn create_formation_category_inner(
    state: &AppState,
    args: CreateFormationCategoryArgs,
) -> Result<FormationCategoryView, CommandError> {
    info!("Creating formation category: {:?}", args);

    let mut uow = state.unit_of_work().await?;
    let result = CreateFormationCategoryUseCase::execute(&mut uow, args.name).await?;
    uow.commit().await?;
    Ok(result)
}

// ── Tauri command shims ───────────────────────────────────────────────────────

/// Create a new train formation.
#[tauri::command]
#[specta::specta]
pub async fn create_train_formation(
    state: tauri::State<'_, AppState>,
    args: CreateTrainFormationArgs,
) -> Result<TrainFormationView, CommandError> {
    args.validate()
        .map_err(|e| CommandError::BusinessRule(format!("Validation failed: {e}")))?;
    create_train_formation_inner(&state, args).await
}

/// Update the metadata of an existing train formation.
#[tauri::command]
#[specta::specta]
pub async fn update_train_formation(
    state: tauri::State<'_, AppState>,
    id: String,
    args: UpdateTrainFormationArgs,
) -> Result<TrainFormationView, CommandError> {
    args.validate()
        .map_err(|e| CommandError::BusinessRule(format!("Validation failed: {e}")))?;
    update_train_formation_inner(&state, id, args).await
}

/// Delete a train formation by ID.
#[tauri::command]
#[specta::specta]
pub async fn delete_train_formation(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<(), CommandError> {
    delete_train_formation_inner(&state, id).await
}

/// Get a single train formation with full element detail.
#[tauri::command]
#[specta::specta]
pub async fn get_train_formation(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<TrainFormationDetail, CommandError> {
    get_train_formation_inner(&state, id).await
}

/// List all train formations as summaries.
#[tauri::command]
#[specta::specta]
pub async fn get_train_formations(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<TrainFormationSummary>, CommandError> {
    get_train_formations_inner(&state).await
}

/// Add a prototype element to a train formation.
#[tauri::command]
#[specta::specta]
pub async fn add_formation_element(
    state: tauri::State<'_, AppState>,
    formation_id: String,
    args: AddFormationElementArgs,
) -> Result<FormationElementView, CommandError> {
    args.validate()
        .map_err(|e| CommandError::BusinessRule(format!("Validation failed: {e}")))?;
    add_formation_element_inner(&state, formation_id, args).await
}

/// Remove an element from a train formation.
#[tauri::command]
#[specta::specta]
pub async fn remove_formation_element(
    state: tauri::State<'_, AppState>,
    element_id: String,
) -> Result<(), CommandError> {
    remove_formation_element_inner(&state, element_id).await
}

/// Reorder the elements within a train formation.
#[tauri::command]
#[specta::specta]
pub async fn reorder_formation_elements(
    state: tauri::State<'_, AppState>,
    formation_id: String,
    args: ReorderFormationElementsArgs,
) -> Result<TrainFormationDetail, CommandError> {
    args.validate()
        .map_err(|e| CommandError::BusinessRule(format!("Validation failed: {e}")))?;
    reorder_formation_elements_inner(&state, formation_id, args).await
}

/// Assign or unassign an owned rolling stock to a formation element.
#[tauri::command]
#[specta::specta]
pub async fn assign_rolling_stock_to_element(
    state: tauri::State<'_, AppState>,
    element_id: String,
    args: AssignRollingStockToElementArgs,
) -> Result<FormationElementView, CommandError> {
    assign_rolling_stock_to_element_inner(&state, element_id, args).await
}

/// Override the traction status of a formation element.
#[tauri::command]
#[specta::specta]
pub async fn set_traction_override(
    state: tauri::State<'_, AppState>,
    element_id: String,
    args: SetTractionOverrideArgs,
) -> Result<FormationElementView, CommandError> {
    args.validate()
        .map_err(|e| CommandError::BusinessRule(format!("Validation failed: {e}")))?;
    set_traction_override_inner(&state, element_id, args).await
}

/// Search prototypes grouped by railway company.
#[tauri::command]
#[specta::specta]
pub async fn get_prototypes(
    state: tauri::State<'_, AppState>,
    query: Option<String>,
) -> Result<Vec<PrototypeGroupView>, CommandError> {
    get_prototypes_inner(&state, query).await
}

/// Create a new custom prototype.
#[tauri::command]
#[specta::specta]
pub async fn create_custom_prototype(
    state: tauri::State<'_, AppState>,
    args: CreateCustomPrototypeArgs,
) -> Result<PrototypeView, CommandError> {
    args.validate()
        .map_err(|e| CommandError::BusinessRule(format!("Validation failed: {e}")))?;
    create_custom_prototype_inner(&state, args).await
}

/// List all formation categories.
#[tauri::command]
#[specta::specta]
pub async fn get_formation_categories(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<FormationCategoryView>, CommandError> {
    get_formation_categories_inner(&state).await
}

/// Create a new custom formation category.
#[tauri::command]
#[specta::specta]
pub async fn create_formation_category(
    state: tauri::State<'_, AppState>,
    args: CreateFormationCategoryArgs,
) -> Result<FormationCategoryView, CommandError> {
    args.validate()
        .map_err(|e| CommandError::BusinessRule(format!("Validation failed: {e}")))?;
    create_formation_category_inner(&state, args).await
}
