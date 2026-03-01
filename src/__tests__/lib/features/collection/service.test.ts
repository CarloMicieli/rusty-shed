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
import { createCollectionState } from '$lib/features/collection/CollectionState.svelte';
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

describe('CollectionState (Read-Only)', () => {
  let collectionService: ReturnType<typeof createCollectionState>;

  beforeEach(() => {
    collectionService = createCollectionState();
    tauriMock.reset();
    vi.clearAllMocks();
    // Ensure we start with empty state
    const emptyCollection: CollectionView = {
      id: 'collection-1',
      name: 'My Collection',
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
      items: []
    };
    tauriMock.mockCommand('get_collection', emptyCollection);
    collectionService.clearFilters();
  });

  const mockCollection: CollectionView = {
    id: 'collection-1',
    name: 'My Collection',
    summary: {
      locomotivesCount: 2,
      passengerCarsCount: 1,
      freightCarsCount: 0,
      trainSetsCount: 0,
      railcarsCount: 0,
      electricMultipleUnitsCount: 0,
      starterSetsCount: 0
    },
    totalValue: { amount: 50000n, currency: 'EUR' },
    items: [
      {
        id: '1',
        railwayModel: {
          railwayModelId: 'model-1',
          manufacturer: 'Roco',
          productCode: '79894',
          description: 'BR 185 Electric Locomotive',
          scale: 'H0',
          epoch: 'VI',
          category: 'LOCOMOTIVES',
          powerMethod: 'AC'
        },
        addedDate: '2024-01-01',
        removedDate: null,
        purchaseCondition: null,
        modelCondition: null,
        boxCondition: null,
        notes: null,
        rollingStocks: [],
        purchaseInfo: null
      },
      {
        id: '2',
        railwayModel: {
          railwayModelId: 'model-2',
          manufacturer: 'Märklin',
          productCode: '37712',
          description: 'ICE 3 High Speed Train',
          scale: 'H0',
          epoch: 'VI',
          category: 'TRAIN_SETS',
          powerMethod: 'DC'
        },
        addedDate: '2024-01-02',
        removedDate: null,
        purchaseCondition: null,
        modelCondition: null,
        boxCondition: null,
        notes: null,
        rollingStocks: [],
        purchaseInfo: null
      },
      {
        id: '3',
        railwayModel: {
          railwayModelId: 'model-3',
          manufacturer: 'Fleischmann',
          productCode: '4170',
          description: 'Tank Wagon',
          scale: 'N',
          epoch: 'IV',
          category: 'FREIGHT_CARS',
          powerMethod: 'DC'
        },
        addedDate: '2024-01-03',
        removedDate: null,
        purchaseCondition: null,
        modelCondition: null,
        boxCondition: null,
        notes: null,
        rollingStocks: [],
        purchaseInfo: null
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
      expect(collectionService.filteredItems[0].railwayModel.description).toContain('ICE');
    });

    it('should filter by scale', () => {
      collectionService.setScale('N');

      expect(collectionService.filteredItems).toHaveLength(1);
      expect(collectionService.filteredItems[0].railwayModel.scale).toBe('N');
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
  describe('CRUD operations (not yet implemented)', () => {
    it.todo('createItem - will be implemented when backend command is available');
    it.todo('updateItem - will be implemented when backend command is available');
    it.todo('deleteItem - will be implemented when backend command is available');
  });
});
