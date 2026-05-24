use crate::core::infrastructure::error::CommandError;
use crate::data_management::application::{
    ExecuteImportUseCase, PreviewImportUseCase, ValidatePackageUseCase,
};
use crate::data_management::domain::{ArchiveFormat, ImportSession, ImportState, ManifestDto};
use crate::data_management::infrastructure::ArchiveExtractor;
use crate::data_management::infrastructure::SqliteImportRepository;
use crate::data_management::interface::types::{
    AnalyzeImportPackageArgs, AnalyzeImportPackageResponse, CancelImportSessionArgs,
    CancelImportSessionResponse, ExecuteImportArgs, GetImportPreviewArgs, ImageFailureDto,
    ImportOutcome, ImportPreviewResponse, ImportResultResponse, ValidationStatus,
};
use crate::state::AppState;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::State;
use tracing::info;

// ---------------------------------------------------------------------------
// Inner (testable) implementations – take &AppState directly
// ---------------------------------------------------------------------------

pub async fn analyze_import_package_inner(
    state: &AppState,
    args: AnalyzeImportPackageArgs,
) -> Result<AnalyzeImportPackageResponse, CommandError> {
    info!("analyze_import_package: {}", args.file_path);

    let path = PathBuf::from(&args.file_path);

    // Detect archive format
    let format = if args.file_path.ends_with(".zip") {
        ArchiveFormat::Zip
    } else if args.file_path.ends_with(".tar.gz") || args.file_path.ends_with(".tgz") {
        ArchiveFormat::TarGz
    } else {
        return Err(CommandError::unknown("Invalid archive format".to_string()));
    };

    // Validate package using application layer
    let (_detected_format, manifest, record_counts) =
        ValidatePackageUseCase::execute(&path).await.map_err(|e| {
            CommandError::unknown(format!("Validation failed [{}]: {}", e.code, e.message))
        })?;

    // Create and store session — cache the parsed manifest to avoid re-extracting later
    let mut session = ImportSession::new(path, format);
    session.validated_manifest = Some(manifest);
    let session_id = session.id.clone();
    state.import_session_store.insert(session).await;

    // Extract image filenames from archive
    let images_found = vec![]; // TODO: Extract from archive

    Ok(AnalyzeImportPackageResponse {
        session_id,
        format,
        manifest_found: true,
        validation_status: ValidationStatus::Valid,
        record_counts,
        images_found,
    })
}

/// Analyze an import package archive.
///
/// This command:
/// 1. Validates the archive format
/// 2. Extracts and validates the manifest
/// 3. Creates an import session
/// 4. Returns validation results and record counts
#[tauri::command]
#[specta::specta]
pub async fn analyze_import_package(
    args: AnalyzeImportPackageArgs,
    state: State<'_, AppState>,
) -> Result<AnalyzeImportPackageResponse, CommandError> {
    analyze_import_package_inner(&state, args).await
}

