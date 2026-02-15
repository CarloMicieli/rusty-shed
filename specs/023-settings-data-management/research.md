# Research: Settings Data Management UI

**Feature**: 023-settings-data-management
**Created**: 2026-02-15
**Purpose**: Technical research and design decisions for database backup/restore functionality

## Overview

This document captures research findings, technical decisions, and best practices for implementing manual database backup and restore functionality in the Rusty Shed settings page.

## Research Questions Resolved

### 1. How to trigger native file dialogs in Tauri 2.x?

**Decision**: Use `@tauri-apps/plugin-dialog` for file picker operations

**Rationale**:

- Tauri 2.x provides dedicated dialog plugin for save/open file pickers
- Provides native OS dialogs (better UX than web-based file pickers)
- Supports filters for file extensions
- Returns file paths as strings that can be passed to backend commands

**API Usage**:

```typescript
import { save, open } from '@tauri-apps/plugin-dialog';

// Export (save dialog)
const filePath = await save({
  defaultPath: `rusty-shed-backup-${new Date().toISOString()}.sqlite`,
  filters: [
    {
      name: 'SQLite Database',
      extensions: ['sqlite', 'db']
    }
  ]
});

// Import (open dialog)
const filePath = await open({
  filters: [
    {
      name: 'SQLite Database',
      extensions: ['sqlite', 'db']
    }
  ]
});
```

**Alternatives Considered**:

- Web File API: Rejected - not available in Tauri context
- Custom file browser UI: Rejected - reinventing the wheel, worse UX

### 2. How to locate the app's database file path?

**Decision**: Use `AppHandle::path().app_data_dir()` to get database location

**Rationale**:

- `AppHandle` provides OS-agnostic path resolution
- `app_data_dir()` returns the standard app data directory for the platform:
  - Linux: `~/.local/share/com.rusty-shed.app/`
  - macOS: `~/Library/Application Support/com.rusty-shed.app/`
  - Windows: `C:\Users\<user>\AppData\Roaming\com.rusty-shed.app\`
- Database file is `database.sqlite` in this directory
- Already used in cloud backup implementation

**Code Pattern** (from `cloud_backup.rs`):

```rust
let db_path = app
    .path()
    .app_data_dir()
    .map_err(|e| CommandError::Unknown(format!("Failed to resolve app data dir: {}", e)))?
    .join("database.sqlite");
```

**Alternatives Considered**:

- Hardcoded paths: Rejected - not cross-platform
- Environment variables: Rejected - less reliable than Tauri's path resolver

### 3. How to safely copy SQLite database files?

**Decision**: Use SQLite's online backup API for export, simple file copy with validation for import

**Rationale for Export (using `VACUUM INTO`)**:

- `VACUUM INTO` creates a clean, compacted copy of the database
- Atomic operation - doesn't interfere with ongoing queries
- Automatically optimizes the exported file (removes fragmentation)
- SQLite built-in command, no external dependencies

**Implementation**:

```rust
// Export using VACUUM INTO
sqlx::query("VACUUM INTO ?")
    .bind(&destination_path)
    .execute(pool)
    .await?;
