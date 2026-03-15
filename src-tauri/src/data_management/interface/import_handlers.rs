use crate::core::infrastructure::error::CommandError;
use crate::data_management::application::{
    ExecuteImportUseCase, PreviewImportUseCase, ValidatePackageUseCase,
};
use crate::data_management::domain::{ArchiveFormat, ImportSession, ImportState, ManifestDto};
use crate::data_management::infrastructure::ArchiveExtractor;
use crate::data_management::interface::types::{
    AnalyzeImportPackageArgs, AnalyzeImportPackageResponse, CancelImportSessionArgs,
    CancelImportSessionResponse, ExecuteImportArgs, GetImportPreviewArgs, ImageFailureDto,
    ImportOutcome, ImportPreviewResponse, ImportResultResponse, ValidationStatus,
};
use crate::state::AppState;
use log::info;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;

/// In-memory session storage (MVP - replace with DB in production)
static IMPORT_SESSIONS: once_cell::sync::Lazy<Mutex<HashMap<String, ImportSession>>> =
    once_cell::sync::Lazy::new(|| Mutex::new(HashMap::new()));

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
    let (_detected_format, _manifest, record_counts) =
        ValidatePackageUseCase::execute(&path).await.map_err(|e| {
            CommandError::unknown(format!("Validation failed [{}]: {}", e.code, e.message))
        })?;

    // Create session
    let session = ImportSession::new(path, format);
    let session_id = session.id.clone();

    // Store session
    {
        let mut sessions = IMPORT_SESSIONS.lock().unwrap();
        sessions.insert(session_id.clone(), session);
    }

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
    info!("get_import_preview: session_id={}", args.session_id);

    // Get session
    let session = {
        let sessions = IMPORT_SESSIONS.lock().unwrap();
        sessions
            .get(&args.session_id)
            .cloned()
            .ok_or_else(|| CommandError::unknown("Session not found".to_string()))?
    };

    // Extract manifest from archive
    let manifest_bytes = ArchiveExtractor::extract_manifest(&session.source_path)
        .map_err(|e| CommandError::unknown(format!("Failed to extract manifest: {}", e)))?;

    let manifest_json: serde_json::Value = serde_json::from_slice(&manifest_bytes)
        .map_err(|e| CommandError::unknown(format!("Failed to parse manifest: {}", e)))?;

    // Generate preview using application layer
    let preview_use_case = PreviewImportUseCase::new(state.db_pool());
    let preview = preview_use_case
        .execute(manifest_json, Some(&session.source_path))
        .await
        .map_err(|e| CommandError::unknown(format!("Preview generation failed: {}", e)))?;

    // Update session state
    {
        let mut sessions = IMPORT_SESSIONS.lock().unwrap();
        if let Some(session) = sessions.get_mut(&args.session_id) {
            session.transition(ImportState::Previewed);
        }
    }

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
    info!("execute_import: session_id={}", args.session_id);

    // Get session
    let session = {
        let sessions = IMPORT_SESSIONS.lock().unwrap();
        sessions
            .get(&args.session_id)
            .cloned()
            .ok_or_else(|| CommandError::unknown("Session not found".to_string()))?
    };

    // Extract and parse manifest from archive
    let manifest_bytes = ArchiveExtractor::extract_manifest(&session.source_path)
        .map_err(|e| CommandError::unknown(format!("Failed to extract manifest: {}", e)))?;

    let manifest_content = String::from_utf8(manifest_bytes)
        .map_err(|e| CommandError::unknown(format!("Invalid UTF-8 in manifest: {}", e)))?;

    let manifest: ManifestDto = serde_json::from_str(&manifest_content)
        .map_err(|e| CommandError::unknown(format!("Failed to parse manifest: {}", e)))?;

    // Execute import
    let use_case = ExecuteImportUseCase::new(state.db_pool());
    let media_dir = state.models_dir();
    let result = use_case
        .execute(&session, &manifest, &session.source_path, &media_dir)
        .await
        .map_err(|e| CommandError::unknown(e.to_string()))?;

    // Update session state
    {
        let mut sessions = IMPORT_SESSIONS.lock().unwrap();
        if let Some(session) = sessions.get_mut(&args.session_id) {
            session.transition(ImportState::Completed);
        }
    }

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
) -> Result<CancelImportSessionResponse, CommandError> {
    info!("cancel_import_session: session_id={}", args.session_id);

    let mut sessions = IMPORT_SESSIONS.lock().unwrap();
    if let Some(mut session) = sessions.remove(&args.session_id) {
        session.transition(ImportState::Failed {
            reason: "Cancelled by user".to_string(),
        });
        Ok(CancelImportSessionResponse {
            session_id: args.session_id,
            cancelled: true,
        })
    } else {
        Err(CommandError::unknown("Session not found".to_string()))
    }
}

/// Check if an import session is currently in progress.
///
/// Returns `true` if any session is in the `Importing` state,
/// `false` otherwise.
pub fn is_import_in_progress() -> bool {
    let sessions = IMPORT_SESSIONS.lock().unwrap();
    sessions
        .values()
        .any(|session| session.state == ImportState::Importing)
}
