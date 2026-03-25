import { save, open } from '@tauri-apps/plugin-dialog';
import { exportDatabase, importDatabase } from '$lib/services';
import { toaster } from '$lib/toaster';
import * as m from '$lib/paraglide/messages.js';
import type { DatabaseBackupState } from './DatabaseBackupState.svelte';

export class DatabaseBackupController {
  #state: DatabaseBackupState;

  constructor() {
    this.#state = $state<DatabaseBackupState>({
      isExporting: false,
      isImporting: false,
      isOperationInProgress: false,
      error: null
    });
  }

  get isExporting(): boolean {
    return this.#state.isExporting;
  }

  get isImporting(): boolean {
    return this.#state.isImporting;
  }

  get isOperationInProgress(): boolean {
    return this.#state.isOperationInProgress;
  }

  get error(): string | null {
    return this.#state.error;
  }

  clearError(): void {
    this.#state.error = null;
  }

  async handleExport(): Promise<void> {
    if (this.#state.isOperationInProgress) return;

    // Open save file dialog
    const destinationPath = await save({
      title: m.data_management_file_picker_export_title(),
      filters: [{ name: 'SQLite Database', extensions: ['sqlite', 'db'] }],
      // 'en-CA' produces ISO 8601 YYYY-MM-DD for safe, sortable filenames — not for UI display.
      defaultPath: `rusty-shed-backup-${new Intl.DateTimeFormat('en-CA').format(Date.now())}.sqlite`
    });

    if (!destinationPath) {
      // User cancelled
      return;
    }

    this.#state.isExporting = true;
    this.#state.isOperationInProgress = true;
    this.#state.error = null;

    try {
      const result = await exportDatabase(destinationPath);
      if (result.ok) {
        toaster.success({
          title: m.data_management_export_success({ path: result.data.file_path })
        });
      } else {
        const errorMsg = result.error.message;
        this.#state.error = errorMsg;
        toaster.error({
          title: m.data_management_export_error({ error: errorMsg })
        });
      }
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : String(err);
      this.#state.error = errorMsg;
      toaster.error({
        title: m.data_management_export_error({ error: errorMsg })
      });
    } finally {
      this.#state.isExporting = false;
      this.#state.isOperationInProgress = false;
    }
  }

  async handleImport(): Promise<void> {
    if (this.#state.isOperationInProgress) return;

    // Open file picker dialog
    const selectedPath = await open({
      title: m.data_management_file_picker_import_title(),
      filters: [{ name: 'SQLite Database', extensions: ['sqlite', 'db'] }],
      multiple: false
    });

    if (!selectedPath) {
      // User cancelled
      return;
    }

    const sourcePath = typeof selectedPath === 'string' ? selectedPath : selectedPath[0];
    if (!sourcePath) return;

    this.#state.isImporting = true;
    this.#state.isOperationInProgress = true;
    this.#state.error = null;

    try {
      const result = await importDatabase(sourcePath, 'RESTORE');
      if (result.ok) {
        toaster.success({
          title: m.data_management_import_success()
        });
      } else {
        const errorMsg = result.error.message;
        this.#state.error = errorMsg;
        toaster.error({
          title: m.data_management_import_error({ error: errorMsg })
        });
      }
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : String(err);
      this.#state.error = errorMsg;
      toaster.error({
        title: m.data_management_import_error({ error: errorMsg })
      });
    } finally {
      this.#state.isImporting = false;
      this.#state.isOperationInProgress = false;
    }
  }
}

let controllerInstance: DatabaseBackupController | null = null;

export function getDatabaseBackupController(): DatabaseBackupController {
  if (!controllerInstance) {
    controllerInstance = new DatabaseBackupController();
  }
  return controllerInstance;
}
