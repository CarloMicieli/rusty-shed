/**
 * Settings types
 */

export type MeasureUnit = 'Metric' | 'Imperial';

export type EntityStatusBadge =
  | { kind: 'protected' }
  | { kind: 'in-use'; usageCount: number }
  | { kind: 'unused' };

export function deriveEntityStatusBadge(
  isSystemSeeded: boolean,
  usageCount: number
): EntityStatusBadge {
  if (isSystemSeeded) {
    return { kind: 'protected' };
  }

  if (usageCount > 0) {
    return { kind: 'in-use', usageCount };
  }

  return { kind: 'unused' };
}
