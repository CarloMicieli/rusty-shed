/**
 * Normalized error types for the application.
 *
 * These types provide a consistent error interface across the application,
 * abstracting away the underlying Rust CommandError structure.
 */

/**
 * Error kinds that map to Rust CommandError variants.
 */
export type ErrorKind = 'database' | 'not_found' | 'validation' | 'permission_denied' | 'unknown';

/**
 * Normalized error structure used throughout the application.
 *
 * This provides a consistent interface for error handling across all UI components,
 * regardless of the underlying error source (Rust IPC, JavaScript, etc.).
 */
export interface NormalizedError {
  /**
   * The category of error that occurred.
   */
  kind: ErrorKind;

  /**
   * Human-readable error message.
   */
  message: string;

  /**
   * Whether this error is retryable.
   */
  retryable: boolean;

  /**
   * Unique error identifier for log correlation (only present on unknown errors).
   */
  errorId?: string;

  /**
   * Field-specific validation errors.
   * Only populated for validation errors.
   * Key is the field name, value is the error message for that field.
   */
  fields?: Record<string, string>;
}

/**
 * Result type for operations that can fail with a normalized error.
 */
export type SafeResult<T> = { ok: true; data: T } | { ok: false; error: NormalizedError };

/**
 * Extract a user-friendly error message from a NormalizedError.
 *
 * For validation errors with multiple fields, creates a summary message.
 *
 * @param error - The normalized error to extract a message from
 * @returns A human-readable error message
 */
export function getErrorMessage(error: NormalizedError): string {
  if (error.kind === 'validation' && error.fields && Object.keys(error.fields).length > 0) {
    const fieldErrors = Object.entries(error.fields)
      .map(([field, msg]) => `${field}: ${msg}`)
      .join(', ');
    return `${error.message} (${fieldErrors})`;
  }
  return error.message;
}

/**
 * Get a shortened error message suitable for toast notifications.
 *
 * @param error - The normalized error
 * @returns A concise error message
 */
export function getToastMessage(error: NormalizedError): string {
  switch (error.kind) {
    case 'not_found':
      return 'Resource not found';
    case 'validation':
      return 'Validation failed';
    case 'permission_denied':
      return 'Permission denied';
    case 'database':
      return 'Database error occurred';
    case 'unknown':
    default:
      return 'An unexpected error occurred';
  }
}

/**
 * Check if an error is a validation error.
 */
export function isValidationError(error: NormalizedError): error is NormalizedError & {
  kind: 'validation';
  fields: Record<string, string>;
} {
  return error.kind === 'validation';
}

/**
 * Check if an error is a not found error.
 */
export function isNotFoundError(error: NormalizedError): boolean {
  return error.kind === 'not_found';
}

/**
 * Check if an error is a database error.
 */
export function isDatabaseError(error: NormalizedError): boolean {
  return error.kind === 'database';
}

/**
 * Check if an error is retryable.
 */
export function isRetryableError(error: NormalizedError): boolean {
  return error.retryable;
}
