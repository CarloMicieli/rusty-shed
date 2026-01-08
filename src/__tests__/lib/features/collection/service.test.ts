import { describe, it, expect, vi, beforeEach } from 'vitest';

// Mock @tauri-apps/api/core BEFORE importing
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}));

// Mock toaster
vi.mock('$lib/toaster', () => ({
  toaster: {
    loading: vi.fn(),
    success: vi.fn(),
    error: vi.fn()
  }
}));

// Mock paraglide messages
vi.mock('$lib/paraglide/messages.js', () => ({
  collection_toast_loading: () => 'Loading...',
  collection_toast_success: () => 'Success',
  collection_toast_error: () => 'Error',
  collection_toast_retry: () => 'Retry'
}));

// Now import after mocks
import { collectionService } from '$lib/features/collection/service.svelte';
import type { CollectionView } from '$lib/bindings';
import { invoke, type InvokeArgs, type InvokeOptions } from '@tauri-apps/api/core';

const mockInvoke = vi.mocked(invoke);
type InvokeArgType = InvokeArgs | undefined;
type InvokeOptionType = InvokeOptions | undefined;
type Handler = (args?: InvokeArgType) => unknown;

// Helper for Tauri mock
const tauriMock = {
  handlers: new Map<string, Handler>(),
  delays: new Map<string, number>(),

  mockCommand<T>(command: string, response: T) {
    this.handlers.set(command, () => response);
  },

  mockCommandError(command: string, error: unknown) {
    this.handlers.set(command, () => {
      throw error;
    });
  },

  mockCommandWithDelay<T>(command: string, delay: number, response: T) {
    this.delays.set(command, delay);
    this.mockCommand(command, response);
  },

  reset() {
    this.handlers.clear();
    this.delays.clear();
    mockInvoke.mockReset();
    // Re-apply the implementation
    mockInvoke.mockImplementation(
      async (command: string, args?: InvokeArgType, _options?: InvokeOptionType) => {
        const handler = this.handlers.get(command);
        const delay = this.delays.get(command) || 0;

        if (!handler) {
          throw new Error(`Unmocked Tauri command: ${command}`);
        }

        if (delay > 0) {
          await new Promise((resolve) => setTimeout(resolve, delay));
        }

        return handler(args);
      }
    );
  }
};

// Initial setup
mockInvoke.mockImplementation(
  async (command: string, args?: InvokeArgType, _options?: InvokeOptionType) => {
    const handler = tauriMock.handlers.get(command);
    const delay = tauriMock.delays.get(command) || 0;

    if (!handler) {
      throw new Error(`Unmocked Tauri command: ${command}`);
    }

    if (delay > 0) {
      await new Promise((resolve) => setTimeout(resolve, delay));
    }

    return handler(args);
  }
);