pub async fn get_import_preview_inner(
    state: &AppState,
    args: GetImportPreviewArgs,
) -> Result<ImportPreviewResponse, CommandError> {
    info!("get_import_preview: session_id={}", args.session_id);

    // Get session
    let session = state
        .import_session_store
        .get(&args.session_id)
        .await
        .ok_or_else(|| CommandError::unknown("Session not found".to_string()))?;

    // Use cached manifest if available, otherwise fall back to archive extraction.
    // When the manifest is already cached, preview does not need archive I/O.
    let (manifest_json, source_path) = if let Some(ref manifest) = session.validated_manifest {
        let value = serde_json::to_value(manifest).map_err(|e| {
            CommandError::unknown(format!("Failed to serialize cached manifest: {}", e))
        })?;
        (value, None)
    } else {
        let manifest_bytes = ArchiveExtractor::extract_manifest_async(session.source_path.clone())
            .await
            .map_err(|e| CommandError::unknown(format!("Failed to extract manifest: {}", e)))?;
        let value = serde_json::from_slice(&manifest_bytes)
            .map_err(|e| CommandError::unknown(format!("Failed to parse manifest: {}", e)))?;
        (value, Some(&session.source_path))
    };

    // Generate preview using application layer
    let repo = Arc::new(SqliteImportRepository::new(state.db_pool()));
    let preview_use_case = PreviewImportUseCase::new(repo)
        .map_err(|e| CommandError::unknown(format!("Failed to initialize preview: {}", e)))?;
    let preview = preview_use_case
        .execute(manifest_json, source_path.map(|p| p.as_path()))
        .await
        .map_err(|e| CommandError::unknown(format!("Preview generation failed: {}", e)))?;

    // Update session state
    state
        .import_session_store
        .update(&args.session_id, |s| s.transition(ImportState::Previewed))
        .await;

    // Calculate new records (total - duplicates)
    let new_records = crate::data_management::domain::RecordCounts {
        manufacturers: preview
            .total_records
            .manufacturers
            .saturating_sub(preview.duplicate_records.manufacturers),
        railway_companies: preview
            .total_records
            .railway_companies
            .saturating_sub(preview.duplicate_records.railway_companies),
        railway_models: preview
            .total_records
            .railway_models
            .saturating_sub(preview.duplicate_records.railway_models),
        collection_items: preview
            .total_records
            .collection_items
            .saturating_sub(preview.duplicate_records.collection_items),
        owned_rolling_stocks: preview
            .total_records
            .owned_rolling_stocks
            .saturating_sub(preview.duplicate_records.owned_rolling_stocks),
        sellers: preview
            .total_records
            .sellers
            .saturating_sub(preview.duplicate_records.sellers),
        maintenance_cards: preview
            .total_records
            .maintenance_cards
            .saturating_sub(preview.duplicate_records.maintenance_cards),
        track_products: preview
            .total_records
            .track_products
            .saturating_sub(preview.duplicate_records.track_products),
        track_inventories: preview
            .total_records
            .track_inventories
            .saturating_sub(preview.duplicate_records.track_inventories),
        prototypes: preview
            .total_records
            .prototypes
            .saturating_sub(preview.duplicate_records.prototypes),
        formation_categories: preview
            .total_records
            .formation_categories
            .saturating_sub(preview.duplicate_records.formation_categories),
        train_formations: preview
            .total_records
            .train_formations
            .saturating_sub(preview.duplicate_records.train_formations),
        wishlists: preview
            .total_records
            .wishlists
            .saturating_sub(preview.duplicate_records.wishlists),
        decoders: preview
            .total_records
            .decoders
            .saturating_sub(preview.duplicate_records.decoders),
        digital_rolling_stocks: preview
            .total_records
            .digital_rolling_stocks
            .saturating_sub(preview.duplicate_records.digital_rolling_stocks),
    };

    let can_import = preview.can_import();

    Ok(ImportPreviewResponse {
        session_id: args.session_id,
        total_records: preview.total_records,
        new_records,
        duplicate_records: preview.duplicate_records,
        duplicate_details: preview.duplicate_details,
        errors: preview.errors,
        warnings: preview.warnings,
        can_import,
    })
}

/// Get a preview of the import before execution.
///
/// This command:
/// 1. Loads the existing session
/// 2. Checks for duplicate records
/// 3. Returns record counts and any validation errors
#[tauri::command]
#[specta::specta]
pub async fn get_import_preview(
    args: GetImportPreviewArgs,
    state: State<'_, AppState>,
) -> Result<ImportPreviewResponse, CommandError> {
    get_import_preview_inner(&state, args).await
}

pub async fn execute_import_inner(
    state: &AppState,
    args: ExecuteImportArgs,
) -> Result<ImportResultResponse, CommandError> {
    info!("execute_import: session_id={}", args.session_id);

    // Guard against concurrent imports
    if is_import_in_progress(&state.import_session_store).await {
        return Err(CommandError::unknown(
            "Another import is already in progress".to_string(),
        ));
    }

    // Get session
    let session = state
        .import_session_store
        .get(&args.session_id)
        .await
        .ok_or_else(|| CommandError::unknown("Session not found".to_string()))?;

    // Use cached manifest if available, otherwise fall back to archive extraction
    let manifest: ManifestDto = if let Some(manifest) = session.validated_manifest.clone() {
        manifest
    } else {
        let manifest_bytes = ArchiveExtractor::extract_manifest_async(session.source_path.clone())
            .await
            .map_err(|e| CommandError::unknown(format!("Failed to extract manifest: {}", e)))?;
        let manifest_content = String::from_utf8(manifest_bytes)
            .map_err(|e| CommandError::unknown(format!("Invalid UTF-8 in manifest: {}", e)))?;
        serde_json::from_str(&manifest_content)
            .map_err(|e| CommandError::unknown(format!("Failed to parse manifest: {}", e)))?
    };

    // Execute import
    let repo = Arc::new(SqliteImportRepository::new(state.db_pool()));
    let use_case = ExecuteImportUseCase::new(repo);
    let media_dir = state.models_dir();
    let result = use_case
        .execute(&session, &manifest, &session.source_path, media_dir)
        .await
        .map_err(CommandError::from)?;

    // Update session state
    state
        .import_session_store
        .update(&args.session_id, |s| s.transition(ImportState::Completed))
        .await;

    Ok(ImportResultResponse {
        session_id: result.session_id,
        status: ImportOutcome::Success,
        added: result.added,
        skipped: result.skipped,
        images_imported: result.images_imported,
        images_failed: result
            .images_failed
            .into_iter()
            .map(|f| ImageFailureDto {
                filename: f.filename,
                reason: f.reason,
            })
            .collect(),
        duration_ms: result.duration_ms,
        warnings: result.warnings,
    })
}

