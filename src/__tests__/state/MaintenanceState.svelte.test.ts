import { describe, it, expect, vi, beforeEach } from 'vitest';
import { flushSync } from 'svelte';

vi.mock('$lib/shared/services/TauriAdapter', () => ({
  safeInvoke: vi.fn()
}));

import { safeInvoke } from '$lib/shared/services/TauriAdapter';
import MaintenanceState from '$lib/features/maintenance/MaintenanceState.svelte';
import { MaintenanceService } from '$lib/features/maintenance/services/MaintenanceService';
import type { MaintenanceCardView } from '$lib/bindings';

const mockSafeInvoke = vi.mocked(safeInvoke);

// ─── helpers ──────────────────────────────────────────────────────────────

function makeCard(id: string, overrides: Partial<MaintenanceCardView> = {}): MaintenanceCardView {
  return {
    id: { value: id } as unknown as MaintenanceCardView['id'],
    rollingStockName: `RS-${id}`,
    lastMaintenanceDate: null,
    nextDueDate: null,
    intervalDays: 90,
    isOverdue: false,
    daysSinceLast: null,
    daysUntilDue: null,
    maintenanceCount: 0,
    ...overrides
  } as unknown as MaintenanceCardView;
}

// ─── tests ────────────────────────────────────────────────────────────────

describe('MaintenanceState', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('starts with empty state', () => {
    const state = new MaintenanceState();
    expect(state.cards).toHaveLength(0);
    expect(state.isLoading).toBe(false);
    expect(state.error).toBeNull();
    expect(state.hasCards).toBe(false);
  });

  describe('loadDashboard', () => {
    it('populates cards on success', async () => {
      const cards = [makeCard('mc-1'), makeCard('mc-2')];
      mockSafeInvoke.mockResolvedValueOnce({ ok: true, data: cards });

      const state = new MaintenanceState();
      await state.loadDashboard();

      expect(state.cards).toHaveLength(2);
      expect(state.hasCards).toBe(true);
      expect(state.isLoading).toBe(false);
      expect(state.error).toBeNull();
    });

    it('sets error on failure', async () => {
      mockSafeInvoke.mockResolvedValueOnce({
        ok: false,
        error: { kind: 'database', message: 'DB error', retryable: true }
      });

      const state = new MaintenanceState();
      await state.loadDashboard();

      expect(state.cards).toHaveLength(0);
      expect(state.error).toBe('DB error');
      expect(state.isLoading).toBe(false);
    });

    it('sets isLoading=false after unexpected exception', async () => {
      mockSafeInvoke.mockRejectedValueOnce(new Error('Network timeout'));

      const state = new MaintenanceState();
      await state.loadDashboard();

      expect(state.isLoading).toBe(false);
      expect(state.error).toBe('Network timeout');
    });

    it('clears previous error on successful reload', async () => {
      // First call fails
      mockSafeInvoke.mockResolvedValueOnce({
        ok: false,
        error: { kind: 'unknown', message: 'Oops', retryable: false }
      });
      // Second call succeeds
      mockSafeInvoke.mockResolvedValueOnce({ ok: true, data: [makeCard('mc-1')] });
      // Third call (retry's loadDashboard inside createMaintenanceCard) not needed here

      const state = new MaintenanceState();
      await state.loadDashboard();
      expect(state.error).toBe('Oops');

      await state.retry();
      expect(state.error).toBeNull();
      expect(state.cards).toHaveLength(1);
    });
  });

  describe('hasCards', () => {
    it('returns true when cards are loaded', async () => {
      mockSafeInvoke.mockResolvedValueOnce({ ok: true, data: [makeCard('mc-1')] });
      const state = new MaintenanceState();
      await state.loadDashboard();
      expect(state.hasCards).toBe(true);
    });

    it('returns false when cards array is empty', async () => {
      mockSafeInvoke.mockResolvedValueOnce({ ok: true, data: [] });
      const state = new MaintenanceState();
      await state.loadDashboard();
      expect(state.hasCards).toBe(false);
    });
  });

  describe('createMaintenanceCard', () => {
    it('creates card and refreshes dashboard', async () => {
      const cardId = { value: 'card-new' } as unknown as import('$lib/bindings').MaintenanceCardId;
      const dashboardCards = [makeCard('mc-1')];

      mockSafeInvoke
        .mockResolvedValueOnce({ ok: true, data: cardId }) // add_maintenance_card
        .mockResolvedValueOnce({ ok: true, data: dashboardCards }); // get_maintenance_dashboard

      const state = new MaintenanceState();
      await state.createMaintenanceCard({
        value: 'rs-1'
      } as unknown as import('$lib/bindings').OwnedRollingStockId);

      expect(state.cards).toHaveLength(1);
    });

    it('throws error when creation fails', async () => {
      mockSafeInvoke.mockResolvedValueOnce({
        ok: false,
        error: { kind: 'validation', message: 'Invalid RS ID', retryable: false }
      });

      const state = new MaintenanceState();
      await expect(
        state.createMaintenanceCard({
          value: 'bad-id'
        } as unknown as import('$lib/bindings').OwnedRollingStockId)
      ).rejects.toThrow('Invalid RS ID');
    });
  });

  describe('addMaintenanceEvent', () => {
    it('adds event and refreshes dashboard', async () => {
      const dashboardCards = [makeCard('mc-1')];

      mockSafeInvoke
        .mockResolvedValueOnce({ ok: true, data: null }) // add_maintenance_event
        .mockResolvedValueOnce({ ok: true, data: dashboardCards }); // get_maintenance_dashboard

      const state = new MaintenanceState();
      await state.addMaintenanceEvent({
        id: 'ev-1',
        maintenanceCardId: 'mc-1',
        datePerformed: '2026-01-15',
        maintenanceType: null,
        notes: null
      });

      expect(state.cards).toHaveLength(1);
    });

    it('throws error when event fails', async () => {
      mockSafeInvoke.mockResolvedValueOnce({
        ok: false,
        error: { kind: 'not_found', message: 'Card not found', retryable: false }
      });

      const state = new MaintenanceState();
      await expect(
        state.addMaintenanceEvent({
          id: 'ev-bad',
          maintenanceCardId: 'bad-id',
          datePerformed: '2026-01-15',
          maintenanceType: null,
          notes: null
        })
      ).rejects.toThrow('Card not found');
    });
  });

  describe('retry', () => {
    it('calls loadDashboard again', async () => {
      mockSafeInvoke.mockResolvedValue({ ok: true, data: [] });
      const state = new MaintenanceState();
      await state.retry();
      expect(mockSafeInvoke).toHaveBeenCalledWith('get_maintenance_dashboard');
    });
  });

  describe('with injected service', () => {
    it('accepts a custom service via constructor', async () => {
      const mockService = {
        getDashboard: vi.fn().mockResolvedValue([makeCard('custom-1')])
      };
      const state = new MaintenanceState(mockService as unknown as MaintenanceService);
      await state.loadDashboard();
      expect(state.cards).toHaveLength(1);
      expect(mockService.getDashboard).toHaveBeenCalledOnce();
    });
  });
});
