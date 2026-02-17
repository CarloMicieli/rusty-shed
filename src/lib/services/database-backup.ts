/**
 * Database backup/restore service layer.
 *
 * Provides typed wrappers for database backup Tauri commands.
 */
import type { ExportDatabaseResponse, ImportDatabaseResponse } from '$lib/bindings';
import type { SafeResult } from './errors';
import { safeInvoke } from './tauri';

export type { ExportDatabaseResponse, ImportDatabaseResponse };

/**
 * Export the database to a user-selected file path.
 */
export async function exportDatabase(
  destinationPath: string
): Promise<SafeResult<ExportDatabaseResponse>> {
  return safeInvoke<ExportDatabaseResponse>('export_database', {
    args: { destination_path: destinationPath }
  });
}

/**
 * Import (restore) the database from a backup file.
 */
export async function importDatabase(
  sourcePath: string,
  confirmation: string
): Promise<SafeResult<ImportDatabaseResponse>> {
  return safeInvoke<ImportDatabaseResponse>('import_database', {
    args: { source_path: sourcePath, confirmation }
  });
}
