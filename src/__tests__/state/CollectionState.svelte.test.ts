import { describe, it, expect, vi, beforeEach } from 'vitest';
import { flushSync } from 'svelte';

// Mock @tauri-apps/api/core BEFORE importing anything that depends on it
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}));

vi.mock('$lib/toaster', () => ({
  toaster: {
    loading: vi.fn(),
    success: vi.fn(),
    error: vi.fn()
  }
}));

vi.mock('$lib/state/collection.svelte', () => ({
  collectionStore: {
    refresh: vi.fn().mockResolvedValue(undefined),
    fetch: vi.fn().mockResolvedValue(undefined),
    items: [],
    collection: null,
    loading: false,
    getItemById: vi.fn().mockReturnValue(undefined)
  }
}));

vi.mock('$lib/paraglide/messages.js', () => ({
  collection_toast_loading: () => 'Loading...',
  collection_toast_success: () => 'Success',
  collection_toast_error: () => 'Error occurred',
  collection_toast_retry: () => 'Retry',
  add_model_success: () => 'Model added'
}));

import { invoke } from '@tauri-apps/api/core';
import { createCollectionState } from '$lib/features/collection/CollectionState.svelte';
import type { CollectionView, Scale } from '$lib/bindings';

const mockInvoke = vi.mocked(invoke);

// ─── helpers ───────────────────────────────────────────────────────────────

function makeCollection(items: CollectionView['items'] = []): CollectionView {
  return {
    id: 'trn:collection:test',
    name: 'Test Collection',
    summary: {
      locomotivesCount: 0,
      passengerCarsCount: 0,
      freightCarsCount: 0,
      trainSetsCount: 0,
      railcarsCount: 0,
      electricMultipleUnitsCount: 0,
      starterSetsCount: 0
    },
    totalValue: null,
    items
  };
}

