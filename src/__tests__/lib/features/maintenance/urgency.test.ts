import { describe, it, expect } from 'vitest';
import { getUrgencyLevel, getUrgencyColors } from '$lib/features/maintenance/utils/urgency';

// Helper: produce a YYYY-MM-DD date string relative to today (local timezone)
function dateOffset(days: number): string {
  const d = new Date();
  d.setHours(0, 0, 0, 0);
  d.setDate(d.getDate() + days);
  const year = d.getFullYear();
  const month = String(d.getMonth() + 1).padStart(2, '0');
  const day = String(d.getDate()).padStart(2, '0');
  return `${year}-${month}-${day}`;
}

describe('getUrgencyLevel', () => {
  it('returns "normal" when dueDate is null', () => {
    expect(getUrgencyLevel(null)).toBe('normal');
  });

  it('returns "overdue" when due date is in the past', () => {
    expect(getUrgencyLevel(dateOffset(-1))).toBe('overdue');
    expect(getUrgencyLevel(dateOffset(-30))).toBe('overdue');
    expect(getUrgencyLevel(dateOffset(-365))).toBe('overdue');
  });

  it('returns "warning" when due date is today (0 days)', () => {
    expect(getUrgencyLevel(dateOffset(0))).toBe('warning');
  });

  it('returns "warning" when due date is within 7 days', () => {
    expect(getUrgencyLevel(dateOffset(1))).toBe('warning');
    expect(getUrgencyLevel(dateOffset(7))).toBe('warning');
  });

  it('returns "normal" when due date is more than 7 days away', () => {
    expect(getUrgencyLevel(dateOffset(8))).toBe('normal');
    expect(getUrgencyLevel(dateOffset(30))).toBe('normal');
    expect(getUrgencyLevel(dateOffset(365))).toBe('normal');
  });
});

describe('getUrgencyColors', () => {
  it('returns error color classes for "overdue"', () => {
    const colors = getUrgencyColors('overdue');
    expect(colors.bg).toContain('error');
    expect(colors.text).toContain('error');
    expect(colors.border).toContain('error');
  });

  it('returns warning color classes for "warning"', () => {
    const colors = getUrgencyColors('warning');
    expect(colors.bg).toContain('warning');
    expect(colors.text).toContain('warning');
    expect(colors.border).toContain('warning');
  });

  it('returns muted/border color classes for "normal"', () => {
    const colors = getUrgencyColors('normal');
    expect(colors.bg).toContain('muted');
    expect(colors.text).toContain('muted');
    expect(colors.border).toContain('border');
  });

  it('returns objects with bg, text, and border keys for all levels', () => {
    for (const level of ['overdue', 'warning', 'normal'] as const) {
      const colors = getUrgencyColors(level);
      expect(colors).toHaveProperty('bg');
      expect(colors).toHaveProperty('text');
      expect(colors).toHaveProperty('border');
    }
  });
});