```

**Rationale for Import**:

- Close database connection pool before import
- Validate backup file is a valid SQLite database
- Copy file to app data directory
- Reopen connection pool
- App restart required to reflect changes

**Validation Strategy**:

```rust
// Check if file is valid SQLite database
async fn validate_sqlite_file(path: &Path) -> Result<(), DatabaseBackupError> {
    let conn = SqliteConnection::connect(&format!("sqlite:{}", path.display())).await?;
    // Try to read schema
    sqlx::query("SELECT name FROM sqlite_master WHERE type='table' LIMIT 1")
        .fetch_one(&mut conn)
        .await?;
    Ok(())
}
```

**Alternatives Considered**:

- `.backup` command: Rejected - requires locking the database
- Simple file copy for export: Rejected - may capture inconsistent state
- Online restore without restart: Rejected - complex, risky with active connections

### 4. How to handle file operations without blocking the UI?

**Decision**: Use Tokio async operations with Tauri's async command handlers

**Rationale**:

- Tauri commands support async/await natively
- Tokio provides efficient async file I/O (`tokio::fs`)
- Operations run on background thread pool
- Frontend can show loading states while commands execute

**Pattern**:

```rust
#[tauri::command]
#[specta::specta]
pub async fn export_database(
    args: ExportDatabaseArgs,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<ExportDatabaseResponse, CommandError> {
    // Async operations here
    tokio::fs::copy(source, destination).await?;
    Ok(response)
}
```

**Alternatives Considered**:

- Synchronous operations: Rejected - would block UI thread
- Manual threading: Rejected - Tauri + Tokio handle this better

### 5. How to ensure consistent UI/UX with existing Settings page patterns?

**Decision**: Follow established patterns from Cloud Backup section

**Key Patterns Identified**:

1. **Section Structure**:

   ```svelte
   <div class="card border-surface-700/40 border p-6 shadow-xl">
     <div class="space-y-4">
       <div>
         <h2 class="text-xl font-bold">{title}</h2>
         <p class="text-surface-400 mt-1 text-sm">{subtitle}</p>
       </div>
       <!-- Action buttons -->
     </div>
   </div>
   ```

2. **Button Styling**:
   - Primary actions use `variant="filled"` with orange accent
   - Same pattern as "Connect Google Drive" and "Save" buttons
   - Use `Button` component from shadcn-svelte

3. **Warning Messages**:
   - Use `variant-soft-warning` for warning callouts
   - Position below destructive action buttons
   - Clear, concise warning text

4. **Loading States**:
   - Disable buttons during operations
   - Show loading spinner or text
   - Use Svelte's reactive state (`$state()`)

5. **i18n**:
   - All strings via Paraglide (`m.*()` functions)
   - Add keys to `messages/en.json` and `messages/it.json`

**Component Pattern** (from Cloud Backup):

```svelte
<script lang="ts">
  import { Button } from '$lib/components';
  import * as m from '$lib/paraglide/messages.js';

  let loading = $state(false);

  async function handleExport() {
    loading = true;
    // ... operation
    loading = false;
  }
</script>

<div class="card border-surface-700/40 border p-6 shadow-xl">
  <div class="space-y-4">
    <div>
      <h2 class="text-xl font-bold">{m.data_management_title()}</h2>
      <p class="text-surface-400 mt-1 text-sm">{m.data_management_subtitle()}</p>
    </div>

    <div class="flex gap-4">
      <Button variant="filled" disabled={loading} onclick={handleExport}>
        {m.data_management_export_button()}
      </Button>
    </div>
  </div>
</div>
```

**Alternatives Considered**:

- Custom styling: Rejected - inconsistent with existing UI
- New component library: Rejected - unnecessary, shadcn-svelte works well

### 6. How to handle import warnings and confirmation?

**Decision**: Use two-step confirmation process with clear warnings

**Rationale**:

- Import is destructive (overwrites database)
- Users need clear understanding of consequences
- Follow pattern from Cloud Backup's RestoreConfirmModal

**Implementation Strategy**:

1. **Inline Warning** (always visible):

   ```svelte
   <div class="variant-soft-warning rounded-container p-3 text-sm">
     <p>{m.data_management_import_warning()}</p>
   </div>
   ```

2. **Confirmation Modal** (on import button click):

   ```svelte
   <ConfirmDialog
     title={m.data_management_import_confirm_title()}
     message={m.data_management_import_confirm_message()}
     confirmText="RESTORE"
     onConfirm={handleImportConfirm}
     onCancel={handleImportCancel}
   />
   ```

3. **Typed Confirmation**:
   - User must type "RESTORE" to confirm (like cloud backup)
   - Prevents accidental imports

**Alternatives Considered**:

- Single-click import: Rejected - too risky for destructive operation
- Simple OK/Cancel: Rejected - not strong enough warning
- No warning: Rejected - violates UX best practices

## Technology Stack Summary

### Backend (Rust)

- **File Dialogs**: Handled in frontend (`@tauri-apps/plugin-dialog`)
- **File Operations**: `tokio::fs` for async file I/O
- **Database Operations**: `sqlx` for `VACUUM INTO` export
- **Path Resolution**: `tauri::AppHandle::path()`
- **Error Handling**: `Result<T, CommandError>` pattern
- **Validation**: SQLite connection test for file validation

### Frontend (TypeScript/Svelte)

- **File Pickers**: `@tauri-apps/plugin-dialog`
- **UI Components**: shadcn-svelte (`Button`, `Card`, etc.)
- **State Management**: Svelte 5 runes (`$state`, `$derived`)
- **i18n**: Paraglide (`$lib/paraglide/messages`)
- **Styling**: Tailwind CSS 4.1.18 with Skeleton UI tokens
- **Toasts**: Custom toaster service (`$lib/toaster`)

## Best Practices Applied

1. **Async Operations**: All file I/O is async to keep UI responsive
2. **Validation**: Validate database file before import
3. **Error Handling**: Comprehensive error messages for all failure modes
4. **User Feedback**: Progress indicators, success/error toasts, clear warnings
5. **Type Safety**: specta-generated TypeScript types from Rust
6. **i18n**: All strings externalized via Paraglide
7. **Accessibility**: Use semantic HTML, ARIA labels where needed
8. **Testing**: Unit tests for commands, component tests for UI

## Dependencies

### New Dependencies Required

None - all required dependencies already in project:

- `@tauri-apps/plugin-dialog` - already installed
- `tokio::fs` - part of tokio (already used)
- `sqlx` - already used for database operations

### Existing Patterns to Reuse

- Cloud Backup command handlers (for structure and error handling)
- Settings page component patterns (for UI consistency)
- Tauri AppHandle path resolution (from cloud backup)
- RestoreConfirmModal pattern (for import confirmation)

## Security Considerations

1. **File Path Validation**:
   - Validate destination paths exist and are writable
   - Prevent path traversal attacks via file picker (OS handles this)

2. **Database Validation**:
   - Verify imported file is a valid SQLite database
   - Check schema compatibility before import

3. **Error Messages**:
   - Don't expose internal file paths in error messages
   - Use generic error messages for security-related failures

4. **User Permissions**:
   - Rely on OS file system permissions
   - Tauri sandboxing provides additional security layer

## Performance Considerations

1. **File Size Limits**:
   - Target: <30s for 100MB databases
   - Use streaming operations where possible

2. **Progress Indication**:
   - Show progress for operations >2s
   - Consider Tauri events for real-time progress updates (future enhancement)

3. **Database Locking**:
   - Export uses `VACUUM INTO` (no locking required)
   - Import requires app restart (closes all connections)

## References

- [Tauri Dialog Plugin Docs](https://v2.tauri.app/plugin/dialog/)
- [SQLite VACUUM INTO](https://www.sqlite.org/lang_vacuum.html)
- [Tokio File I/O](https://docs.rs/tokio/latest/tokio/fs/)
- [Cloud Backup Implementation](../../src-tauri/src/commands/cloud_backup.rs)
- [Settings Page Component](../../src/routes/my-settings/+page.svelte)
