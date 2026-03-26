import { describe, it, expect, vi, beforeEach } from 'vitest';

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }));
vi.mock('$lib/toaster', () => ({
  toaster: {
    loading: vi.fn(),
    success: vi.fn(),
    error: vi.fn(),
    dismiss: vi.fn()
  }
}));
vi.mock('$lib/paraglide/messages.js', () => ({
  collection_toast_loading: () => 'Loading...',
  collection_toast_success: () => 'Success',
  collection_toast_retry: () => 'Retry',
  collection_toast_error: () => 'Error'
}));

import { WishlistService } from '$lib/features/wishlists/services/WishlistService.svelte';
import type { WishlistPreview, WishlistItem } from '$lib/bindings';
import { invoke } from '@tauri-apps/api/core';
import { toaster } from '$lib/toaster';

const mockInvoke = vi.mocked(invoke);
const mockToaster = vi.mocked(toaster);

function setupInvokeMock(handlers: Record<string, () => unknown>): void {
  mockInvoke.mockImplementation(async (cmd) => {
    const key = String(cmd);
    if (key in handlers) return handlers[key]() as never;
    throw new Error(`Unmocked command: ${key}`);
  });
}

function makeWishlist(overrides: Partial<WishlistPreview> = {}): WishlistPreview {
  return {
    id: 'wl-1',
    name: 'My Wishlist',
    notes: null,
    isDefault: false,
    count: 0n,
    updatedAt: '2024-01-01T00:00:00Z',
    totalValue: {},
    ...overrides
  } as unknown as WishlistPreview;
}

function makeItem(overrides: Partial<WishlistItem> = {}): WishlistItem {
  return {
    id: 'item-1',
    name: 'Test Item',
    railwayModelId: 'trn:railway-model:acme:1234',
    priority: 'NORMAL',
    status: 'WANTED',
    addedDate: '2024-01-01',
    removedDate: null,
    notes: null,
    desiredPrice: null,
    purchasedPrice: null,
    ...overrides
  } as unknown as WishlistItem;
}

