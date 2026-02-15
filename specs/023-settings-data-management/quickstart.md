# Quickstart Guide: Settings Data Management UI

**Feature**: 023-settings-data-management
**For**: Developers implementing the database backup/restore feature
**Estimated Time**: 3-4 hours for complete implementation

## Overview

This guide provides a step-by-step walkthrough for implementing the Settings Data Management UI feature, which adds manual database backup (export) and restore (import) functionality to the Rusty Shed application.

## Prerequisites

- Rust 1.93.0 or later
- Node.js with pnpm installed
- Tauri 2.9.x environment set up
- Familiarity with Svelte 5 runes and Tauri command patterns

## Implementation Steps

### Phase 1: Backend Implementation (Rust)

#### Step 1.1: Create Domain Module

**Location**: `src-tauri/src/database_backup/domain/`

1. Create `errors.rs`:

   ```rust
   #[derive(Debug, thiserror::Error)]
   pub enum DatabaseBackupError {
       #[error("Invalid file path: {0}")]
       InvalidPath(String),
       // ... other variants from data-model.md
   }
   ```

2. Create `validation.rs`:

   ```rust
   use std::path::Path;
   use sqlx::SqliteConnection;

   pub async fn validate_sqlite_file(path: &Path) -> Result<(), DatabaseBackupError> {
       // Implementation from research.md
   }
   ```

3. Create `mod.rs` to export types

**Validation**: Run `cargo check` to ensure no compilation errors

#### Step 1.2: Create Application Layer

**Location**: `src-tauri/src/database_backup/application/`

1. Create `export_database.rs`:

   ```rust
   use sqlx::SqlitePool;
   use std::path::Path;

   pub async fn export_database(
       pool: &SqlitePool,
       destination_path: &Path,
   ) -> Result<ExportResult, DatabaseBackupError> {
       let start = std::time::Instant::now();

       // Execute VACUUM INTO
       sqlx::query("VACUUM INTO ?")
           .bind(destination_path.to_str().unwrap())
           .execute(pool)
           .await
           .map_err(|e| DatabaseBackupError::DatabaseError(e.to_string()))?;

       // Get file size
       let metadata = tokio::fs::metadata(destination_path).await
           .map_err(|e| DatabaseBackupError::FileSystemError(e.to_string()))?;

       Ok(ExportResult {
           file_path: destination_path.to_string_lossy().to_string(),
           file_size_bytes: metadata.len(),
           duration_ms: start.elapsed().as_millis() as u64,
       })
   }
   ```

2. Create `import_database.rs`:
   ```rust
   // Import use case implementation
   // See data-model.md for full implementation
   ```

**Validation**: Run `cargo test` for application layer tests

#### Step 1.3: Create Command Handlers

**Location**: `src-tauri/src/commands/database_backup.rs`

1. Create command handlers:

   ```rust
   use crate::database_backup::application;
   use crate::core::infrastructure::error::CommandError;
   use crate::state::AppState;
   use tauri::{AppHandle, State};

   #[derive(Debug, Clone, Validate, Deserialize, specta::Type)]
   pub struct ExportDatabaseArgs {
       #[validate(length(min = 1))]
       pub destination_path: String,
   }

   #[derive(Debug, Clone, Serialize, specta::Type)]
   pub struct ExportDatabaseResponse {
       pub file_path: String,
       pub file_size_bytes: u64,
       pub duration_ms: u64,
       pub message: String,
   }

   #[tauri::command]
   #[specta::specta]
   pub async fn export_database(
       args: ExportDatabaseArgs,
       state: State<'_, AppState>,
   ) -> Result<ExportDatabaseResponse, CommandError> {
       // Validate args
       args.validate().map_err(|e| CommandError::validation_field("destination_path", &e.to_string()))?;

       // Call use case
       let result = application::export_database(
           state.db_pool(),
           &std::path::Path::new(&args.destination_path),
       )
       .await
       .map_err(CommandError::from)?;

       Ok(ExportDatabaseResponse {
           file_path: result.file_path,
           file_size_bytes: result.file_size_bytes,
           duration_ms: result.duration_ms,
           message: "Database exported successfully".to_string(),
       })
   }

   // Similar implementation for import_database
   ```

