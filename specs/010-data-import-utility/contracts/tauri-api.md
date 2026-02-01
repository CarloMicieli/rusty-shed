# Tauri IPC API Contract: Import Utility

**Feature**: 010-data-import-utility  
**Created**: January 30, 2026  
**Version**: 1.0

---

## Overview

This document defines the Tauri IPC commands exposed by the import utility feature. All commands follow ADR-008 conventions with specta-generated TypeScript bindings.

---

## Commands

### 1. `analyze_import_package`

Analyzes an import package file, extracts the manifest, and performs initial validation.

**Command Name**: `analyze_import_package`

**Args**:

```rust
#[derive(Debug, Clone, Deserialize, specta::Type, garde::Validate)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzeImportPackageArgs {
    /// Absolute path to the import package file (.zip or .tar.gz)
    #[garde(length(min = 1))]
    pub file_path: String,
}
```

**Response**:

```rust
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzeImportPackageResponse {
    /// Unique session ID for subsequent operations
    pub session_id: String,

    /// Detected archive format
    pub format: ArchiveFormat,

    /// Whether the manifest was found and parseable
    pub manifest_found: bool,

    /// Initial validation status
    pub validation_status: ValidationStatus,

    /// Quick summary of found records
    pub record_counts: RecordCounts,

    /// List of images found in the archive
    pub images_found: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum ArchiveFormat {
    Zip,
    TarGz,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum ValidationStatus {
    /// Schema validation passed
    Valid,
    /// Schema validation failed with errors
    Invalid { error_count: u32 },
    /// Manifest could not be parsed
    ParseError { message: String },
}
```

**Errors**:

- `FileNotFound` - The specified file path does not exist
- `UnsupportedFormat` - File extension is not `.zip` or `.gz`/`.tgz`
- `ArchiveCorrupted` - Archive cannot be extracted
- `ManifestMissing` - No `manifest.json` at archive root
- `ManifestParseError` - JSON syntax error in manifest

---

### 2. `get_import_preview`

Generates a detailed preview of what the import will do, including duplicate detection.

**Command Name**: `get_import_preview`

**Args**:

```rust
#[derive(Debug, Clone, Deserialize, specta::Type, garde::Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetImportPreviewArgs {
    /// Session ID from analyze_import_package
    #[garde(length(min = 1))]
    pub session_id: String,
}
```

**Response**:

```rust
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ImportPreviewResponse {
    /// Session ID
    pub session_id: String,

    /// Total records in manifest
    pub total_records: RecordCounts,

    /// Records that will be imported (new)
    pub new_records: RecordCounts,

    /// Records that will be skipped (duplicates)
    pub duplicate_records: RecordCounts,

    /// Validation errors (if any, import cannot proceed)
    pub errors: Vec<ValidationErrorDto>,

    /// Warnings (non-blocking, e.g., missing images)
    pub warnings: Vec<ImportWarningDto>,

    /// Whether import can proceed
    pub can_import: bool,
}

#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RecordCounts {
    pub manufacturers: u32,    /// Note: serializes to "railwayCompanies" via rename_all    pub railway_companies: u32,
    pub railway_models: u32,
    pub collection_items: u32,
    pub sellers: u32,
    pub maintenance_cards: u32,
}

#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ValidationErrorDto {
    /// JSON path to the error, e.g., "data.railwayModels[3].productCode"
    pub path: String,

    /// Error code for i18n
    pub code: String,

    /// Human-readable message
    pub message: String,
}

#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ImportWarningDto {
    /// Warning code for i18n
    pub code: String,

    /// Human-readable message
    pub message: String,

    /// Related entity or file
    pub context: Option<String>,
}
```

**Errors**:

- `SessionNotFound` - Session ID does not exist or expired
- `SessionInvalidState` - Session is not in `Analyzed` state

---

### 3. `execute_import`

Executes the import after user confirmation. This is an atomic operation.

**Command Name**: `execute_import`

**Args**:

