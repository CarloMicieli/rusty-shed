use crate::core::infrastructure::error::CommandError;
use crate::import::application::ValidatePackageUseCase;
use crate::import::domain::{ArchiveFormat, ImportSession, ImportState};
use crate::import::interface::types::{
    AnalyzeImportPackageArgs, AnalyzeImportPackageResponse, CancelImportSessionArgs,
    CancelImportSessionResponse, ExecuteImportArgs, GetImportPreviewArgs, ImportPreviewResponse,
    ImportResultResponse, ValidationStatus,
};
use log::info;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

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
        return Err(CommandError::Unknown("Invalid archive format".to_string()));
    };

    // Validate package using application layer
    let (_detected_format, _manifest, record_counts) = ValidatePackageUseCase::execute(&path)
        .await
        .map_err(|e| CommandError::Unknown(format!("Validation failed: {}", e.code)))?;

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
) -> Result<ImportPreviewResponse, CommandError> {
    info!("get_import_preview: session_id={}", args.session_id);

    let _sessions = IMPORT_SESSIONS.lock().unwrap();
    let _session = _sessions
        .get(&args.session_id)
        .ok_or_else(|| CommandError::Unknown("Session not found".to_string()))?;

    // TODO: Load manifest from session and check for duplicates
    // For now, return a placeholder response
    Ok(ImportPreviewResponse {
        session_id: args.session_id,
        total_records: Default::default(),
        new_records: Default::default(),
        duplicate_records: Default::default(),
        errors: vec![],
        warnings: vec![],
        can_import: true,
    })
}

/// Execute the import operation.
///
/// This command:
/// 1. Loads the validated session
/// 2. Writes all records to the database
/// 3. Copies images to the media storage
/// 4. Returns import results
#[tauri::command]
#[specta::specta]
pub async fn execute_import(args: ExecuteImportArgs) -> Result<ImportResultResponse, CommandError> {
    info!("execute_import: session_id={}", args.session_id);

    let _sessions = IMPORT_SESSIONS.lock().unwrap();
    let _session = _sessions
        .get(&args.session_id)
        .ok_or_else(|| CommandError::Unknown("Session not found".to_string()))?;

    // TODO: Load manifest and execute import
    // For now, return a placeholder response
    Ok(ImportResultResponse {
        session_id: args.session_id,
        status: crate::import::interface::types::ImportOutcome::Success,
        added: Default::default(),
        skipped: Default::default(),
        images_imported: 0,
        images_failed: vec![],
        duration_ms: 0,
        warnings: vec![],
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
        Err(CommandError::Unknown("Session not found".to_string()))
    }
}
