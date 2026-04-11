//! Tauri command handlers for the train-formations feature.

use crate::core::infrastructure::error::CommandError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
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
use crate::trains::infrastructure::mappers::{
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

/// Helper to open a `SqliteUnitOfWork` directly from the app state pool.
async fn open_uow(state: &AppState) -> Result<SqliteUnitOfWork, CommandError> {
    SqliteUnitOfWork::new(&state.db_pool())
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))
}

// ── Formation CRUD ────────────────────────────────────────────────────────────

/// Create a new train formation.
#[tauri::command]
#[specta::specta]
pub async fn create_train_formation(
    state: tauri::State<'_, AppState>,
    args: CreateTrainFormationArgs,
) -> Result<TrainFormationView, CommandError> {
    info!("Creating train formation: {:?}", args);
    args.validate()
        .map_err(|e| CommandError::BusinessRule(format!("Validation failed: {e}")))?;

    let mut uow = open_uow(&state).await?;
    let result = CreateTrainFormationUseCase::execute(&mut uow, args).await?;
    uow.commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;
    Ok(result)
}

/// Update the metadata of an existing train formation.
#[tauri::command]
#[specta::specta]
pub async fn update_train_formation(
    state: tauri::State<'_, AppState>,
    id: String,
    args: UpdateTrainFormationArgs,
) -> Result<TrainFormationView, CommandError> {
    info!("Updating train formation {}: {:?}", id, args);
    args.validate()
        .map_err(|e| CommandError::BusinessRule(format!("Validation failed: {e}")))?;

    let mut uow = open_uow(&state).await?;
    let result = UpdateTrainFormationUseCase::execute(&mut uow, id, args).await?;
    uow.commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;
    Ok(result)
}