/// Execute the import operation.
///
/// This command:
/// 1. Loads the validated session and manifest
/// 2. Checks for duplicates and skips them
/// 3. Writes new records to the database
/// 4. Returns import results with added/skipped counts
#[tauri::command]
#[specta::specta]
pub async fn execute_import(
    args: ExecuteImportArgs,
    state: State<'_, AppState>,
) -> Result<ImportResultResponse, CommandError> {
    execute_import_inner(&state, args).await
}

pub async fn cancel_import_session_inner(
    state: &AppState,
    args: CancelImportSessionArgs,
) -> Result<CancelImportSessionResponse, CommandError> {
    info!("cancel_import_session: session_id={}", args.session_id);

    if state
        .import_session_store
        .remove(&args.session_id)
        .await
        .is_some()
    {
        Ok(CancelImportSessionResponse {
            session_id: args.session_id,
            cancelled: true,
        })
    } else {
        Err(CommandError::unknown("Session not found".to_string()))
    }
}

/// Cancel an import session.
///
/// This command:
/// 1. Finds the session
/// 2. Transitions it to failed state
/// 3. Cleans up temporary files
#[tauri::command]
#[specta::specta]
pub async fn cancel_import_session(
    args: CancelImportSessionArgs,
    state: State<'_, AppState>,
) -> Result<CancelImportSessionResponse, CommandError> {
    cancel_import_session_inner(&state, args).await
}

/// Check if an import session is currently in progress.
///
/// Returns `true` if any session is in the `Importing` state,
/// `false` otherwise.
pub async fn is_import_in_progress(
    store: &crate::data_management::application::ImportSessionStore,
) -> bool {
    store
        .any(|session| session.state == ImportState::Importing)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_management::domain::{
        ArchiveFormat, DataContainerDto, ImportSession, ImportState, ManifestDto,
    };
    use sqlx::SqlitePool;
    use std::path::PathBuf;

    fn app_state(pool: SqlitePool) -> AppState {
        AppState::for_test(pool)
    }

    #[sqlx::test]
    async fn get_import_preview_missing_session_returns_unknown(pool: SqlitePool) {
        let state = app_state(pool);
        let args = GetImportPreviewArgs {
            session_id: "missing-session".to_string(),
        };

        let result = get_import_preview_inner(&state, args).await;
        assert!(matches!(result, Err(CommandError::Unknown { .. })));
    }

    #[sqlx::test]
    async fn execute_import_missing_session_returns_unknown(pool: SqlitePool) {
        let state = app_state(pool);
        let args = ExecuteImportArgs {
            session_id: "missing-session".to_string(),
        };

        let result = execute_import_inner(&state, args).await;
        assert!(matches!(result, Err(CommandError::Unknown { .. })));
    }

    #[sqlx::test]
    async fn get_import_preview_session_with_missing_archive_returns_unknown(pool: SqlitePool) {
        let state = app_state(pool);
        let session = ImportSession::new(
            PathBuf::from("/tmp/non-existent-import-archive.zip"),
            ArchiveFormat::Zip,
        );
        let session_id = session.id.clone();
        state.import_session_store.insert(session).await;

        let args = GetImportPreviewArgs { session_id };
        let result = get_import_preview_inner(&state, args).await;

        assert!(matches!(result, Err(CommandError::Unknown { .. })));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn get_import_preview_with_cached_manifest_returns_preview_and_updates_state(
        pool: SqlitePool,
    ) {
        let state = app_state(pool);

        let mut session = ImportSession::new(
            PathBuf::from("/tmp/unused-cached-manifest.zip"),
            ArchiveFormat::Zip,
        );
        let session_id = session.id.clone();
        session.validated_manifest = Some(ManifestDto {
            schema: None,
            version: "1.0".to_string(),
            exported_at: None,
            source: None,
            data: DataContainerDto::default(),
        });
        state.import_session_store.insert(session).await;

        let args = GetImportPreviewArgs {
            session_id: session_id.clone(),
        };
        let preview = get_import_preview_inner(&state, args)
            .await
            .expect("preview should succeed for cached empty manifest");

        assert_eq!(preview.session_id, session_id);
        assert!(preview.can_import);
        assert_eq!(preview.total_records.manufacturers, 0);
        assert_eq!(preview.new_records.manufacturers, 0);
        assert_eq!(preview.duplicate_records.manufacturers, 0);

        let stored = state
            .import_session_store
            .get(&preview.session_id)
            .await
            .expect("session should remain in store");
        assert_eq!(stored.state, ImportState::Previewed);
    }
}
