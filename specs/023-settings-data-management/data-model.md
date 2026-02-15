# Data Model: Settings Data Management UI

**Feature**: 023-settings-data-management
**Created**: 2026-02-15
**Purpose**: Define command contracts, DTOs, and data structures for database backup/restore

## Overview

This feature introduces minimal data structures focused on command arguments and responses for database export/import operations. No domain entities or database schema changes are required.

## Command Contracts

### Export Database Command

**Command Name**: `export_database`

**Purpose**: Export the entire SQLite database to a user-selected file location

**Input Arguments** (`ExportDatabaseArgs`):

```rust
#[derive(Debug, Clone, Validate, Deserialize, specta::Type)]
pub struct ExportDatabaseArgs {
    /// Destination file path (absolute path from file picker)
    #[validate(length(min = 1, message = "Destination path is required"))]
    pub destination_path: String,
}
```

**Response** (`ExportDatabaseResponse`):

```rust
#[derive(Debug, Clone, Serialize, specta::Type)]
pub struct ExportDatabaseResponse {
    /// Path where the backup was saved
    pub file_path: String,

    /// Size of the exported file in bytes
    pub file_size_bytes: u64,

    /// Duration of the export operation in milliseconds
    pub duration_ms: u64,

    /// Success message
    pub message: String,
}
```

**Error Cases**:

- `InvalidPath`: Destination path doesn't exist or is not writable
- `DatabaseError`: Failed to execute VACUUM INTO
- `FileSystemError`: Failed to verify exported file
- `PermissionDenied`: Insufficient permissions to write to destination

### Import Database Command

**Command Name**: `import_database`

**Purpose**: Restore the SQLite database from a user-selected backup file

**Input Arguments** (`ImportDatabaseArgs`):

```rust
#[derive(Debug, Clone, Validate, Deserialize, specta::Type)]
pub struct ImportDatabaseArgs {
    /// Source backup file path (absolute path from file picker)
    #[validate(length(min = 1, message = "Source path is required"))]
    pub source_path: String,

    /// Confirmation string (must be "RESTORE")
    #[validate(custom(function = "validate_confirmation"))]
    pub confirmation: String,
}

fn validate_confirmation(confirmation: &str) -> Result<(), ValidationError> {
    if confirmation != "RESTORE" {
        return Err(ValidationError::new("Must type 'RESTORE' to confirm"));
    }
    Ok(())
}
```

**Response** (`ImportDatabaseResponse`):

```rust
#[derive(Debug, Clone, Serialize, specta::Type)]
pub struct ImportDatabaseResponse {
    /// Path of the imported file
    pub file_path: String,

    /// Size of the imported file in bytes
    pub file_size_bytes: u64,

    /// Duration of the import operation in milliseconds
    pub duration_ms: u64,

    /// Success message
    pub message: String,

    /// Whether app restart is required (always true for import)
    pub requires_restart: bool,
}
```

**Error Cases**:

- `InvalidPath`: Source file doesn't exist or is not readable
- `InvalidDatabase`: File is not a valid SQLite database
- `IncompatibleSchema`: Database schema doesn't match expected structure
- `ConfirmationFailed`: Confirmation string doesn't match "RESTORE"
- `FileSystemError`: Failed to copy file to app data directory
- `PermissionDenied`: Insufficient permissions to read source or write destination

## Domain Types

### DatabaseBackupError

**Purpose**: Custom error type for database backup/restore operations

```rust
#[derive(Debug, thiserror::Error)]
pub enum DatabaseBackupError {
    #[error("Invalid file path: {0}")]
    InvalidPath(String),

    #[error("Invalid SQLite database: {0}")]
    InvalidDatabase(String),

    #[error("Incompatible database schema: {0}")]
    IncompatibleSchema(String),

    #[error("Confirmation failed: {0}")]
    ConfirmationFailed(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("File system error: {0}")]
    FileSystemError(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Operation in progress")]
    OperationInProgress,
}

impl From<DatabaseBackupError> for CommandError {
    fn from(err: DatabaseBackupError) -> Self {
        match err {
            DatabaseBackupError::InvalidPath(msg) => CommandError::validation("destination_path", &msg),
            DatabaseBackupError::InvalidDatabase(msg) => CommandError::validation("source_path", &msg),
            DatabaseBackupError::IncompatibleSchema(msg) => CommandError::validation("source_path", &msg),
            DatabaseBackupError::ConfirmationFailed(msg) => CommandError::validation("confirmation", &msg),
            _ => CommandError::Unknown(err.to_string()),
        }
    }
}
```

## Validation Rules

### Export Validation

1. **Destination Path**:
   - Must not be empty
   - Parent directory must exist
   - Parent directory must be writable
   - File extension should be `.sqlite` or `.db` (warning if not)

2. **Database State**:
   - Database must be accessible (connection pool must be healthy)
   - No other export/import operation in progress

### Import Validation

1. **Source Path**:
   - Must not be empty
   - File must exist
   - File must be readable
   - File size must be reasonable (<5GB warning threshold)

2. **Database Format**:
   - File must be a valid SQLite database
   - Must be able to open and query schema
   - Schema must contain expected tables:
     - `railway_models`
     - `collection_items`
     - `manufacturers`
     - `railway_companies`
     - (Other core tables)

3. **Confirmation**:
   - Must exactly match "RESTORE" (case-sensitive)
   - Cannot be empty or null

