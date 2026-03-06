/**
 * Error ID generation utilities.
 *
 * Provides session-scoped unique identifiers for error correlation between
 * the UI and backend logs. Format: `ERR-NNNN-X` where NNNN is 1000–9999
 * and X is an uppercase letter A–Z.
 */

/**
 * Generate a unique Error ID.
 *
 * @returns A string in the format `ERR-NNNN-X`
 */
export function generateErrorId(): string {
  const n = Math.floor(Math.random() * 9000) + 1000;
  const c = String.fromCharCode(65 + Math.floor(Math.random() * 26));
  return `ERR-${String(n)}-${c}`;
}

/**
 * Type guard to check if a value is a valid Error ID.
 *
 * @param value - The value to check
 * @returns `true` if the value matches the `ERR-NNNN-X` format
 */
export function isErrorId(value: unknown): value is string {
  return typeof value === 'string' && /^ERR-\d{4}-[A-Z]$/.test(value);
}
