import { describe, it, expect, vi, beforeEach } from 'vitest';
import { flushSync } from 'svelte';

// ─── Mocks (must be before any imports) ─────────────────────────────────────

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}));

vi.mock('$lib/toaster', () => ({
  toaster: {
    loading: vi.fn(),
    success: vi.fn(),
    error: vi.fn(),
    dismiss: vi.fn()
  }
}));

vi.mock('$lib/paraglide/messages.js', () => ({}));

vi.mock('$lib/config/tags', () => ({
  FIXED_TAG_META: {},
  sortAvailableTags: (tags: string[]) => tags,
  tagIcon: () => null
}));

// ─── Imports ─────────────────────────────────────────────────────────────────

import { invoke } from '@tauri-apps/api/core';
import { toaster } from '$lib/toaster';
import { CollectionService } from '$lib/features/collection/services/CollectionService.svelte';

// ─── Helpers ─────────────────────────────────────────────────────────────────

const mockInvoke = vi.mocked(invoke);
const mockToaster = vi.mocked(toaster);

/**
 * Route invoke calls by command name so each test can register handlers
 * independently without worrying about call order.
 */
function setupInvokeMock(handlers: Record<string, () => unknown>): void {
  mockInvoke.mockImplementation(async (command: string) => {
    const handler = handlers[command];
    if (!handler) {
      throw new Error(`Unmocked Tauri command: ${command}`);
    }
    return handler();
  });
}

type CollectionItemOverrides = {
  id?: string;
  scale?: string;
  manufacturer?: string | { name: string };
  productCode?: string;
  description?: string;
  category?: string;
};

function makeItem(overrides: CollectionItemOverrides = {}) {
  return {
    id: overrides.id ?? 'item-1',
    railwayModel: {
      railwayModelId: `trn:railway-model:test:${overrides.id ?? 'item-1'}`,
      manufacturer: overrides.manufacturer ?? 'Roco',
      productCode: overrides.productCode ?? 'CODE-1',
      description: overrides.description ?? 'Model Description',
      scale: overrides.scale ?? 'H0',
      epoch: 'VI',
      category: overrides.category ?? 'LOCOMOTIVES',
      powerMethod: 'DC'
    },
    addedDate: '2026-01-01',
    removedDate: null,
    purchaseCondition: null,
    modelCondition: null,
    boxCondition: null,
    notes: null,
    rollingStocks: [],
    purchaseInfo: null
  };
}

function makeCollection(items = makeItem() ? [makeItem()] : []) {
  return {
    id: 'trn:collection:test',
    name: 'My Collection',
    summary: { totalItems: items.length },
    totalValue: null,
    items
  };
}

// ─── Tests ───────────────────────────────────────────────────────────────────