```rust
#[derive(Debug, Clone, Deserialize, specta::Type, garde::Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteImportArgs {
    /// Session ID from analyze_import_package
    #[garde(length(min = 1))]
    pub session_id: String,
}
```

**Response**:

```rust
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ImportResultResponse {
    /// Session ID
    pub session_id: String,

    /// Import outcome
    pub status: ImportOutcome,

    /// Records successfully added
    pub added: RecordCounts,

    /// Records skipped (duplicates)
    pub skipped: RecordCounts,

    /// Images successfully imported
    pub images_imported: u32,

    /// Images that failed to import
    pub images_failed: Vec<ImageFailureDto>,

    /// Duration in milliseconds
    pub duration_ms: u64,

    /// Any warnings during import
    pub warnings: Vec<ImportWarningDto>,
}

#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum ImportOutcome {
    /// All operations succeeded
    Success,
    /// Import completed with some warnings
    SuccessWithWarnings,
    /// Import failed and was rolled back
    Failed { reason: String },
}

#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ImageFailureDto {
    pub filename: String,
    pub reason: String,
}
```

**Errors**:

- `SessionNotFound` - Session ID does not exist or expired
- `SessionInvalidState` - Session is not in `Previewed` state
- `ValidationFailed` - Re-validation failed (data changed)
- `DatabaseError` - Transaction failed (automatically rolled back)

---

### 4. `cancel_import_session`

Cancels an active import session and cleans up temporary files.

**Command Name**: `cancel_import_session`

**Args**:

```rust
#[derive(Debug, Clone, Deserialize, specta::Type, garde::Validate)]
#[serde(rename_all = "camelCase")]
pub struct CancelImportSessionArgs {
    #[garde(length(min = 1))]
    pub session_id: String,
}
```

**Response**:

```rust
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct CancelImportSessionResponse {
    pub session_id: String,
    pub cancelled: bool,
}
```

**Errors**:

- `SessionNotFound` - Session ID does not exist

---

## Events

### `import-progress`

Emitted during long-running import operations for progress updates.

**Event Name**: `import-progress`

**Payload**:

```rust
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ImportProgressEvent {
    /// Session ID
    pub session_id: String,

    /// Current phase description
    pub phase: String,

    /// Current item being processed
    pub current: u32,

    /// Total items to process
    pub total: u32,

    /// Completion percentage (0-100)
    pub percentage: u8,
}
```

**Phases**:

- `"Extracting archive"`
- `"Validating manifest"`
- `"Checking duplicates"`
- `"Importing manufacturers"`
- `"Importing railway companies"`
- `"Importing railway models"`
- `"Importing collection items"`
- `"Importing sellers"`
- `"Importing maintenance cards"`
- `"Copying images"`

---

## Error Response Format

All errors follow the standard `CommandError` format:

```rust
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct CommandError {
    /// Error code for programmatic handling
    pub code: String,

    /// Human-readable message (for fallback, prefer i18n)
    pub message: String,

    /// Additional context
    pub details: Option<serde_json::Value>,
}
```

---

## TypeScript Usage Examples

```typescript
import { commands, events } from '$lib/bindings';

// 1. Analyze package
const analyzeResult = await commands.analyzeImportPackage({
  filePath: '/path/to/export.zip'
});

if (analyzeResult.status === 'error') {
  console.error('Analysis failed:', analyzeResult.error);
  return;
}

const { sessionId, recordCounts } = analyzeResult.data;

// 2. Get preview
const previewResult = await commands.getImportPreview({ sessionId });

if (previewResult.status === 'ok' && previewResult.data.canImport) {
  // Show preview to user...

  // 3. Listen for progress
  const unlisten = await events.importProgress.listen((event) => {
    console.log(`${event.payload.phase}: ${event.payload.percentage}%`);
  });

  // 4. Execute import
  const importResult = await commands.executeImport({ sessionId });

  unlisten();

  if (importResult.status === 'ok') {
    console.log(`Added: ${importResult.data.added.railwayModels} models`);
  }
}

// Cleanup if cancelled
await commands.cancelImportSession({ sessionId });
```
