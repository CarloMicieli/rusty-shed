import { describe, it, expect, vi, beforeEach } from 'vitest';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}));

vi.mock('$lib/toaster', () => ({
  toaster: {
    loading: vi.fn().mockReturnValue('toast-id'),
    success: vi.fn(),
    error: vi.fn(),
    dismiss: vi.fn()
  }
}));

vi.mock('$lib/paraglide/messages.js', () => ({
  collection_toast_loading: () => 'Loading...',
  collection_toast_success: () => 'Success',
  collection_toast_error: () => 'Error',
  collection_toast_retry: () => 'Retry',
  purchase_dialog_success: () => 'Purchase recorded'
}));

import { invoke } from '@tauri-apps/api/core';
import { createWishlistState } from '$lib/features/wishlists/WishlistState.svelte';
import type { WishlistPreview } from '$lib/bindings';

const mockInvoke = vi.mocked(invoke);

// ─── helpers ──────────────────────────────────────────────────────────────

function makePreview(id: string, isDefault = false): WishlistPreview {
  return {
    id,
    name: `Wishlist ${id}`,
    notes: null,
    isDefault,
    count: 0n,
    updatedAt: '2026-01-01T00:00:00Z',
    totalValue: {}
  };
}

function makeItem(id: string, railwayModelId = 'model-1') {
  return {
    id,
    railwayModelId,
    priority: 'NORMAL',
    status: 'WANTED',
    addedDate: '2026-01-01',
    removedDate: null,
    notes: null,
    desiredPrice: null,
    purchasedPrice: null
  };
}

// ─── tests ────────────────────────────────────────────────────────────────