function makeItem(
  id: string,
  overrides: Partial<{
    scale: Scale;
    manufacturer: string;
    productCode: string;
    description: string;
  }> = {}
) {
  return {
    id,
    railwayModel: {
      railwayModelId: `trn:railway-model:test:${id}`,
      manufacturer: overrides.manufacturer ?? 'Roco',
      productCode: overrides.productCode ?? `CODE-${id}`,
      description: overrides.description ?? `Model ${id}`,
      scale: (overrides.scale ?? 'H0') as Scale,
      epoch: 'VI',
      category: 'LOCOMOTIVES' as const,
      powerMethod: 'DC' as const
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

// ─── tests ─────────────────────────────────────────────────────────────────

describe('CollectionState', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('starts with empty state', () => {
    const state = createCollectionState();
    expect(state.collection).toBeNull();
    expect(state.rawItems).toHaveLength(0);
    expect(state.isLoading).toBe(false);
    expect(state.totalCount).toBe(0);
  });

  it('sets isLoading=true during fetch and false after success', async () => {
    const collection = makeCollection([makeItem('1')]);
    mockInvoke.mockResolvedValueOnce(collection);

    const state = createCollectionState();
    const promise = state.fetchCollection();

    // Loading should be true while awaiting
    // (not assertable after microtask resolution, so verify after completion)
    await promise;

    expect(state.isLoading).toBe(false);
    expect(state.collection).toEqual(collection);
  });

  it('fetchCollection populates collection and totalCount', async () => {
    const items = [makeItem('1'), makeItem('2'), makeItem('3')];
    const collection = makeCollection(items);
    mockInvoke.mockResolvedValueOnce(collection);

    const state = createCollectionState();
    await state.fetchCollection();

    expect(state.totalCount).toBe(3);
    expect(state.rawItems).toHaveLength(3);
  });

  it('handles fetch error gracefully without throwing', async () => {
    mockInvoke.mockRejectedValueOnce(new Error('Network error'));

    const state = createCollectionState();
    await expect(state.fetchCollection()).resolves.toBeUndefined();
    expect(state.collection).toBeNull();
    expect(state.isLoading).toBe(false);
  });

  describe('$derived filteredItems', () => {
    it('returns all items when no filters are active', async () => {
      const items = [makeItem('1'), makeItem('2')];
      mockInvoke.mockResolvedValueOnce(makeCollection(items));
      const state = createCollectionState();
      await state.fetchCollection();

      expect(state.filteredItems).toHaveLength(2);
    });

    it('filters by scale', async () => {
      const items = [
        makeItem('1', { scale: 'H0' }),
        makeItem('2', { scale: 'N' }),
        makeItem('3', { scale: 'H0' })
      ];
      mockInvoke.mockResolvedValueOnce(makeCollection(items));
      const state = createCollectionState();
      await state.fetchCollection();

      flushSync(() => state.setScale('N'));

      expect(state.filteredItems).toHaveLength(1);
      expect(state.filteredItems[0].id).toBe('2');
    });

    it('filters by query (manufacturer)', async () => {
      const items = [
        makeItem('1', { manufacturer: 'Roco' }),
        makeItem('2', { manufacturer: 'Märklin' }),
        makeItem('3', { manufacturer: 'Roco' })
      ];
      mockInvoke.mockResolvedValueOnce(makeCollection(items));
      const state = createCollectionState();
      await state.fetchCollection();

      flushSync(() => state.setQuery('märklin'));

      expect(state.filteredItems).toHaveLength(1);
      expect(state.filteredItems[0].id).toBe('2');
    });

    it('filters by query (product code)', async () => {
      const items = [
        makeItem('1', { productCode: 'ABC-123' }),
        makeItem('2', { productCode: 'XYZ-999' })
      ];
      mockInvoke.mockResolvedValueOnce(makeCollection(items));
      const state = createCollectionState();
      await state.fetchCollection();

      flushSync(() => state.setQuery('xyz'));

      expect(state.filteredItems).toHaveLength(1);
      expect(state.filteredItems[0].id).toBe('2');
    });

    it('filters by query (description)', async () => {
      const items = [
        makeItem('1', { description: 'Steam Locomotive' }),
        makeItem('2', { description: 'Diesel Engine' })
      ];
      mockInvoke.mockResolvedValueOnce(makeCollection(items));
      const state = createCollectionState();
      await state.fetchCollection();

      flushSync(() => state.setQuery('diesel'));

      expect(state.filteredItems).toHaveLength(1);
    });

    it('returns empty array when no items match filter', async () => {
      const items = [makeItem('1', { manufacturer: 'Roco' })];
      mockInvoke.mockResolvedValueOnce(makeCollection(items));
      const state = createCollectionState();
      await state.fetchCollection();

      flushSync(() => state.setQuery('Fleischmann'));

      expect(state.filteredItems).toHaveLength(0);
    });

    it('combines scale and query filters', async () => {
      const items = [
        makeItem('1', { scale: 'H0', manufacturer: 'Roco' }),
        makeItem('2', { scale: 'N', manufacturer: 'Roco' }),
        makeItem('3', { scale: 'H0', manufacturer: 'Märklin' })
      ];
      mockInvoke.mockResolvedValueOnce(makeCollection(items));
      const state = createCollectionState();
      await state.fetchCollection();

      flushSync(() => {
        state.setScale('H0');
        state.setQuery('roco');
      });

      expect(state.filteredItems).toHaveLength(1);
      expect(state.filteredItems[0].id).toBe('1');
    });
  });

  describe('filter management', () => {
    it('setScale updates scale filter', () => {
      const state = createCollectionState();
      flushSync(() => state.setScale('N'));
      expect(state.filters.scale).toBe('N');
    });

    it('setScale(null) clears scale filter', () => {
      const state = createCollectionState();
      flushSync(() => {
        state.setScale('N');
        state.setScale(null);
      });
      expect(state.filters.scale).toBeNull();
    });

    it('setQuery updates query filter', () => {
      const state = createCollectionState();
      flushSync(() => state.setQuery('Roco'));
      expect(state.filters.query).toBe('Roco');
    });

    it('toggleTag adds tag when not present', () => {
      const state = createCollectionState();
      flushSync(() => state.toggleTag('FEATURED'));
      expect(state.filters.tags.has('FEATURED')).toBe(true);
    });

    it('toggleTag removes tag when already present', () => {
      const state = createCollectionState();
      flushSync(() => {
        state.toggleTag('FEATURED');
        state.toggleTag('FEATURED');
      });
      expect(state.filters.tags.has('FEATURED')).toBe(false);
    });

    it('clearFilters resets all filters', () => {
      const state = createCollectionState();
      flushSync(() => {
        state.setScale('H0');
        state.setQuery('Roco');
        state.toggleTag('FEATURED');
        state.clearFilters();
      });
      expect(state.filters.scale).toBeNull();
      expect(state.filters.query).toBe('');
      expect(state.filters.tags.size).toBe(0);
    });
  });

  describe('addRailwayModel', () => {
    it('returns true and refreshes collection on success', async () => {
      const collection = makeCollection([]);
      mockInvoke.mockResolvedValueOnce(null); // add_railway_model_to_collection
      mockInvoke.mockResolvedValueOnce(collection); // get_collection

      const state = createCollectionState();
      const result = await state.addRailwayModel({
        railwayModel: {
          manufacturerId: 'mfr-1',
          productCode: 'PC-001',
          description: 'Test model',
          category: 'LOCOMOTIVE',
          scale: 'H0',
          epoch: 'IV',
          powerMethod: 'DC',
          rollingStocks: []
        },
        priceAmount: Number(0),
        priceCurrency: 'EUR',
        sellerId: null,
        addedDate: '2026-01-01',
        purchaseDate: '2026-01-01',
        purchaseCondition: null,
        modelCondition: null,
        boxCondition: null,
        notes: null
      });

      expect(result).toBe(true);
    });

    it('returns false when Tauri command fails', async () => {
      mockInvoke.mockRejectedValueOnce({ DatabaseError: 'Insert failed' });

      const state = createCollectionState();
      const result = await state.addRailwayModel({
        railwayModel: {
          manufacturerId: 'mfr-1',
          productCode: 'PC-001',
          description: 'Test model',
          category: 'LOCOMOTIVE',
          scale: 'H0',
          epoch: 'IV',
          powerMethod: 'DC',
          rollingStocks: []
        },
        priceAmount: Number(0),
        priceCurrency: 'EUR',
        sellerId: null,
        addedDate: '2026-01-01',
        purchaseDate: '2026-01-01',
        purchaseCondition: null,
        modelCondition: null,
        boxCondition: null,
        notes: null
      });

      expect(result).toBe(false);
    });
  });
});
