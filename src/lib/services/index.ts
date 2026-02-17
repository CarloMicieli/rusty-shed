/**
 * Service layer exports.
 *
 * This module provides the main service layer for the application,
 * including error handling and Tauri IPC communication.
 */

export { safeInvoke, invokeOrThrow } from './tauri';
export type { NormalizedError, SafeResult, ErrorKind } from './errors';
export {
  getErrorMessage,
  getToastMessage,
  isValidationError,
  isNotFoundError,
  isRetryableError
} from './errors';
export { fetchSettings, saveSettings } from './settings';
export type {
  SettingsDto,
  UpdateSettingsPayload,
  Currency,
  MeasureUnit,
  PowerMethod,
  Scale,
  LanguageCode,
  ThemeValue
} from './settings';
export { exportDatabase, importDatabase } from './database-backup';
export type { ExportDatabaseResponse, ImportDatabaseResponse } from './database-backup';
