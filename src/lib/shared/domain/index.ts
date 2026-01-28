/**
 * Shared Domain - Public API
 *
 * This module exports all shared domain types and utilities.
 */

// Error handling
export type { NormalizedError, SafeResult, ErrorKind } from './errors';
export {
  getErrorMessage,
  getToastMessage,
  isValidationError,
  isNotFoundError,
  isDatabaseError,
  isRetryableError
} from './errors';
