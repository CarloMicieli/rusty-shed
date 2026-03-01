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

  const due = new Date(dueDate);
  due.setHours(0, 0, 0, 0);

  const diffTime = due.getTime() - today.getTime();
  const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));

  // Overdue: due date is in the past
  if (diffDays < 0) {
    return 'overdue';
  }

  // Warning: due within 7 days
  if (diffDays <= 7) {
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
