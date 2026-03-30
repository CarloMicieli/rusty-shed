/**
 * Urgency calculation utilities for maintenance cards.
 */

import type { UrgencyLevel } from '../types';

/**
 * Calculate urgency level based on the due date.
 *
 * @param dueDate - The next maintenance due date (YYYY-MM-DD format or null)
 * @returns UrgencyLevel: 'overdue', 'warning', or 'normal'
 */
export function getUrgencyLevel(dueDate: string | null): UrgencyLevel {
  if (!dueDate) {
    return 'normal';
  }

  const today = new Date();
  today.setHours(0, 0, 0, 0); // Reset time for accurate date comparison

  // Parse as local date to avoid UTC-offset shifting (YYYY-MM-DD → local midnight)
  const [year, month, day] = dueDate.split('-').map(Number);
  const due = new Date(year, month - 1, day);

  // Overdue: due date is strictly before today (direct comparison avoids DST rounding errors)
  if (due < today) {
    return 'overdue';
  }

  // Warning: due within 7 days (use ms comparison; DST shifts ≤1h safely within the 7-day window)
  const diffMs = due.getTime() - today.getTime();
  if (diffMs <= 7 * 24 * 60 * 60 * 1000) {
    return 'warning';
  }

  // Normal: due beyond 7 days
  return 'normal';
}

/**
 * Get Tailwind CSS color classes for an urgency level.
 *
 * @param urgency - The urgency level
 * @returns Object with background and text color classes
 */
export function getUrgencyColors(urgency: UrgencyLevel): {
  bg: string;
  text: string;
  border: string;
} {
  switch (urgency) {
    case 'overdue':
      return {
        bg: 'bg-error-500/10',
        text: 'text-error-700',
        border: 'border-error-500'
      };
    case 'warning':
      return {
        bg: 'bg-warning-500/10',
        text: 'text-warning-700',
        border: 'border-warning-500'
      };
    case 'normal':
      return {
        bg: 'bg-muted',
        text: 'text-muted-foreground',
        border: 'border-border'
      };
  }
}