describe('WishlistService', () => {
  let service: WishlistService;

  beforeEach(() => {
    vi.resetAllMocks();
    service = new WishlistService();
  });

  // ── Initial state ──────────────────────────────────────────────────────────

  describe('initial state', () => {
    it('has empty wishlists', () => {
      expect(service.wishlists).toEqual([]);
    });

    it('has empty itemsByWishlist', () => {
      expect(service.itemsByWishlist).toEqual({});
    });

    it('has null activeWishlistId', () => {
      expect(service.activeWishlistId).toBeNull();
    });

    it('has isLoading false', () => {
      expect(service.isLoading).toBe(false);
    });

    it('defaultWishlist is null when no wishlists', () => {
      expect(service.defaultWishlist).toBeNull();
    });

    it('activeWishlist is null when no active id', () => {
      expect(service.activeWishlist).toBeNull();
    });

    it('wishlistItems is empty when no active id', () => {
      expect(service.wishlistItems).toEqual([]);
    });
  });

  // ── fetchWishlists ─────────────────────────────────────────────────────────

  describe('fetchWishlists', () => {
    it('sets wishlists on success', async () => {
      const lists = [makeWishlist({ id: 'wl-1', name: 'Main' })];
      setupInvokeMock({ get_wishlists: () => lists });

      await service.fetchWishlists();

      expect(service.wishlists).toEqual(lists);
    });

    it('sets isLoading to false after success', async () => {
      setupInvokeMock({ get_wishlists: () => [] });

      await service.fetchWishlists();

      expect(service.isLoading).toBe(false);
    });

    it('sets isLoading to false after failure', async () => {
      mockInvoke.mockRejectedValue(new Error('network error'));

      await service.fetchWishlists();

      expect(service.isLoading).toBe(false);
    });

    it('auto-selects default wishlist when none active', async () => {
      const lists = [
        makeWishlist({ id: 'wl-1', isDefault: false }),
        makeWishlist({ id: 'wl-2', isDefault: true })
      ];
      setupInvokeMock({ get_wishlists: () => lists });

      await service.fetchWishlists();

      expect(service.activeWishlistId).toBe('wl-2');
    });

    it('does not override existing activeWishlistId on re-fetch', async () => {
      const lists = [makeWishlist({ id: 'wl-1', isDefault: true })];
      setupInvokeMock({ get_wishlists: () => lists });
      await service.fetchWishlists();
      expect(service.activeWishlistId).toBe('wl-1');

      // Re-fetch with different default — active should not change
      const lists2 = [
        makeWishlist({ id: 'wl-1', isDefault: false }),
        makeWishlist({ id: 'wl-2', isDefault: true })
      ];
      setupInvokeMock({ get_wishlists: () => lists2 });
      await service.fetchWishlists();

      expect(service.activeWishlistId).toBe('wl-1');
    });

    it('does not auto-select when no default exists', async () => {
      const lists = [makeWishlist({ id: 'wl-1', isDefault: false })];
      setupInvokeMock({ get_wishlists: () => lists });

      await service.fetchWishlists();

      expect(service.activeWishlistId).toBeNull();
    });

    it('shows error toast on failure', async () => {
      mockInvoke.mockRejectedValue(new Error('network error'));

      await service.fetchWishlists();

      expect(mockToaster.error).toHaveBeenCalled();
    });

    it('does not throw on failure', async () => {
      mockInvoke.mockRejectedValue(new Error('network error'));

      await expect(service.fetchWishlists()).resolves.toBeUndefined();
    });
  });

  // ── loadWishlistItems ──────────────────────────────────────────────────────

  describe('loadWishlistItems', () => {
    it('loads items for a wishlist', async () => {
      const items = [makeItem({ id: 'item-1' })];
      setupInvokeMock({
        get_wishlist_by_id: () => ({ items })
      });

      await service.loadWishlistItems('wl-1');

      expect(service.itemsByWishlist['wl-1']).toEqual(items);
    });

    it('stores empty array when result has no items', async () => {
      setupInvokeMock({
        get_wishlist_by_id: () => ({ items: [] })
      });

      await service.loadWishlistItems('wl-1');

      expect(service.itemsByWishlist['wl-1']).toEqual([]);
    });

    it('stores empty array when result is null', async () => {
      setupInvokeMock({
        get_wishlist_by_id: () => null
      });

      await service.loadWishlistItems('wl-1');

      expect(service.itemsByWishlist['wl-1']).toEqual([]);
    });

    it('shows error toast on failure', async () => {
      mockInvoke.mockRejectedValue(new Error('load failed'));

      await service.loadWishlistItems('wl-1');

      expect(mockToaster.error).toHaveBeenCalled();
    });

    it('does not throw on failure', async () => {
      mockInvoke.mockRejectedValue(new Error('load failed'));

      await expect(service.loadWishlistItems('wl-1')).resolves.toBeUndefined();
    });

    it('preserves existing items for other wishlists', async () => {
      const existingItems = [makeItem({ id: 'item-2' })];
      setupInvokeMock({ get_wishlist_by_id: () => ({ items: existingItems }) });
      await service.loadWishlistItems('wl-2');

      const newItems = [makeItem({ id: 'item-1' })];
      setupInvokeMock({ get_wishlist_by_id: () => ({ items: newItems }) });
      await service.loadWishlistItems('wl-1');

      expect(service.itemsByWishlist['wl-2']).toEqual(existingItems);
      expect(service.itemsByWishlist['wl-1']).toEqual(newItems);
    });
  });

  // ── selectWishlist ─────────────────────────────────────────────────────────

  describe('selectWishlist', () => {
    it('sets activeWishlistId', async () => {
      setupInvokeMock({ get_wishlist_by_id: () => ({ items: [] }) });

      await service.selectWishlist('wl-1');

      expect(service.activeWishlistId).toBe('wl-1');
    });

    it('loads items when not cached', async () => {
      const items = [makeItem()];
      setupInvokeMock({ get_wishlist_by_id: () => ({ items }) });

      await service.selectWishlist('wl-1');

      expect(mockInvoke).toHaveBeenCalledWith(
        'get_wishlist_by_id',
        expect.objectContaining({ id: 'wl-1' })
      );
      expect(service.itemsByWishlist['wl-1']).toEqual(items);
    });

    it('skips loading items when already cached', async () => {
      const items = [makeItem()];
      setupInvokeMock({ get_wishlist_by_id: () => ({ items }) });
      await service.loadWishlistItems('wl-1');

      vi.clearAllMocks();
      setupInvokeMock({ get_wishlist_by_id: () => ({ items: [] }) });

      await service.selectWishlist('wl-1');

      expect(mockInvoke).not.toHaveBeenCalled();
    });
  });

  // ── createWishlist ─────────────────────────────────────────────────────────

  describe('createWishlist', () => {
    it('optimistically adds a temp entry before resolving', async () => {
      const real = makeWishlist({ id: 'real-id', name: 'New List' });
      // Use a delayed mock to observe optimistic state
      mockInvoke.mockImplementation(async (cmd) => {
        if (String(cmd) === 'create_wishlist') {
          await new Promise((r) => setTimeout(r, 20));
          return real;
        }
        throw new Error(`Unmocked: ${String(cmd)}`);
      });

      const promise = service.createWishlist('New List');
      // Optimistic entry should be present before the promise resolves
      expect(service.wishlists.length).toBe(1);
      expect(service.wishlists[0].id).toMatch(/^temp-/);
      expect(service.wishlists[0].name).toBe('New List');

      await promise;
    });

    it('replaces temp entry with real data on success', async () => {
      const real = makeWishlist({ id: 'real-id', name: 'New List' });
      setupInvokeMock({ create_wishlist: () => real });

      await service.createWishlist('New List');

      expect(service.wishlists.length).toBe(1);
      expect(service.wishlists[0].id).toBe('real-id');
    });

    it('returns the created wishlist on success', async () => {
      const real = makeWishlist({ id: 'real-id', name: 'New List' });
      setupInvokeMock({ create_wishlist: () => real });

      const result = await service.createWishlist('New List');

      expect(result).toEqual(real);
    });

    it('shows loading toast during operation', async () => {
      const real = makeWishlist({ id: 'real-id' });
      setupInvokeMock({ create_wishlist: () => real });

      await service.createWishlist('New List');

      expect(mockToaster.loading).toHaveBeenCalledWith('Loading...', expect.anything());
    });

    it('shows success toast on success', async () => {
      const real = makeWishlist({ id: 'real-id' });
      setupInvokeMock({ create_wishlist: () => real });

      await service.createWishlist('New List');

      expect(mockToaster.success).toHaveBeenCalledWith('Success', expect.anything());
    });

    it('dismisses loading toast on success', async () => {
      const toastId = 'toast-123';
      mockToaster.loading.mockReturnValue(toastId);
      const real = makeWishlist({ id: 'real-id' });
      setupInvokeMock({ create_wishlist: () => real });

      await service.createWishlist('New List');

      expect(mockToaster.dismiss).toHaveBeenCalledWith(toastId);
    });

    it('rolls back optimistic update on failure', async () => {
      mockInvoke.mockRejectedValue(new Error('create failed'));

      await service.createWishlist('New List');

      expect(service.wishlists).toEqual([]);
    });

    it('returns null on failure', async () => {
      mockInvoke.mockRejectedValue(new Error('create failed'));

      const result = await service.createWishlist('New List');

      expect(result).toBeNull();
    });

    it('shows error toast on failure', async () => {
      mockInvoke.mockRejectedValue(new Error('create failed'));

      await service.createWishlist('New List');

      expect(mockToaster.error).toHaveBeenCalled();
    });

    it('dismisses loading toast on failure', async () => {
      const toastId = 'toast-456';
      mockToaster.loading.mockReturnValue(toastId);
      mockInvoke.mockRejectedValue(new Error('create failed'));

      await service.createWishlist('New List');

      expect(mockToaster.dismiss).toHaveBeenCalledWith(toastId);
    });

    describe('when isDefault=true', () => {
      it('clears isDefault on existing wishlists optimistically', async () => {
        // Pre-populate with an existing default
        setupInvokeMock({
          get_wishlists: () => [makeWishlist({ id: 'wl-existing', isDefault: true })]
        });
        await service.fetchWishlists();

        const real = makeWishlist({ id: 'real-id', isDefault: true });
        mockInvoke.mockImplementation(async (cmd) => {
          if (String(cmd) === 'create_wishlist') {
            await new Promise((r) => setTimeout(r, 20));
            return real;
          }
          throw new Error(`Unmocked: ${String(cmd)}`);
        });

        const promise = service.createWishlist('Default List', true);

        // The old default should be cleared optimistically
        const existingOptimistic = service.wishlists.find((w) => w.id === 'wl-existing');
        expect(existingOptimistic?.isDefault).toBe(false);

        await promise;
      });

      it('sets activeWishlistId to real id on success', async () => {
        const real = makeWishlist({ id: 'real-id', isDefault: true });
        setupInvokeMock({ create_wishlist: () => real });

        await service.createWishlist('Default List', true);

        expect(service.activeWishlistId).toBe('real-id');
      });

      it('sets activeWishlistId to tempId during optimistic phase', async () => {
        const real = makeWishlist({ id: 'real-id', isDefault: true });
        mockInvoke.mockImplementation(async (cmd) => {
          if (String(cmd) === 'create_wishlist') {
            await new Promise((r) => setTimeout(r, 20));
            return real;
          }
          throw new Error(`Unmocked: ${String(cmd)}`);
        });

        const promise = service.createWishlist('Default List', true);

        expect(service.activeWishlistId).toMatch(/^temp-/);

        await promise;
      });
    });
  });

  // ── renameWishlist ─────────────────────────────────────────────────────────

  describe('renameWishlist', () => {
    beforeEach(async () => {
      setupInvokeMock({
        get_wishlists: () => [makeWishlist({ id: 'wl-1', name: 'Original Name' })]
      });
      await service.fetchWishlists();
    });

    it('optimistically updates the name', async () => {
      mockInvoke.mockImplementation(async (cmd) => {
        if (String(cmd) === 'rename_wishlist') {
          await new Promise((r) => setTimeout(r, 20));
          return null;
        }
        throw new Error(`Unmocked: ${String(cmd)}`);
      });

      const promise = service.renameWishlist('wl-1', 'New Name');

      expect(service.wishlists.find((w) => w.id === 'wl-1')?.name).toBe('New Name');

      await promise;
    });

    it('shows success toast on success', async () => {
      setupInvokeMock({ rename_wishlist: () => null });

      await service.renameWishlist('wl-1', 'New Name');

      expect(mockToaster.success).toHaveBeenCalledWith('Success', expect.anything());
    });

    it('dismisses loading toast on success', async () => {
      const toastId = 'toast-789';
      mockToaster.loading.mockReturnValue(toastId);
      setupInvokeMock({ rename_wishlist: () => null });

      await service.renameWishlist('wl-1', 'New Name');

      expect(mockToaster.dismiss).toHaveBeenCalledWith(toastId);
    });

    it('rolls back name on failure', async () => {
      mockInvoke.mockImplementation(async (cmd) => {
        if (String(cmd) === 'rename_wishlist') throw new Error('rename failed');
        throw new Error(`Unmocked: ${String(cmd)}`);
      });

      await service.renameWishlist('wl-1', 'New Name');

      expect(service.wishlists.find((w) => w.id === 'wl-1')?.name).toBe('Original Name');
    });

    it('shows error toast on failure', async () => {
      mockInvoke.mockImplementation(async (cmd) => {
        if (String(cmd) === 'rename_wishlist') throw new Error('rename failed');
        throw new Error(`Unmocked: ${String(cmd)}`);
      });

      await service.renameWishlist('wl-1', 'New Name');

      expect(mockToaster.error).toHaveBeenCalled();
    });

    it('dismisses loading toast on failure', async () => {
      const toastId = 'toast-abc';
      mockToaster.loading.mockReturnValue(toastId);
      mockInvoke.mockImplementation(async (cmd) => {
        if (String(cmd) === 'rename_wishlist') throw new Error('rename failed');
        throw new Error(`Unmocked: ${String(cmd)}`);
      });

      await service.renameWishlist('wl-1', 'New Name');

      expect(mockToaster.dismiss).toHaveBeenCalledWith(toastId);
    });

    it('does not throw on failure', async () => {
      mockInvoke.mockImplementation(async (cmd) => {
        if (String(cmd) === 'rename_wishlist') throw new Error('rename failed');
        throw new Error(`Unmocked: ${String(cmd)}`);
      });

      await expect(service.renameWishlist('wl-1', 'New Name')).resolves.toBeUndefined();
    });
  });

  // ── deleteWishlist ─────────────────────────────────────────────────────────

  describe('deleteWishlist', () => {
    beforeEach(async () => {
      setupInvokeMock({
        get_wishlists: () => [
          makeWishlist({ id: 'wl-1', name: 'List One' }),
          makeWishlist({ id: 'wl-2', name: 'List Two' })
        ]
      });
      await service.fetchWishlists();
    });

    it('optimistically removes the wishlist', async () => {
      mockInvoke.mockImplementation(async (cmd) => {
        if (String(cmd) === 'delete_wishlist') {
          await new Promise((r) => setTimeout(r, 20));
          return null;
        }
        throw new Error(`Unmocked: ${String(cmd)}`);
      });

      const promise = service.deleteWishlist('wl-1');

      expect(service.wishlists.find((w) => w.id === 'wl-1')).toBeUndefined();
      expect(service.wishlists.length).toBe(1);

      await promise;
    });

    it('optimistically removes items for the deleted wishlist', async () => {
      // Load items for wl-1
      setupInvokeMock({ get_wishlist_by_id: () => ({ items: [makeItem()] }) });
      await service.loadWishlistItems('wl-1');

      mockInvoke.mockImplementation(async (cmd) => {
        if (String(cmd) === 'delete_wishlist') {
          await new Promise((r) => setTimeout(r, 20));
          return null;
        }
        throw new Error(`Unmocked: ${String(cmd)}`);
      });

      const promise = service.deleteWishlist('wl-1');

      expect(service.itemsByWishlist['wl-1']).toBeUndefined();

      await promise;
    });

    it('clears activeWishlistId when deleting the active wishlist', async () => {
      setupInvokeMock({ get_wishlist_by_id: () => ({ items: [] }) });
      await service.selectWishlist('wl-1');

      setupInvokeMock({ delete_wishlist: () => null });
      await service.deleteWishlist('wl-1');

      expect(service.activeWishlistId).toBeNull();
    });

    it('does not clear activeWishlistId when deleting a non-active wishlist', async () => {
      setupInvokeMock({ get_wishlist_by_id: () => ({ items: [] }) });
      await service.selectWishlist('wl-1');

      setupInvokeMock({ delete_wishlist: () => null });
      await service.deleteWishlist('wl-2');

      expect(service.activeWishlistId).toBe('wl-1');
    });

    it('shows success toast on success', async () => {
      setupInvokeMock({ delete_wishlist: () => null });

      await service.deleteWishlist('wl-1');

      expect(mockToaster.success).toHaveBeenCalledWith('Success', expect.anything());
    });

    it('rolls back on failure', async () => {
      mockInvoke.mockImplementation(async (cmd) => {
        if (String(cmd) === 'delete_wishlist') throw new Error('delete failed');
        throw new Error(`Unmocked: ${String(cmd)}`);
      });

      await service.deleteWishlist('wl-1');

      expect(service.wishlists.find((w) => w.id === 'wl-1')).toBeDefined();
      expect(service.wishlists.length).toBe(2);
    });

    it('shows error toast on failure', async () => {
      mockInvoke.mockImplementation(async (cmd) => {
        if (String(cmd) === 'delete_wishlist') throw new Error('delete failed');
        throw new Error(`Unmocked: ${String(cmd)}`);
      });

      await service.deleteWishlist('wl-1');

      expect(mockToaster.error).toHaveBeenCalled();
    });

    it('dismisses loading toast on failure', async () => {
      const toastId = 'toast-del';
      mockToaster.loading.mockReturnValue(toastId);
      mockInvoke.mockImplementation(async (cmd) => {
        if (String(cmd) === 'delete_wishlist') throw new Error('delete failed');
        throw new Error(`Unmocked: ${String(cmd)}`);
      });

      await service.deleteWishlist('wl-1');

      expect(mockToaster.dismiss).toHaveBeenCalledWith(toastId);
    });

    it('does not throw on failure', async () => {
      mockInvoke.mockImplementation(async (cmd) => {
        if (String(cmd) === 'delete_wishlist') throw new Error('delete failed');
        throw new Error(`Unmocked: ${String(cmd)}`);
      });

      await expect(service.deleteWishlist('wl-1')).resolves.toBeUndefined();
    });
  });

  // ── refreshWishlist ────────────────────────────────────────────────────────

  describe('refreshWishlist', () => {
    it('delegates to loadWishlistItems', async () => {
      const items = [makeItem()];
      setupInvokeMock({ get_wishlist_by_id: () => ({ items }) });

      await service.refreshWishlist('wl-1');

      expect(service.itemsByWishlist['wl-1']).toEqual(items);
    });

    it('calls get_wishlist_by_id with the given id', async () => {
      setupInvokeMock({ get_wishlist_by_id: () => ({ items: [] }) });

      await service.refreshWishlist('wl-42');

      expect(mockInvoke).toHaveBeenCalledWith(
        'get_wishlist_by_id',
        expect.objectContaining({ id: 'wl-42' })
      );
    });
  });

  // ── derived getters ────────────────────────────────────────────────────────

  describe('derived getters', () => {
    beforeEach(async () => {
      setupInvokeMock({
        get_wishlists: () => [
          makeWishlist({ id: 'wl-1', isDefault: false }),
          makeWishlist({ id: 'wl-2', isDefault: true })
        ]
      });
      await service.fetchWishlists();
    });

    describe('defaultWishlist', () => {
      it('returns the wishlist marked as default', () => {
        expect(service.defaultWishlist?.id).toBe('wl-2');
      });

      it('returns null when no default exists', async () => {
        setupInvokeMock({
          get_wishlists: () => [
            makeWishlist({ id: 'wl-1', isDefault: false }),
            makeWishlist({ id: 'wl-2', isDefault: false })
          ]
        });
        await service.fetchWishlists();

        expect(service.defaultWishlist).toBeNull();
      });
    });

    describe('activeWishlist', () => {
      it('returns null when activeWishlistId is null', () => {
        // fetchWishlists auto-selected wl-2 (isDefault). Force a service with no auto-select.
        const freshService = new WishlistService();
        expect(freshService.activeWishlist).toBeNull();
      });

      it('returns the wishlist matching activeWishlistId', () => {
        // wl-2 was auto-selected (isDefault=true)
        expect(service.activeWishlist?.id).toBe('wl-2');
      });

      it('returns null when activeWishlistId does not match any wishlist', async () => {
        setupInvokeMock({ get_wishlist_by_id: () => ({ items: [] }) });
        await service.selectWishlist('non-existent-id');

        expect(service.activeWishlist).toBeNull();
      });
    });

    describe('wishlistItems', () => {
      it('returns empty array when activeWishlistId is null', () => {
        const freshService = new WishlistService();
        expect(freshService.wishlistItems).toEqual([]);
      });

      it('returns empty array when no items cached for active wishlist', () => {
        // wl-2 is active but has no cached items
        expect(service.wishlistItems).toEqual([]);
      });

      it('returns items for the active wishlist', async () => {
        const items = [makeItem()];
        setupInvokeMock({ get_wishlist_by_id: () => ({ items }) });
        await service.loadWishlistItems('wl-2');

        expect(service.wishlistItems).toEqual(items);
      });

      it('returns items for the correct wishlist when multiple are cached', async () => {
        const items1 = [makeItem({ id: 'item-1' })];
        const items2 = [makeItem({ id: 'item-2' })];

        setupInvokeMock({ get_wishlist_by_id: () => ({ items: items1 }) });
        await service.loadWishlistItems('wl-1');

        setupInvokeMock({ get_wishlist_by_id: () => ({ items: items2 }) });
        await service.loadWishlistItems('wl-2');

        // wl-2 is active (auto-selected as default)
        expect(service.wishlistItems).toEqual(items2);
      });
    });
  });
});