2. Register commands in `src-tauri/src/commands/mod.rs`:

   ```rust
   pub mod database_backup;
   ```

3. Register in `src-tauri/src/lib.rs`:

   ```rust
   use commands::database_backup::{export_database, import_database};

   // In the tauri::Builder setup:
   .invoke_handler(tauri::generate_handler![
       // ... existing commands
       export_database,
       import_database,
   ])
   ```

**Validation**:

- Run `cargo clippy` - should have no warnings
- Run `cargo test` - all tests should pass

#### Step 1.4: Generate TypeScript Bindings

**Run specta**:

```bash
cd src-tauri
cargo test --features specta  # Generates TypeScript types
```

**Validation**: Check that `src/lib/bindings.ts` includes new types

### Phase 2: Frontend Implementation (TypeScript/Svelte)

#### Step 2.1: Create Service Layer

**Location**: `src/lib/services/database-backup.ts`

```typescript
import { invoke } from '@tauri-apps/api/core';
import type { Result } from './types';
import type {
  ExportDatabaseArgs,
  ExportDatabaseResponse,
  ImportDatabaseArgs,
  ImportDatabaseResponse
} from '../bindings';

export async function exportDatabase(
  destinationPath: string
): Promise<Result<ExportDatabaseResponse>> {
  try {
    const response = await invoke<ExportDatabaseResponse>('export_database', {
      args: { destination_path: destinationPath }
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
      args: { source_path: sourcePath, confirmation }
    });
    return { ok: true, data: response };
  } catch (error) {
    return { ok: false, error: error as Error };
  }
}
```

**Export from**: `src/lib/services/index.ts`

#### Step 2.2: Create State Management

**Location**: `src/lib/features/database-backup/`

1. Create `DatabaseBackupState.svelte.ts`:

   ```typescript
   export type DatabaseBackupState = {
     isExporting: boolean;
     isImporting: boolean;
     isOperationInProgress: boolean;
     error: string | null;
   };

   export function createDatabaseBackupState(): DatabaseBackupState {
     return $state({
       isExporting: false,
       isImporting: false,
       isOperationInProgress: false,
       error: null
     });
   }
   ```

2. Create `DatabaseBackupController.svelte.ts`:

   ```typescript
   import { save, open } from '@tauri-apps/plugin-dialog';
   import { exportDatabase, importDatabase } from '$lib/services';
   import { toaster } from '$lib/toaster';
   import * as m from '$lib/paraglide/messages';

   class DatabaseBackupController {
     private state = createDatabaseBackupState();

     get isExporting() {
       return this.state.isExporting;
     }

     get isImporting() {
       return this.state.isImporting;
     }

     get isOperationInProgress() {
       return this.state.isOperationInProgress;
     }

     async handleExport() {
       // Implementation from research.md
     }

     async handleImport(confirmation: string) {
       // Implementation from research.md
     }
   }

   export function getDatabaseBackupController() {
     return new DatabaseBackupController();
   }
   ```

3. Create `index.ts` to export controller

#### Step 2.3: Create UI Component

**Location**: `src/lib/features/database-backup/components/DataManagementSection.svelte`

```svelte
<script lang="ts">
  import { Button } from '$lib/components';
  import { getDatabaseBackupController } from '../index';
  import * as m from '$lib/paraglide/messages';

  const controller = getDatabaseBackupController();

  let isExporting = $derived(controller.isExporting);
  let isImporting = $derived(controller.isImporting);
  let isDisabled = $derived(isExporting || isImporting);

  async function handleExport() {
    await controller.handleExport();
  }

  async function handleImport() {
    await controller.handleImport();
  }
</script>

<div class="card border-surface-700/40 border p-6 shadow-xl">
  <div class="space-y-4">
    <div>
      <h2 class="text-xl font-bold">{m.data_management_title()}</h2>
      <p class="text-surface-400 mt-1 text-sm">{m.data_management_subtitle()}</p>
    </div>

    <div class="flex gap-4">
      <Button variant="filled" disabled={isDisabled} onclick={handleExport}>
        {isExporting ? 'Exporting...' : m.data_management_export_button()}
      </Button>

      <Button variant="filled" disabled={isDisabled} onclick={handleImport}>
        {isImporting ? 'Importing...' : m.data_management_import_button()}
      </Button>
    </div>

    <div class="variant-soft-warning rounded-container p-3 text-sm">
      <p>{m.data_management_import_warning()}</p>
    </div>
  </div>
</div>
```