describe('WishlistState', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('starts with empty state', () => {
    const state = createWishlistState();
    expect(state.wishlists).toHaveLength(0);
    expect(state.activeWishlistId).toBeNull();
    expect(state.isLoading).toBe(false);
    expect(state.defaultWishlist).toBeNull();
    expect(state.activeWishlist).toBeNull();
    expect(state.wishlistItems).toHaveLength(0);
  });

  describe('$derived defaultWishlist', () => {
    it('resolves to the wishlist marked isDefault', async () => {
      const previews = [makePreview('w1'), makePreview('w2', true), makePreview('w3')];
      mockInvoke.mockResolvedValueOnce(previews);

      const state = createWishlistState();
      await state.fetchWishlists();

      expect(state.defaultWishlist?.id).toBe('w2');
    });

    it('returns null when no default wishlist is set', async () => {
      const previews = [makePreview('w1'), makePreview('w2')];
      mockInvoke.mockResolvedValueOnce(previews);

      const state = createWishlistState();
      await state.fetchWishlists();

      expect(state.defaultWishlist).toBeNull();
    });
  });

  describe('$derived activeWishlist', () => {
    it('resolves to the active wishlist by ID', async () => {
      const previews = [makePreview('w1'), makePreview('w2')];
      mockInvoke.mockResolvedValueOnce(previews);
      // selectWishlist will also load items via get_wishlist_by_id
      mockInvoke.mockResolvedValueOnce({ id: 'w2', name: 'Wishlist w2', items: [] });

      const state = createWishlistState();
      await state.fetchWishlists();
      await state.selectWishlist('w2');

      expect(state.activeWishlist?.id).toBe('w2');
    });

    it('returns null when no wishlist is selected', async () => {
      const state = createWishlistState();
      expect(state.activeWishlist).toBeNull();
    });
  });

  describe('$derived wishlistItems', () => {
    it('returns items for the active wishlist', async () => {
      const previews = [makePreview('w1')];
      const item = makeItem('i1');
      mockInvoke.mockResolvedValueOnce(previews);
      mockInvoke.mockResolvedValueOnce({ id: 'w1', name: 'Wishlist w1', items: [item] });

      const state = createWishlistState();
      await state.fetchWishlists();
      await state.selectWishlist('w1');

      expect(state.wishlistItems).toHaveLength(1);
      expect(state.wishlistItems[0].id).toBe('i1');
    });

    it('returns empty array when no wishlist is active', () => {
      const state = createWishlistState();
      expect(state.wishlistItems).toHaveLength(0);
    });
  });

  describe('fetchWishlists', () => {
    it('populates wishlists', async () => {
      const previews = [makePreview('w1'), makePreview('w2')];
      mockInvoke.mockResolvedValueOnce(previews);

      const state = createWishlistState();
      await state.fetchWishlists();

      expect(state.wishlists).toHaveLength(2);
    });

    it('auto-selects default wishlist as activeWishlistId', async () => {
      const previews = [makePreview('w1'), makePreview('w2', true)];
      mockInvoke.mockResolvedValueOnce(previews);

      const state = createWishlistState();
      await state.fetchWishlists();

      expect(state.activeWishlistId).toBe('w2');
    });

    it('handles fetch error gracefully', async () => {
      mockInvoke.mockRejectedValueOnce({ DatabaseError: 'Connection failed' });

      const state = createWishlistState();
      await state.fetchWishlists();

      expect(state.wishlists).toHaveLength(0);
      expect(state.isLoading).toBe(false);
    });
  });

  describe('selectWishlist', () => {
    it('sets activeWishlistId and loads items', async () => {
      const previews = [makePreview('w1')];
      mockInvoke.mockResolvedValueOnce(previews);
      mockInvoke.mockResolvedValueOnce({ id: 'w1', name: 'Wishlist w1', items: [] });

      const state = createWishlistState();
      await state.fetchWishlists();
      await state.selectWishlist('w1');

      expect(state.activeWishlistId).toBe('w1');
    });

    it('does not re-load items when already loaded', async () => {
      const previews = [makePreview('w1')];
      mockInvoke.mockResolvedValueOnce(previews);
      mockInvoke.mockResolvedValueOnce({ id: 'w1', name: 'Wishlist w1', items: [] });

      const state = createWishlistState();
      await state.fetchWishlists();
      await state.selectWishlist('w1');
      await state.selectWishlist('w1'); // second call — should not invoke again

      // Only 2 invocations: fetchWishlists + selectWishlist (first time)
      expect(mockInvoke).toHaveBeenCalledTimes(2);
    });
  });

  describe('createWishlist optimistic update', () => {
    it('adds wishlist optimistically and confirms on success', async () => {
      const created = makePreview('w-new');
      mockInvoke.mockResolvedValueOnce(created);

      const state = createWishlistState();
      const result = await state.createWishlist('New Wishlist');

      expect(state.wishlists).toHaveLength(1);
      expect(state.wishlists[0].id).toBe('w-new');
      expect(result).toBeTruthy();
    });

    it('reverts optimistic wishlist on failure', async () => {
      mockInvoke.mockRejectedValueOnce({ Unknown: 'Server error' });

      const state = createWishlistState();
      const result = await state.createWishlist('Bad Wishlist');

      expect(state.wishlists).toHaveLength(0);
      expect(result).toBeNull();
    });
  });

  describe('deleteWishlist optimistic update', () => {
    it('removes wishlist optimistically and confirms on success', async () => {
      const previews = [makePreview('w1'), makePreview('w2')];
      mockInvoke.mockResolvedValueOnce(previews);
      mockInvoke.mockResolvedValueOnce(null); // delete_wishlist

      const state = createWishlistState();
      await state.fetchWishlists();
      await state.deleteWishlist('w1');

      expect(state.wishlists).toHaveLength(1);
      expect(state.wishlists[0].id).toBe('w2');
    });

    it('reverts deleted wishlist on failure', async () => {
      const previews = [makePreview('w1'), makePreview('w2')];
      mockInvoke.mockResolvedValueOnce(previews);
      mockInvoke.mockRejectedValueOnce({ DatabaseError: 'Constraint error' });

      const state = createWishlistState();
      await state.fetchWishlists();
      await state.deleteWishlist('w1');

      expect(state.wishlists).toHaveLength(2);
    });
  });

  describe('addItem optimistic update', () => {
    it('adds item optimistically then replaces with real data', async () => {
      const realItem = makeItem('real-id', 'model-42');
      mockInvoke.mockResolvedValueOnce(realItem);

      const state = createWishlistState();
      const result = await state.addItem('w1', 'model-42');

      expect(state.itemsByWishlist['w1']).toHaveLength(1);
      expect(result).toBeTruthy();
    });

    it('reverts item on failure', async () => {
      mockInvoke.mockRejectedValueOnce({ Unknown: 'Not allowed' });

      const state = createWishlistState();
      await state.addItem('w1', 'model-42');

      expect(state.itemsByWishlist['w1'] ?? []).toHaveLength(0);
    });
  });

  describe('removeItem optimistic update', () => {
    it('removes item optimistically and confirms on success', async () => {
      const previews = [makePreview('w1')];
      const item = makeItem('i1');
      mockInvoke.mockResolvedValueOnce(previews);
      mockInvoke.mockResolvedValueOnce({ id: 'w1', name: 'Wishlist w1', items: [item] });
      mockInvoke.mockResolvedValueOnce(null); // remove_from_wishlist

      const state = createWishlistState();
      await state.fetchWishlists();
      await state.selectWishlist('w1');
      await state.removeItem('w1', 'i1');

      expect(state.itemsByWishlist['w1']).toHaveLength(0);
    });

    it('reverts item removal on failure', async () => {
      const previews = [makePreview('w1')];
      const item = makeItem('i1');
      mockInvoke.mockResolvedValueOnce(previews);
      mockInvoke.mockResolvedValueOnce({ id: 'w1', name: 'Wishlist w1', items: [item] });
      mockInvoke.mockRejectedValueOnce({ DatabaseError: 'Removal failed' });

      const state = createWishlistState();
      await state.fetchWishlists();
      await state.selectWishlist('w1');
      await state.removeItem('w1', 'i1');

      expect(state.itemsByWishlist['w1']).toHaveLength(1);
    });
  });

  describe('_normalizeItem', () => {
    it('normalizes camelCase fields', () => {
      const state = createWishlistState();
      const normalized = state._normalizeItem({
        id: 'i1',
        railwayModelId: 'model-1',
        priority: 'HIGH',
        status: 'WANTED',
        addedDate: '2026-01-01',
        removedDate: null,
        notes: 'test',
        desiredPrice: null,
        purchasedPrice: null
      });
      expect(normalized.id).toBe('i1');
      expect(normalized.railwayModelId).toBe('model-1');
      expect(normalized.priority).toBe('HIGH');
    });

    it('normalizes snake_case fields from backend', () => {
      const state = createWishlistState();
      const normalized = state._normalizeItem({
        id: 'i2',
        railway_model_id: 'model-2',
        priority: 'NORMAL',
        status: 'WANTED',
        added_date: '2026-02-01',
        removed_date: null,
        notes: null,
        desired_price: null,
        purchased_price: null
      });
      expect(normalized.id).toBe('i2');
      expect(normalized.railwayModelId).toBe('model-2');
      expect(normalized.addedDate).toBe('2026-02-01');
    });
  });
});