describe('CollectionService', () => {
  let service: CollectionService;

  beforeEach(() => {
    vi.resetAllMocks();
    service = new CollectionService();
  });

  // ── Initial state ──────────────────────────────────────────────────────────

  describe('initial state', () => {
    it('collection is null', () => {
      expect(service.collection).toBeNull();
    });

    it('rawItems is empty array', () => {
      expect(service.rawItems).toHaveLength(0);
    });

    it('isLoading is false', () => {
      expect(service.isLoading).toBe(false);
    });

    it('summary is undefined', () => {
      expect(service.summary).toBeUndefined();
    });

    it('totalCount is 0', () => {
      expect(service.totalCount).toBe(0);
    });

    it('filteredCount is 0', () => {
      expect(service.filteredCount).toBe(0);
    });

    it('filters start with empty query and no scale', () => {
      expect(service.filters.query).toBe('');
      expect(service.filters.scale).toBeNull();
      expect(service.filters.tags.size).toBe(0);
    });
  });

  // ── fetchCollection ────────────────────────────────────────────────────────

  describe('fetchCollection', () => {
    it('populates collection on success', async () => {
      const collection = makeCollection([makeItem({ id: '1' }), makeItem({ id: '2' })]);
      setupInvokeMock({ get_collection: () => collection });

      await service.fetchCollection();

      expect(service.collection).toEqual(collection);
      expect(service.rawItems).toHaveLength(2);
    });

    it('sets isLoading to true during fetch and false afterwards', async () => {
      const collection = makeCollection([]);

      let resolveFetch!: (value: unknown) => void;
      const fetchPromise = new Promise((res) => {
        resolveFetch = res;
      });

      mockInvoke.mockImplementation(async (command: string) => {
        if (command === 'get_collection') {
          return fetchPromise;
        }
        throw new Error(`Unmocked command: ${command}`);
      });

      const invokePromise = service.fetchCollection();
      // isLoading should be true while the fetch is pending
      expect(service.isLoading).toBe(true);

      resolveFetch(collection);
      await invokePromise;

      expect(service.isLoading).toBe(false);
    });

    it('resets isLoading to false after success', async () => {
      setupInvokeMock({ get_collection: () => makeCollection([]) });

      await service.fetchCollection();

      expect(service.isLoading).toBe(false);
    });

    it('resets isLoading to false after backend failure', async () => {
      mockInvoke.mockRejectedValueOnce({ DatabaseError: 'Connection failed' });

      await service.fetchCollection();

      expect(service.isLoading).toBe(false);
    });

    it('does not throw on backend failure', async () => {
      mockInvoke.mockRejectedValueOnce({ DatabaseError: 'Connection failed' });

      await expect(service.fetchCollection()).resolves.toBeUndefined();
    });

    it('calls toaster.error on backend failure', async () => {
      mockInvoke.mockRejectedValueOnce({ DatabaseError: 'Connection failed' });

      await service.fetchCollection();

      expect(mockToaster.error).toHaveBeenCalledOnce();
    });

    it('collection remains null after backend failure', async () => {
      mockInvoke.mockRejectedValueOnce({ DatabaseError: 'Connection failed' });

      await service.fetchCollection();

      expect(service.collection).toBeNull();
    });

    it('accepts an optional query parameter without error', async () => {
      setupInvokeMock({ get_collection: () => makeCollection([]) });

      await expect(service.fetchCollection('some-query')).resolves.toBeUndefined();
    });
  });

  // ── setQuery ───────────────────────────────────────────────────────────────

  describe('setQuery', () => {
    it('updates filters.query', () => {
      flushSync(() => service.setQuery('Roco'));
      expect(service.filters.query).toBe('Roco');
    });

    it('updates filters.query to empty string', () => {
      flushSync(() => {
        service.setQuery('Roco');
        service.setQuery('');
      });
      expect(service.filters.query).toBe('');
    });

    it('overwrites previous query', () => {
      flushSync(() => {
        service.setQuery('Roco');
        service.setQuery('Märklin');
      });
      expect(service.filters.query).toBe('Märklin');
    });
  });

  // ── toggleTag ──────────────────────────────────────────────────────────────

  describe('toggleTag', () => {
    it('adds tag when not present', () => {
      flushSync(() => service.toggleTag('FEATURED'));
      expect(service.filters.tags.has('FEATURED')).toBe(true);
    });

    it('removes tag when already present', () => {
      flushSync(() => {
        service.toggleTag('FEATURED');
        service.toggleTag('FEATURED');
      });
      expect(service.filters.tags.has('FEATURED')).toBe(false);
    });

    it('can add multiple tags independently', () => {
      flushSync(() => {
        service.toggleTag('FEATURED');
        service.toggleTag('NEW');
      });
      expect(service.filters.tags.has('FEATURED')).toBe(true);
      expect(service.filters.tags.has('NEW')).toBe(true);
      expect(service.filters.tags.size).toBe(2);
    });

    it('removing one tag does not affect another', () => {
      flushSync(() => {
        service.toggleTag('FEATURED');
        service.toggleTag('NEW');
        service.toggleTag('FEATURED');
      });
      expect(service.filters.tags.has('FEATURED')).toBe(false);
      expect(service.filters.tags.has('NEW')).toBe(true);
    });
  });

  // ── setScale ───────────────────────────────────────────────────────────────

  describe('setScale', () => {
    it('sets scale filter', () => {
      flushSync(() => service.setScale('N'));
      expect(service.filters.scale).toBe('N');
    });

    it('clears scale filter when given null', () => {
      flushSync(() => {
        service.setScale('N');
        service.setScale(null);
      });
      expect(service.filters.scale).toBeNull();
    });

    it('overwrites previous scale', () => {
      flushSync(() => {
        service.setScale('H0');
        service.setScale('N');
      });
      expect(service.filters.scale).toBe('N');
    });
  });

  // ── clearFilters ───────────────────────────────────────────────────────────

  describe('clearFilters', () => {
    it('resets query to empty string', () => {
      flushSync(() => {
        service.setQuery('Roco');
        service.clearFilters();
      });
      expect(service.filters.query).toBe('');
    });

    it('resets scale to null', () => {
      flushSync(() => {
        service.setScale('H0');
        service.clearFilters();
      });
      expect(service.filters.scale).toBeNull();
    });

    it('clears all tags', () => {
      flushSync(() => {
        service.toggleTag('FEATURED');
        service.toggleTag('NEW');
        service.clearFilters();
      });
      expect(service.filters.tags.size).toBe(0);
    });

    it('restores filteredItems to all items after clearing', async () => {
      setupInvokeMock({
        get_collection: () => makeCollection([makeItem({ id: '1' }), makeItem({ id: '2' })])
      });
      await service.fetchCollection();

      flushSync(() => {
        service.setQuery('NoMatchHere');
        service.clearFilters();
      });

      expect(service.filteredItems).toHaveLength(2);
    });
  });

  // ── filteredItems ──────────────────────────────────────────────────────────

  describe('filteredItems', () => {
    beforeEach(async () => {
      const items = [
        makeItem({
          id: '1',
          scale: 'H0',
          manufacturer: 'Roco',
          productCode: 'ROCO-1',
          description: 'Steam Locomotive'
        }),
        makeItem({
          id: '2',
          scale: 'N',
          manufacturer: 'Märklin',
          productCode: 'MRK-2',
          description: 'Diesel Engine'
        }),
        makeItem({
          id: '3',
          scale: 'H0',
          manufacturer: 'Fleischmann',
          productCode: 'FL-3',
          description: 'Tank Wagon'
        })
      ];
      setupInvokeMock({ get_collection: () => makeCollection(items) });
      await service.fetchCollection();
    });

    it('returns all items when no filters are active', () => {
      expect(service.filteredItems).toHaveLength(3);
    });

    it('filters by scale', () => {
      flushSync(() => service.setScale('N'));
      expect(service.filteredItems).toHaveLength(1);
      expect(service.filteredItems[0].id).toBe('2');
    });

    it('filters by scale and returns all matching items', () => {
      flushSync(() => service.setScale('H0'));
      expect(service.filteredItems).toHaveLength(2);
    });

    it('returns empty array when scale has no matches', () => {
      flushSync(() => service.setScale('Z'));
      expect(service.filteredItems).toHaveLength(0);
    });

    it('filters by text query matching manufacturer', () => {
      flushSync(() => service.setQuery('roco'));
      expect(service.filteredItems).toHaveLength(1);
      expect(service.filteredItems[0].id).toBe('1');
    });

    it('filters by text query matching product code', () => {
      flushSync(() => service.setQuery('MRK-2'));
      expect(service.filteredItems).toHaveLength(1);
      expect(service.filteredItems[0].id).toBe('2');
    });

    it('filters by text query matching description', () => {
      flushSync(() => service.setQuery('tank wagon'));
      expect(service.filteredItems).toHaveLength(1);
      expect(service.filteredItems[0].id).toBe('3');
    });

    it('text query is case-insensitive', () => {
      flushSync(() => service.setQuery('MÄRKLIN'));
      expect(service.filteredItems).toHaveLength(1);
      expect(service.filteredItems[0].id).toBe('2');
    });

    it('combines scale and query filters', () => {
      flushSync(() => {
        service.setScale('H0');
        service.setQuery('roco');
      });
      expect(service.filteredItems).toHaveLength(1);
      expect(service.filteredItems[0].id).toBe('1');
    });

    it('returns empty array when combined filters have no matches', () => {
      flushSync(() => {
        service.setScale('N');
        service.setQuery('roco');
      });
      expect(service.filteredItems).toHaveLength(0);
    });
  });

  describe('filteredItems with manufacturer as object', () => {
    it('extracts name from manufacturer object for text search', async () => {
      const items = [
        makeItem({
          id: '1',
          manufacturer: { name: 'Roco' },
          productCode: 'X1',
          description: 'Loco'
        }),
        makeItem({
          id: '2',
          manufacturer: { name: 'Märklin' },
          productCode: 'X2',
          description: 'Wagon'
        })
      ];
      setupInvokeMock({ get_collection: () => makeCollection(items) });
      await service.fetchCollection();

      flushSync(() => service.setQuery('märklin'));

      expect(service.filteredItems).toHaveLength(1);
      expect(service.filteredItems[0].id).toBe('2');
    });

    it('uses empty string when manufacturer object has no name property', async () => {
      const items = [
        // manufacturer object with no `name` key — should not crash
        makeItem({
          id: '1',
          manufacturer: {} as { name: string },
          productCode: 'X1',
          description: 'Gadget'
        })
      ];
      setupInvokeMock({ get_collection: () => makeCollection(items) });
      await service.fetchCollection();

      // Query on manufacturer should find nothing, but should not throw
      flushSync(() => service.setQuery('Roco'));
      expect(service.filteredItems).toHaveLength(0);
    });
  });

  // ── totalCount / filteredCount ─────────────────────────────────────────────

  describe('totalCount and filteredCount', () => {
    it('totalCount equals number of items in collection', async () => {
      const items = [makeItem({ id: '1' }), makeItem({ id: '2' }), makeItem({ id: '3' })];
      setupInvokeMock({ get_collection: () => makeCollection(items) });
      await service.fetchCollection();

      expect(service.totalCount).toBe(3);
    });

    it('totalCount is 0 when collection is null', () => {
      expect(service.totalCount).toBe(0);
    });

    it('filteredCount equals filteredItems length', async () => {
      const items = [
        makeItem({ id: '1', scale: 'H0' }),
        makeItem({ id: '2', scale: 'N' }),
        makeItem({ id: '3', scale: 'H0' })
      ];
      setupInvokeMock({ get_collection: () => makeCollection(items) });
      await service.fetchCollection();

      flushSync(() => service.setScale('H0'));

      expect(service.filteredCount).toBe(2);
      expect(service.filteredCount).toBe(service.filteredItems.length);
    });

    it('filteredCount matches totalCount when no filter is active', async () => {
      const items = [makeItem({ id: '1' }), makeItem({ id: '2' })];
      setupInvokeMock({ get_collection: () => makeCollection(items) });
      await service.fetchCollection();

      expect(service.filteredCount).toBe(service.totalCount);
    });
  });

  // ── deleteItem ─────────────────────────────────────────────────────────────

  describe('deleteItem', () => {
    const existingItem = makeItem({ id: 'item-to-delete', category: 'LOCOMOTIVES' });

    beforeEach(async () => {
      setupInvokeMock({
        get_collection: () => makeCollection([existingItem]),
        remove_collection_item: () => null
      });
      await service.fetchCollection();
      // Re-setup so remove_collection_item and subsequent get_collection work
      setupInvokeMock({
        get_collection: () => makeCollection([]),
        remove_collection_item: () => null
      });
    });

    it('returns true on success', async () => {
      const result = await service.deleteItem('item-to-delete');
      expect(result).toBe(true);
    });

    it('calls remove_collection_item with correct args', async () => {
      await service.deleteItem('item-to-delete');

      expect(mockInvoke).toHaveBeenCalledWith(
        'remove_collection_item',
        expect.objectContaining({
          args: expect.objectContaining({
            collectionItemId: 'item-to-delete',
            category: 'LOCOMOTIVES'
          })
        })
      );
    });

    it('calls fetchCollection after successful delete', async () => {
      await service.deleteItem('item-to-delete');

      // get_collection is called once on setup and once after delete
      const getCalls = mockInvoke.mock.calls.filter(([cmd]) => cmd === 'get_collection');
      expect(getCalls.length).toBeGreaterThanOrEqual(1);
    });

    it('toasts success message after delete', async () => {
      await service.deleteItem('item-to-delete');
      expect(mockToaster.success).toHaveBeenCalledWith('Item removed from collection');
    });

    it('returns false when item is not found in collection', async () => {
      const result = await service.deleteItem('non-existent-id');
      expect(result).toBe(false);
    });

    it('calls toaster.error when item is not found', async () => {
      await service.deleteItem('non-existent-id');
      expect(mockToaster.error).toHaveBeenCalledWith('Item not found');
    });

    it('returns false on backend failure', async () => {
      setupInvokeMock({
        get_collection: () => makeCollection([existingItem]),
        remove_collection_item: () => {
          throw { NotFound: 'Item does not exist' };
        }
      });
      // Re-fetch so the item is present in local state
      await service.fetchCollection();

      setupInvokeMock({
        remove_collection_item: () => {
          throw { NotFound: 'Item does not exist' };
        }
      });

      const result = await service.deleteItem('item-to-delete');
      expect(result).toBe(false);
    });

    it('toasts error with message on backend failure', async () => {
      setupInvokeMock({
        get_collection: () => makeCollection([existingItem]),
        remove_collection_item: () => {
          throw { NotFound: 'Item does not exist' };
        }
      });
      await service.fetchCollection();

      setupInvokeMock({
        remove_collection_item: () => {
          throw { NotFound: 'Item does not exist' };
        }
      });

      await service.deleteItem('item-to-delete');
      expect(mockToaster.error).toHaveBeenCalledWith(
        expect.stringContaining('Failed to remove item:')
      );
    });

    it('returns false on unexpected exception (e.g. thrown Error)', async () => {
      setupInvokeMock({
        get_collection: () => makeCollection([existingItem])
      });
      await service.fetchCollection();

      // Remove handler so invoke throws a real JS exception
      mockInvoke.mockImplementationOnce(async () => {
        throw new Error('Unexpected crash');
      });

      const result = await service.deleteItem('item-to-delete');
      expect(result).toBe(false);
    });

    it('toasts generic error message on unexpected exception', async () => {
      setupInvokeMock({
        get_collection: () => makeCollection([existingItem])
      });
      await service.fetchCollection();

      mockInvoke.mockImplementationOnce(async () => {
        throw new Error('Unexpected crash');
      });

      await service.deleteItem('item-to-delete');
      expect(mockToaster.error).toHaveBeenLastCalledWith('Failed to remove item: Unexpected crash');
    });
  });
});
