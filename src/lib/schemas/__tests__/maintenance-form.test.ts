import { describe, expect, it, vi } from 'vitest';

vi.mock('$lib/paraglide/messages.js', () => ({
  error_required: () => 'Required'
}));

import { maintenanceSchema } from '../maintenance-form';

describe('maintenanceSchema', () => {
  it('accepts a complete valid maintenance event', () => {
    const result = maintenanceSchema.safeParse({
      selectedLocoId: 'rolling-stock-1',
      datePerformed: '2026-11-27',
      maintenanceType: 'CLEANING',
      notes: 'Lubricated axles',
      initialCondition: 'Good',
      nextMaintenanceDate: '2026-12-27'
    });

    expect(result.success).toBe(true);
  });

  it('requires a selected rolling stock', () => {
    for (const selectedLocoId of [null, '', '   ']) {
      const result = maintenanceSchema.safeParse({
        selectedLocoId,
        datePerformed: '2026-11-27'
      });

      expect(result.success).toBe(false);
      if (!result.success) {
        expect(result.error.issues[0].message).toBe('Required');
      }
    }
  });

  it('requires a date even though optional fields have defaults', () => {
    const result = maintenanceSchema.safeParse({
      selectedLocoId: 'rolling-stock-1',
      datePerformed: ''
    });

    expect(result.success).toBe(false);
  });

  it('defaults optional fields to null or empty strings', () => {
    const result = maintenanceSchema.parse({
      selectedLocoId: 'rolling-stock-1',
      datePerformed: '2026-11-27'
    });

    expect(result).toEqual({
      selectedLocoId: 'rolling-stock-1',
      datePerformed: '2026-11-27',
      maintenanceType: null,
      notes: '',
      initialCondition: '',
      nextMaintenanceDate: null
    });
  });

  it('rejects non-string rolling-stock and date values', () => {
    expect(
      maintenanceSchema.safeParse({
        selectedLocoId: 42,
        datePerformed: '2026-11-27'
      }).success
    ).toBe(false);
    expect(
      maintenanceSchema.safeParse({
        selectedLocoId: 'rolling-stock-1',
        datePerformed: 20261127
      }).success
    ).toBe(false);
  });

  it('preserves meaningful maintenance notes and next-date values', () => {
    const result = maintenanceSchema.parse({
      selectedLocoId: 'rolling-stock-1',
      datePerformed: '2026-11-27',
      maintenanceType: 'REPRESENTATIVE',
      notes: 'Changed carbon brushes',
      initialCondition: 'Intermittent',
      nextMaintenanceDate: '2027-01-15'
    });

    expect(result.notes).toBe('Changed carbon brushes');
    expect(result.initialCondition).toBe('Intermittent');
    expect(result.nextMaintenanceDate).toBe('2027-01-15');
  });
});