describe('CollectionService (Read-Only)', () => {
  beforeEach(async () => {
    tauriMock.reset();
    vi.clearAllMocks();
    // Ensure we start with empty state
    const emptyCollection: CollectionView = {
      id: 'collection-1',
      name: 'My Collection',
      summary: {
        locomotives_count: 0,
        passenger_cars_count: 0,
        freight_cars_count: 0,
        train_sets_count: 0,
        railcars_count: 0,
        electric_multiple_units_count: 0,
        starter_sets_count: 0
      },
      total_value: null,
      items: []
    };
    tauriMock.mockCommand('get_collection', emptyCollection);
    collectionService.clearFilters();
    await collectionService.fetchCollection();
  });

  const mockCollection: CollectionView = {
    id: 'collection-1',
    name: 'My Collection',
    summary: {
      locomotives_count: 2,
      passenger_cars_count: 1,
      freight_cars_count: 0,
      train_sets_count: 0,
      railcars_count: 0,
      electric_multiple_units_count: 0,
      starter_sets_count: 0
    },
    total_value: { amount: 50000n, currency: 'EUR' },
    items: [
      {
        id: '1',
        railway_model: {
          railway_model_id: 'model-1',
          manufacturer: 'Roco',
          product_code: '79894',
          description: 'BR 185 Electric Locomotive',
          scale: 'H0',
          epoch: 'VI'
        },
        added_date: '2024-01-01',
        removed_date: null,
        purchase_condition: null,
        model_condition: null,
        box_condition: null,
        notes: null,
        rolling_stocks: [],
        purchase_info: null
      },
      {
        id: '2',
        railway_model: {
          railway_model_id: 'model-2',
          manufacturer: 'Märklin',
          product_code: '37712',
          description: 'ICE 3 High Speed Train',
          scale: 'H0',
          epoch: 'VI'
        },
        added_date: '2024-01-02',
        removed_date: null,
        purchase_condition: null,
        model_condition: null,
        box_condition: null,
        notes: null,
        rolling_stocks: [],
        purchase_info: null
      },
      {
        id: '3',
        railway_model: {
          railway_model_id: 'model-3',
          manufacturer: 'Fleischmann',
          product_code: '4170',
          description: 'Tank Wagon',
          scale: 'N',
          epoch: 'IV'
        },
        added_date: '2024-01-03',
        removed_date: null,
        purchase_condition: null,
        model_condition: null,
        box_condition: null,
        notes: null,
        rolling_stocks: [],
        purchase_info: null
      }
    ]
  };

  describe('fetchCollection', () => {
    it('should load collection successfully', async () => {
      tauriMock.mockCommand('get_collection', mockCollection);

      await collectionService.fetchCollection();

      expect(collectionService.collection).toEqual(mockCollection);
      expect(collectionService.rawItems).toHaveLength(3);
      expect(collectionService.isLoading).toBe(false);
    });

    it('should set loading state during fetch', async () => {
      tauriMock.mockCommandWithDelay('get_collection', 50, mockCollection);

      const fetchPromise = collectionService.fetchCollection();

      expect(collectionService.isLoading).toBe(true);

      await fetchPromise;

      expect(collectionService.isLoading).toBe(false);
    });

    it('should handle fetch errors gracefully', async () => {
      const error = { DatabaseError: 'Connection failed' };
      tauriMock.mockCommandError('get_collection', error);

      await collectionService.fetchCollection();

      expect(collectionService.rawItems).toEqual([]);
      expect(collectionService.isLoading).toBe(false);
    });
  });

  describe('filtering', () => {
    beforeEach(async () => {
      tauriMock.mockCommand('get_collection', mockCollection);
      await collectionService.fetchCollection();
    });

    it('should filter by query text', () => {
      collectionService.setQuery('ICE');

      expect(collectionService.filteredItems).toHaveLength(1);
      expect(collectionService.filteredItems[0].railway_model.description).toContain('ICE');
    });

    it('should filter by scale', () => {
      collectionService.setScale('N');

      expect(collectionService.filteredItems).toHaveLength(1);
      expect(collectionService.filteredItems[0].railway_model.scale).toBe('N');
    });

    it('should combine multiple filters', () => {
      collectionService.setQuery('Roco');
      collectionService.setScale('H0');

      expect(collectionService.filteredItems).toHaveLength(1);
      expect(collectionService.filteredItems[0].id).toBe('1');
    });

    it('should clear all filters', () => {
      collectionService.setQuery('test');
      collectionService.setScale('H0');

      collectionService.clearFilters();

      expect(collectionService.filters.query).toBe('');
      expect(collectionService.filters.scale).toBeNull();
      expect(collectionService.filters.tags.size).toBe(0);
      expect(collectionService.filteredItems).toHaveLength(3);
    });
  });

  describe('derived values', () => {
    it('should compute totalCount', async () => {
      tauriMock.mockCommand('get_collection', mockCollection);
      await collectionService.fetchCollection();

      expect(collectionService.totalCount).toBe(3);
    });
  });

  // CRUD operations are commented out in the service - tests disabled
  describe.skip('CRUD operations (not yet implemented)', () => {
    it('createItem - will be implemented when backend command is available', () => {});
    it('updateItem - will be implemented when backend command is available', () => {});
    it('deleteItem - will be implemented when backend command is available', () => {});
  });
});
