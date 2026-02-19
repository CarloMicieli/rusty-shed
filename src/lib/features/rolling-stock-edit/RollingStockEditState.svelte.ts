/**
 * RollingStockEditState
 *
 * Per-card reactive state for tracking the in-progress inline edit of a single
 * rolling stock field. Shared by InPlaceEdit instances within a RollingStockCard
 * so that only one field can be in the "saving" state at a time.
 */

export class RollingStockEditState {
  /** Which field identifier (e.g. "series_code", "road_number") is actively being edited, or null. */
  activeField = $state<string | null>(null);
  /** Value currently being edited / saved. */
  pendingValue = $state('');
  /** Whether a save is in progress. */
  isSaving = $state(false);
  /** Last error message from a failed save, or null. */
  lastError = $state<string | null>(null);
}