/// Delete a train formation by ID.
#[tauri::command]
#[specta::specta]
pub async fn delete_train_formation(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<(), CommandError> {
    info!("Deleting train formation {}", id);

    let mut uow = open_uow(&state).await?;
    DeleteTrainFormationUseCase::execute(&mut uow, id).await?;
    uow.commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;
    Ok(())
}

/// Get a single train formation with full element detail.
#[tauri::command]
#[specta::specta]
pub async fn get_train_formation(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<TrainFormationDetail, CommandError> {
    info!("Getting train formation {}", id);

    let mut uow = open_uow(&state).await?;
    let result = GetTrainFormationUseCase::execute(&mut uow, id).await?;
    Ok(result)
}

/// List all train formations as summaries.
#[tauri::command]
#[specta::specta]
pub async fn get_train_formations(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<TrainFormationSummary>, CommandError> {
    info!("Listing train formations");

    let mut uow = open_uow(&state).await?;
    let result = GetTrainFormationsUseCase::execute(&mut uow).await?;
    Ok(result)
}

// ── Element composition ───────────────────────────────────────────────────────

/// Add a prototype element to a train formation.
#[tauri::command]
#[specta::specta]
pub async fn add_formation_element(
    state: tauri::State<'_, AppState>,
    formation_id: String,
    args: AddFormationElementArgs,
) -> Result<FormationElementView, CommandError> {
    info!("Adding element to formation {}: {:?}", formation_id, args);
    args.validate()
        .map_err(|e| CommandError::BusinessRule(format!("Validation failed: {e}")))?;

    let mut uow = open_uow(&state).await?;
    let result = AddFormationElementUseCase::execute(&mut uow, formation_id, args).await?;
    uow.commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;
    Ok(result)
}

/// Remove an element from a train formation.
#[tauri::command]
#[specta::specta]
pub async fn remove_formation_element(
    state: tauri::State<'_, AppState>,
    element_id: String,
) -> Result<(), CommandError> {
    info!("Removing formation element {}", element_id);

    let mut uow = open_uow(&state).await?;
    RemoveFormationElementUseCase::execute(&mut uow, element_id).await?;
    uow.commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;
    Ok(())
}

/// Reorder the elements within a train formation.
#[tauri::command]
#[specta::specta]
pub async fn reorder_formation_elements(
    state: tauri::State<'_, AppState>,
    formation_id: String,
    args: ReorderFormationElementsArgs,
) -> Result<TrainFormationDetail, CommandError> {
    info!(
        "Reordering elements in formation {}: {:?}",
        formation_id, args
    );
    args.validate()
        .map_err(|e| CommandError::BusinessRule(format!("Validation failed: {e}")))?;

    let mut uow = open_uow(&state).await?;
    let result = ReorderFormationElementsUseCase::execute(&mut uow, formation_id, args).await?;
    uow.commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;
    Ok(result)
}

// ── Ownership ─────────────────────────────────────────────────────────────────

/// Assign or unassign an owned rolling stock to a formation element.
#[tauri::command]
#[specta::specta]
pub async fn assign_rolling_stock_to_element(
    state: tauri::State<'_, AppState>,
    element_id: String,
    args: AssignRollingStockToElementArgs,
) -> Result<FormationElementView, CommandError> {
    info!(
        "Assigning rolling stock to element {}: {:?}",
        element_id, args
    );

    let mut uow = open_uow(&state).await?;
    let result = AssignRollingStockToElementUseCase::execute(&mut uow, element_id, args).await?;
    uow.commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;
    Ok(result)
}

// ── Traction ──────────────────────────────────────────────────────────────────

/// Override the traction status of a formation element.
#[tauri::command]
#[specta::specta]
pub async fn set_traction_override(
    state: tauri::State<'_, AppState>,
    element_id: String,
    args: SetTractionOverrideArgs,
) -> Result<FormationElementView, CommandError> {
    info!(
        "Setting traction override for element {}: {:?}",
        element_id, args
    );
    args.validate()
        .map_err(|e| CommandError::BusinessRule(format!("Validation failed: {e}")))?;

    let mut uow = open_uow(&state).await?;
    let result = SetTractionOverrideUseCase::execute(&mut uow, element_id, args).await?;
    uow.commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;
    Ok(result)
}

// ── Prototypes ────────────────────────────────────────────────────────────────

/// Search prototypes grouped by railway company.
#[tauri::command]
#[specta::specta]
pub async fn get_prototypes(
    state: tauri::State<'_, AppState>,
    query: Option<String>,
) -> Result<Vec<PrototypeGroupView>, CommandError> {
    info!("Searching prototypes with query: {:?}", query);

    let mut uow = open_uow(&state).await?;
    let result = GetPrototypesUseCase::execute(&mut uow, query).await?;
    Ok(result)
}

/// Create a new custom prototype.
#[tauri::command]
#[specta::specta]
pub async fn create_custom_prototype(
    state: tauri::State<'_, AppState>,
    args: CreateCustomPrototypeArgs,
) -> Result<PrototypeView, CommandError> {
    info!("Creating custom prototype: {:?}", args);
    args.validate()
        .map_err(|e| CommandError::BusinessRule(format!("Validation failed: {e}")))?;

    let mut uow = open_uow(&state).await?;
    let result = CreateCustomPrototypeUseCase::execute(&mut uow, args).await?;
    uow.commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;
    Ok(result)
}

// ── Categories ────────────────────────────────────────────────────────────────

/// List all formation categories.
#[tauri::command]
#[specta::specta]
pub async fn get_formation_categories(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<FormationCategoryView>, CommandError> {
    info!("Getting formation categories");

    let mut uow = open_uow(&state).await?;
    let result = GetFormationCategoriesUseCase::execute(&mut uow).await?;
    Ok(result)
}

/// Create a new custom formation category.
#[tauri::command]
#[specta::specta]
pub async fn create_formation_category(
    state: tauri::State<'_, AppState>,
    args: CreateFormationCategoryArgs,
) -> Result<FormationCategoryView, CommandError> {
    info!("Creating formation category: {:?}", args);
    args.validate()
        .map_err(|e| CommandError::BusinessRule(format!("Validation failed: {e}")))?;

    let mut uow = open_uow(&state).await?;
    let result = CreateFormationCategoryUseCase::execute(&mut uow, args).await?;
    uow.commit()
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;
    Ok(result)
}