#### Step 2.4: Add i18n Messages

**Location**: `messages/en.json`

Add all message keys from data-model.md

**Location**: `messages/it.json`

Add Italian translations

#### Step 2.5: Integrate with Settings Page

**Location**: `src/routes/my-settings/+page.svelte`

1. Import component:

   ```svelte
   import DataManagementSection from
   '$lib/features/database-backup/components/DataManagementSection.svelte';
   ```

2. Add section ABOVE Cloud Backup:

   ```svelte
   <div class="space-y-6">
     <SettingsForm {settings} {saving} onsubmit={handleSubmit} />

     <!-- NEW: Data Management Section -->
     <DataManagementSection />

     <!-- Existing: Cloud Backup Section -->
     <div class="card border-surface-700/40 border p-6 shadow-xl">
       <!-- ... existing cloud backup content ... -->
     </div>
   </div>
   ```

### Phase 3: Testing

#### Step 3.1: Backend Tests

**Location**: `src-tauri/src/database_backup/application/`

Create tests for:

- Export with valid destination
- Export with invalid destination
- Import with valid database file
- Import with invalid file
- Import with wrong confirmation

Run: `cargo test`

#### Step 3.2: Frontend Tests

**Location**: `src/__tests__/features/database-backup/`

Create component tests:

- DataManagementSection renders correctly
- Export button triggers file picker
- Import button shows confirmation
- Loading states work correctly

Run: `pnpm test`

### Phase 4: Quality Checks

#### Step 4.1: Lint and Format

```bash
# Frontend
pnpm format
pnpm lint
pnpm check

# Backend
cargo fmt
cargo clippy
```

#### Step 4.2: Manual Testing

1. **Export Test**:
   - Open Settings page
   - Click "Export Data"
   - Select save location
   - Verify file is created
   - Check file size is reasonable

2. **Import Test**:
   - Click "Import Data"
   - Select previously exported file
   - Confirm warning dialog
   - Verify database is restored
   - Restart app and verify data

3. **Error Cases**:
   - Try importing an invalid file
   - Try exporting to read-only location
   - Cancel file pickers

## Development Workflow

### Quick Start

```bash
# 1. Create backend structure
cd src-tauri
cargo new --lib database_backup
cargo build

# 2. Create frontend structure
cd ..
mkdir -p src/lib/features/database-backup/components

# 3. Run in dev mode
pnpm tauri dev

# 4. Test changes
cargo test
pnpm test
```

### Debugging Tips

1. **Backend Logs**:

   ```rust
   log::info!("Exporting to: {:?}", destination_path);
   ```

2. **Frontend Logs**:

   ```typescript
   console.log('Export response:', response);
   ```

3. **Tauri DevTools**:
   - Open DevTools in Tauri window
   - Check Network tab for command calls
   - Check Console for errors

## Common Issues & Solutions

### Issue: "Command not found" error

**Solution**: Ensure command is registered in `lib.rs` invoke_handler

### Issue: TypeScript types not generated

**Solution**: Run `cargo test --features specta` to regenerate bindings

### Issue: File picker doesn't open

**Solution**: Check `@tauri-apps/plugin-dialog` is installed and imported

### Issue: Database file not found

**Solution**: Verify `AppHandle::path().app_data_dir()` is correct

## Next Steps

After completing this implementation:

1. Test on all platforms (Windows, macOS, Linux)
2. Update user documentation
3. Consider adding:
   - Automatic backup scheduling
   - Backup file encryption
   - Progress indicators for large files
4. Monitor user feedback for improvements

## References

- [Feature Specification](./spec.md)
- [Implementation Plan](./plan.md)
- [Research Document](./research.md)
- [Data Model](./data-model.md)
- [Tauri 2.x Documentation](https://v2.tauri.app/)
- [Svelte 5 Documentation](https://svelte.dev/)
