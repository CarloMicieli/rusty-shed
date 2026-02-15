/**
 * Database Backup/Restore API Contracts
 *
 * These types define the contract between the frontend and backend
 * for database export and import operations.
 *
 * NOTE: In production, these types are auto-generated from Rust via specta.
 * This file serves as documentation and reference during development.
 */

// ============================================================================
// Export Database
// ============================================================================

/**
 * Arguments for exporting the database
 */
export interface ExportDatabaseArgs {
  /**
   * Absolute path to the destination file
   * Example: "/Users/john/Documents/rusty-shed-backup-2026-02-15.sqlite"
   */
  destination_path: string;
}

/**
 * Response from successful database export
 */
export interface ExportDatabaseResponse {
  /**
   * Full path where the backup was saved
   */
  file_path: string;

  /**
   * Size of the exported file in bytes
   */
  file_size_bytes: number;

  /**
   * Duration of the export operation in milliseconds
   */
  duration_ms: number;

  /**
   * Human-readable success message
   */
  message: string;
}

// ============================================================================
// Import Database
// ============================================================================

/**
 * Arguments for importing (restoring) the database
 */
export interface ImportDatabaseArgs {
  /**
   * Absolute path to the source backup file
   * Example: "/Users/john/Downloads/rusty-shed-backup-2026-02-01.sqlite"
   */
  source_path: string;

  /**
   * Confirmation string (must be exactly "RESTORE")
   * This prevents accidental imports
   */
  confirmation: string;
}

/**
 * Response from successful database import
 */
export interface ImportDatabaseResponse {
  /**
   * Path of the imported file
   */
  file_path: string;

  /**
   * Size of the imported file in bytes
   */
  file_size_bytes: number;

  /**
   * Duration of the import operation in milliseconds
   */
  duration_ms: number;

  /**
   * Human-readable success message
   */
  message: string;

  /**
   * Whether the application requires restart to reflect changes
   * Always true for import operations
   */
  requires_restart: boolean;
}

// ============================================================================
// Error Types
// ============================================================================

/**
 * Error codes that can be returned from database backup operations
 */
export enum DatabaseBackupErrorCode {
  InvalidPath = 'INVALID_PATH',
  InvalidDatabase = 'INVALID_DATABASE',
  IncompatibleSchema = 'INCOMPATIBLE_SCHEMA',
  ConfirmationFailed = 'CONFIRMATION_FAILED',
  DatabaseError = 'DATABASE_ERROR',
  FileSystemError = 'FILE_SYSTEM_ERROR',
  PermissionDenied = 'PERMISSION_DENIED',
  OperationInProgress = 'OPERATION_IN_PROGRESS'
}

/**
 * Structured error response from backend
 */
export interface DatabaseBackupError {
  code: DatabaseBackupErrorCode;
  message: string;
  field?: string; // Optional field name for validation errors
}

// ============================================================================
// Frontend Service Types
// ============================================================================

/**
 * Result type for service layer operations
 */
export type Result<T> = { ok: true; data: T } | { ok: false; error: Error | DatabaseBackupError };

/**
 * Frontend state for database backup operations
 */
export interface DatabaseBackupState {
  // Export state
  isExporting: boolean;
  exportProgress: number | null; // 0-100 or null
  lastExportPath: string | null;
  lastExportDate: Date | null;

  // Import state
  isImporting: boolean;
  importProgress: number | null; // 0-100 or null

  // General state
  isOperationInProgress: boolean;
  error: string | null;
}

// ============================================================================
// File Picker Types
// ============================================================================

/**
 * File filter for database files
 */
export interface DatabaseFileFilter {
  name: string;
  extensions: string[];
}

/**
 * Default file filter for SQLite databases
 */
export const SQLITE_FILE_FILTER: DatabaseFileFilter = {
  name: 'SQLite Database',
  extensions: ['sqlite', 'db']
};

/**
 * Save dialog options for database export
 */
export interface ExportDialogOptions {
  defaultPath: string;
  filters: DatabaseFileFilter[];
}

/**
 * Open dialog options for database import
 */
export interface ImportDialogOptions {
  filters: DatabaseFileFilter[];
  multiple?: boolean;
}

// ============================================================================
// Constants
// ============================================================================

/**
 * Maximum file size warning threshold (1GB)
 */
export const FILE_SIZE_WARNING_THRESHOLD = 1024 * 1024 * 1024;

/**
 * Maximum file size hard limit (5GB)
 */
export const FILE_SIZE_HARD_LIMIT = 5 * 1024 * 1024 * 1024;

/**
 * Progress indicator delay (milliseconds)
 */
export const PROGRESS_INDICATOR_DELAY = 2000;

/**
 * Default confirmation text for import
 */
export const IMPORT_CONFIRMATION_TEXT = 'RESTORE';
