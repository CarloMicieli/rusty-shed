import { describe, it, expect, vi, beforeEach } from 'vitest';

vi.mock('$lib/shared/services/TauriAdapter', () => ({
  safeInvoke: vi.fn()
}));

import { safeInvoke } from '$lib/shared/services/TauriAdapter';
import MaintenanceDetailState from '$lib/features/maintenance/MaintenanceDetailState.svelte';
import type { MaintenanceCardView } from '$lib/bindings';

const mockSafeInvoke = vi.mocked(safeInvoke);

function makeCard(id: string): MaintenanceCardView {
  return {
    id: id as MaintenanceCardView['id'],
    ownedRollingStockId:
      'trn:owned-rolling-stock:test' as MaintenanceCardView['ownedRollingStockId'],
    lastMaintenanceDate: null,
    nextMaintenanceDate: null,
    events: [],
    displayInfo: null
  };
}

describe('MaintenanceDetailState', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('starts with null card', () => {
    const state = new MaintenanceDetailState();
    expect(state.card).toBeNull();
    expect(state.isLoading).toBe(false);
    expect(state.error).toBeNull();
  });

  describe('loadCard', () => {
    it('populates card on success', async () => {
      const card = makeCard('mc-1');
      mockSafeInvoke.mockResolvedValueOnce({ ok: true, data: card });

      const state = new MaintenanceDetailState();
      await state.loadCard('mc-1');

      expect(state.card).toEqual(card);
      expect(state.isLoading).toBe(false);
      expect(state.error).toBeNull();
    });

    it('sets card to null when not found', async () => {
      mockSafeInvoke.mockResolvedValueOnce({ ok: true, data: null });

      const state = new MaintenanceDetailState();
      await state.loadCard('missing-id');

      expect(state.card).toBeNull();
    });

    it('sets error on failure', async () => {
      mockSafeInvoke.mockResolvedValueOnce({
        ok: false,
        error: { kind: 'database', message: 'DB error', retryable: true }
      });

      const state = new MaintenanceDetailState();
      await state.loadCard('mc-1');

      expect(state.card).toBeNull();
      expect(state.error).toBe('DB error');
    });
  });

  describe('addEvent', () => {
    it('optimistically prepends event before backend resolves', async () => {
      const card = makeCard('trn:maintenance-card:test-card');
      mockSafeInvoke.mockResolvedValueOnce({ ok: true, data: card });

      const state = new MaintenanceDetailState();
      await state.loadCard('trn:maintenance-card:test-card');
      expect(state.card!.events).toHaveLength(0);

      // Mock add_maintenance_event to be pending (never resolves immediately)
      let resolveAddEvent!: (v: unknown) => void;
      const pendingAdd = new Promise<Awaited<ReturnType<typeof safeInvoke>>>((r) => {
        resolveAddEvent = r as (v: unknown) => void;
      });
      mockSafeInvoke.mockImplementationOnce(() => pendingAdd);

      const eventArgs = {
        id: 'new-event-id',
        maintenanceCardId: 'trn:maintenance-card:test-card',
        datePerformed: '2026-03-05',
        maintenanceType: null,
        notes: null
      };

      // Start addEvent but do NOT await — optimistic update should happen synchronously
      const addPromise = state.addEvent(eventArgs);

      // Optimistic update: event should already be prepended
      expect(state.card!.events).toHaveLength(1);
      expect(state.card!.events[0].id).toBe('new-event-id');

      // Resolve the backend call to clean up
      resolveAddEvent({ ok: null, data: null });
      await addPromise.catch(() => {});
    });

    it('does nothing when card is null', async () => {
      const state = new MaintenanceDetailState();
      // No card loaded — addEvent should be a no-op
      await state.addEvent({
        id: 'ev-1',
        maintenanceCardId: 'mc-1',
        datePerformed: '2026-03-05',
        maintenanceType: null,
        notes: null
      });
      expect(mockSafeInvoke).not.toHaveBeenCalled();
    });
  });
});