4. **Operation State**:
   - No other export/import operation in progress
   - Database connection pool must be closeable (no active transactions)

## State Management (Frontend)

### DatabaseBackupState (Svelte Runes)

```typescript
export type DatabaseBackupState = {
  // Export state
  isExporting: boolean;
  exportProgress: number | null; // 0-100 or null if not tracked
  lastExportPath: string | null;
  lastExportDate: Date | null;

  // Import state
  isImporting: boolean;
  importProgress: number | null; // 0-100 or null if not tracked

  // General state
  isOperationInProgress: boolean; // true if export OR import active
  error: string | null;
};
```

### DatabaseBackupController (Svelte Class)

```typescript
class DatabaseBackupController {
  // State
  private state = $state<DatabaseBackupState>({
    isExporting: false,
    exportProgress: null,
    lastExportPath: null,
    lastExportDate: null,
    isImporting: false,
    importProgress: null,
    isOperationInProgress: false,
    error: null
  });

  // Derived values
  get isExporting() {
    return this.state.isExporting;
  }
  get isImporting() {
    return this.state.isImporting;
  }
  get isOperationInProgress() {
    return this.state.isOperationInProgress;
  }

  // Actions
  async exportDatabase(): Promise<void>;
  async importDatabase(confirmation: string): Promise<void>;
  clearError(): void;
}
```

## File Formats

### Database File

**Format**: SQLite 3 database file

**Extension**: `.sqlite` (primary), `.db` (secondary)

**MIME Type**: `application/vnd.sqlite3`

**Structure**: Standard SQLite format with Rusty Shed schema

**Validation Queries**:

```sql
-- Check database format
PRAGMA integrity_check;

-- Verify core tables exist
SELECT name FROM sqlite_master
WHERE type='table' AND name IN (
  'railway_models',
  'collection_items',
  'manufacturers',
  'railway_companies',
  'sellers',
  'maintenance_cards',
  'rolling_stocks'
);
```

## TypeScript Types (Generated by specta)

```typescript
// Auto-generated from Rust types via specta

export type ExportDatabaseArgs = {
  destination_path: string;
};

export type ExportDatabaseResponse = {
  file_path: string;
  file_size_bytes: number;
  duration_ms: number;
  message: string;
};

export type ImportDatabaseArgs = {
  source_path: string;
  confirmation: string;
};

export type ImportDatabaseResponse = {
  file_path: string;
  file_size_bytes: number;
  duration_ms: number;
  message: string;
  requires_restart: boolean;
};
```

## Service Layer (Frontend)

```typescript
// src/lib/services/database-backup.ts

import { invoke } from '@tauri-apps/api/core';
import type { Result } from './types';
import type {
  ExportDatabaseArgs,
  ExportDatabaseResponse,
  ImportDatabaseArgs,
  ImportDatabaseResponse
} from '../bindings'; // Generated by specta

export async function exportDatabase(
  destinationPath: string
): Promise<Result<ExportDatabaseResponse>> {
  try {
    const response = await invoke<ExportDatabaseResponse>('export_database', {
      args: { destination_path: destinationPath } as ExportDatabaseArgs
    });
    return { ok: true, data: response };
  } catch (error) {
    return { ok: false, error: error as Error };
  }
}

export async function importDatabase(
  sourcePath: string,
  confirmation: string
): Promise<Result<ImportDatabaseResponse>> {
  try {
    const response = await invoke<ImportDatabaseResponse>('import_database', {
      args: {
        source_path: sourcePath,
        confirmation
      } as ImportDatabaseArgs
    });
    return { ok: true, data: response };
  } catch (error) {
    return { ok: false, error: error as Error };
  }
}
```

## i18n Message Keys

### English Messages (`messages/en.json`)

```json
{
  "data_management_title": "Data Management",
  "data_management_subtitle": "Export and import your railway collection data",
  "data_management_export_button": "Export Data",
  "data_management_export_description": "Save a complete backup of your database to a local file",
  "data_management_import_button": "Import Data",
  "data_management_import_description": "Restore your database from a previously exported file",
  "data_management_import_warning": "⚠️ Importing data will overwrite your current local database",
  "data_management_import_confirm_title": "Confirm Database Import",
  "data_management_import_confirm_message": "This will replace your current database with the backup file. All existing data will be overwritten. Type 'RESTORE' to confirm.",
  "data_management_export_success": "Database exported successfully to {path}",
  "data_management_export_error": "Failed to export database: {error}",
  "data_management_import_success": "Database imported successfully. Please restart the app.",
  "data_management_import_error": "Failed to import database: {error}",
  "data_management_file_picker_export_title": "Export Database",
  "data_management_file_picker_import_title": "Import Database"
}
```

## Non-Functional Requirements

### Performance Targets

- Export: <30s for 100MB database
- Import: <60s for 100MB database
- Progress indicator appears after 2s

### File Size Limits

- Warning at 1GB
- Hard limit at 5GB (prevent out-of-memory errors)

### Error Recovery

- Failed export: No changes to original database
- Failed import: Original database remains intact, no partial imports

## References

- [Tauri Command Pattern](https://v2.tauri.app/develop/calling-rust/)
- [specta Type Generation](https://github.com/oscartbeaumont/specta)
- [validator crate](https://docs.rs/validator/latest/validator/)
- [SQLite VACUUM INTO](https://www.sqlite.org/lang_vacuum.html)
