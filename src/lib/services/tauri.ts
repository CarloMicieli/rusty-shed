/**
 * @deprecated Use `$lib/shared/services/TauriAdapter` directly for new code.
 *
 * This module re-exports the canonical IPC adapter so that existing internal
 * service files continue to resolve without modification.
 */

export { safeInvoke, invokeOrThrow, safeInvokeWithRetry, safeCommand } from '$lib/shared/services/TauriAdapter';
