/**
 * UI-specific types for the maintenance feature.
 */

/**
 * Urgency level for maintenance cards based on due date.
 */
export type UrgencyLevel = 'overdue' | 'warning' | 'normal';

/**
 * UI state for maintenance card creation form.
 */
export interface MaintenanceCardFormState {
  ownedRollingStockId: string | null;
  isSubmitting: boolean;
  error: string | null;
}

/**
 * UI state for maintenance event form.
 */
export interface MaintenanceEventFormState {
  maintenanceCardId: string | null;
  datePerformed: string;
  maintenanceType: string | null;
  notes: string | null;
  isSubmitting: boolean;
  error: string | null;
}
