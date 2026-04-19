import { describe, it, expect, vi, beforeEach } from 'vitest';

// Mock TauriAdapter before importing the service
vi.mock('$lib/shared/services/TauriAdapter', () => ({
  safeInvoke: vi.fn()
}));

import { safeInvoke } from '$lib/shared/services/TauriAdapter';
import { MaintenanceService } from '$lib/features/maintenance/services/MaintenanceService';
import type { MaintenanceCardView, MaintenanceCardId, AddMaintenanceArgs } from '$lib/bindings';

const mockSafeInvoke = vi.mocked(safeInvoke);

// ─── helpers ──────────────────────────────────────────────────────────────

function makeCard(id: string, overrides: Partial<MaintenanceCardView> = {}): MaintenanceCardView {
  return {
    id: id as MaintenanceCardId,
    ownedRollingStockId: `rs-${id}`,
    lastMaintenanceDate: null,
    nextMaintenanceDate: null,
    events: [],
    displayInfo: null,
    ...overrides
  } as unknown as MaintenanceCardView;
}

function makeEventArgs(overrides: Partial<AddMaintenanceArgs> = {}): AddMaintenanceArgs {
  return {
    maintenanceCardId: 'mc-1',
    datePerformed: '2026-03-01',
    maintenanceType: null,
    notes: null,
    nextMaintenanceDate: null,
    ...overrides
  };
}

// ─── tests ────────────────────────────────────────────────────────────────

describe('MaintenanceService', () => {
  let service: MaintenanceService;

  beforeEach(() => {
    vi.clearAllMocks();
    service = new MaintenanceService();
  });

  describe('getDashboard', () => {
    it('returns array of MaintenanceCardView on success', async () => {
      const cards = [makeCard('mc-1'), makeCard('mc-2')];
      mockSafeInvoke.mockResolvedValueOnce({ ok: true, data: cards });

      const result = await service.getDashboard();

      expect(result).toEqual(cards);
      expect(result).toHaveLength(2);
    });

    it('calls safeInvoke with the correct command', async () => {
      mockSafeInvoke.mockResolvedValueOnce({ ok: true, data: [] });

      await service.getDashboard();

      expect(mockSafeInvoke).toHaveBeenCalledWith('get_maintenance_dashboard');
    });

    it('returns an empty array when dashboard has no cards', async () => {
      mockSafeInvoke.mockResolvedValueOnce({ ok: true, data: [] });

      const result = await service.getDashboard();

      expect(result).toEqual([]);
    });

    it('throws when safeInvoke returns ok:false', async () => {
      mockSafeInvoke.mockResolvedValueOnce({
        ok: false,
        error: { kind: 'database', message: 'DB connection failed', retryable: true }
      });

      await expect(service.getDashboard()).rejects.toThrow('DB connection failed');
    });

    it('throws with the error message from a not_found response', async () => {
      mockSafeInvoke.mockResolvedValueOnce({
        ok: false,
        error: { kind: 'not_found', message: 'Dashboard not found', retryable: false }
      });

      await expect(service.getDashboard()).rejects.toThrow('Dashboard not found');
    });
  });

  describe('createCard', () => {
    it('returns the new MaintenanceCardId on success', async () => {
      const cardId = 'mc-new-123' as MaintenanceCardId;
      mockSafeInvoke.mockResolvedValueOnce({ ok: true, data: cardId });

      const result = await service.createCard('rs-42');

      expect(result).toBe(cardId);
    });

    it('calls safeInvoke with correct command and ownedRollingStockId', async () => {
      const cardId = 'mc-new-123' as MaintenanceCardId;
      mockSafeInvoke.mockResolvedValueOnce({ ok: true, data: cardId });

      await service.createCard('rs-42');

      expect(mockSafeInvoke).toHaveBeenCalledWith('add_maintenance_card', {
        ownedRollingStockId: 'rs-42'
      });
    });

    it('throws when safeInvoke returns ok:false', async () => {
      mockSafeInvoke.mockResolvedValueOnce({
        ok: false,
        error: { kind: 'validation', message: 'Invalid rolling stock ID', retryable: false }
      });

      await expect(service.createCard('bad-id')).rejects.toThrow('Invalid rolling stock ID');
    });

    it('throws BusinessRule errors by message', async () => {
      mockSafeInvoke.mockResolvedValueOnce({
        ok: false,
        error: { kind: 'unknown', message: 'Card already exists', retryable: false }
      });

      await expect(service.createCard('rs-42')).rejects.toThrow('Card already exists');
    });
  });

  describe('addEvent', () => {
    it('resolves without returning a value on success', async () => {
      mockSafeInvoke.mockResolvedValueOnce({ ok: true, data: null });

      const args = makeEventArgs();
      const result = await service.addEvent(args);

      expect(result).toBeUndefined();
    });

    it('calls safeInvoke with correct command and input args', async () => {
      mockSafeInvoke.mockResolvedValueOnce({ ok: true, data: null });

      const args = makeEventArgs({
        maintenanceCardId: 'mc-99',
        datePerformed: '2026-02-15',
        maintenanceType: 'CLEANING',
        notes: 'Lubricated axles'
      });
      await service.addEvent(args);

      expect(mockSafeInvoke).toHaveBeenCalledWith('add_maintenance_event', { input: args });
    });

    it('passes null maintenanceType and notes correctly', async () => {
      mockSafeInvoke.mockResolvedValueOnce({ ok: true, data: null });

      const args = makeEventArgs({ maintenanceType: null, notes: null });
      await service.addEvent(args);

      const callArgs = mockSafeInvoke.mock.calls[0][1] as { input: AddMaintenanceArgs };
      expect(callArgs.input.maintenanceType).toBeNull();
      expect(callArgs.input.notes).toBeNull();
    });

    it('throws when safeInvoke returns ok:false', async () => {
      mockSafeInvoke.mockResolvedValueOnce({
        ok: false,
        error: { kind: 'not_found', message: 'Card not found', retryable: false }
      });

      await expect(service.addEvent(makeEventArgs())).rejects.toThrow('Card not found');
    });
  });
});
