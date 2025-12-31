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
import { collectionStore } from '$lib/stores/collectionStore.svelte';
import type { CollectionItemLite, CreateCollectionItemInput } from '$lib/bindings';
import { invoke, type InvokeArgs, type InvokeOptions } from '@tauri-apps/api/core';
import { SvelteSet } from 'svelte/reactivity';

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

  mockCommandErrorWithDelay(command: string, delay: number, error: unknown) {
    this.delays.set(command, delay);
    this.mockCommandError(command, error);
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

describe('CollectionStore', () => {
  beforeEach(() => {
    tauriMock.reset();
    vi.clearAllMocks();
    // Reset store state
    collectionStore.rawItems = [];
    collectionStore.filters = { query: '', scale: null, tags: new SvelteSet<string>() };
    collectionStore.isLoading = false;
  });

  const mockItems: CollectionItemLite[] = [
    {
      id: '1',
      createdAt: '2024-01-01T00:00:00Z',
      brand: 'Roco',
      catalogNumber: '79894',
      title: 'BR 185',
      scale: 'H0',
      powerSystem: 'AC',
      description: 'Electric Locomotive',
      tags: ['electric', 'modern']
    },
    {
      id: '2',
      createdAt: '2024-01-02T00:00:00Z',
      brand: 'Märklin',
      catalogNumber: '37712',
      title: 'ICE 3',
      scale: 'H0',
      powerSystem: 'AC',
      description: 'High Speed Train',
      tags: ['electric', 'passenger']
    },
    {
      id: '3',
      createdAt: '2024-01-03T00:00:00Z',
      brand: 'Fleischmann',
      catalogNumber: '4170',
      title: 'Tank Wagon',
      scale: 'N',
      powerSystem: 'None',
      description: null,
      tags: ['freight']
    }
  ];

  describe('fetchCollection', () => {
    it('should load collection items successfully', async () => {
      tauriMock.mockCommand('list_collection_items', mockItems);

      await collectionStore.fetchCollection();

      // Assert synchronously - $state updates immediately
      expect(collectionStore.rawItems).toEqual(mockItems);
      expect(collectionStore.isLoading).toBe(false);
    });

    it('should set loading state during fetch', async () => {
      tauriMock.mockCommandWithDelay('list_collection_items', 50, mockItems);

      const fetchPromise = collectionStore.fetchCollection();

      // Loading state updates synchronously
      expect(collectionStore.isLoading).toBe(true);

      await fetchPromise;

      expect(collectionStore.isLoading).toBe(false);
    });

    it('should handle fetch errors gracefully', async () => {
      const error = { DatabaseError: 'Connection failed' };
      tauriMock.mockCommandError('list_collection_items', error);

      await collectionStore.fetchCollection();

      expect(collectionStore.rawItems).toEqual([]);
      expect(collectionStore.isLoading).toBe(false);
    });

    it('should update query filter when provided', async () => {
      tauriMock.mockCommand('list_collection_items', mockItems);

      await collectionStore.fetchCollection('BR 185');

      expect(collectionStore.filters.query).toBe('BR 185');
    });
  });

  describe('filtering', () => {
    beforeEach(async () => {
      collectionStore.rawItems = mockItems;
    });

    it('should filter by query text', () => {
      collectionStore.setQuery('ICE');

      // $derived updates synchronously
      expect(collectionStore.filteredItems).toHaveLength(1);
      expect(collectionStore.filteredItems[0].title).toBe('ICE 3');
    });

    it('should filter by scale', () => {
      collectionStore.setScale('N');

      expect(collectionStore.filteredItems).toHaveLength(1);
      expect(collectionStore.filteredItems[0].scale).toBe('N');
    });

    it('should filter by tags', () => {
      collectionStore.toggleTag('electric');

      expect(collectionStore.filteredItems).toHaveLength(2);
      expect(collectionStore.filteredItems.every((item) => item.tags.includes('electric'))).toBe(
        true
      );
    });

    it('should combine multiple filters', () => {
      collectionStore.setQuery('Roco');
      collectionStore.setScale('H0');
      collectionStore.toggleTag('electric');

      expect(collectionStore.filteredItems).toHaveLength(1);
      expect(collectionStore.filteredItems[0].id).toBe('1');
    });

    it('should clear all filters', () => {
      collectionStore.setQuery('test');
      collectionStore.setScale('H0');
      collectionStore.toggleTag('electric');

      collectionStore.clearFilters();

      expect(collectionStore.filters.query).toBe('');
      expect(collectionStore.filters.scale).toBeNull();
      expect(collectionStore.filters.tags.size).toBe(0);
      expect(collectionStore.filteredItems).toEqual(mockItems);
    });
  });

  describe('createItem - optimistic updates', () => {
    it('should optimistically add item before backend confirmation', async () => {
      const input: CreateCollectionItemInput = {
        brand: 'Piko',
        catalogNumber: '51234',
        title: 'New Loco',
        scale: 'H0',
        powerSystem: 'DC',
        description: 'Test',
        tags: ['test']
      };

      // Mock with delay to observe optimistic state
      const createdItem: CollectionItemLite = {
        id: 'real-id-123',
        createdAt: '2024-01-04T00:00:00Z',
        ...input,
        description: 'Test'
      };
      tauriMock.mockCommandWithDelay('create_collection_item', 50, createdItem);

      const createPromise = collectionStore.createItem(input);

      // Assert optimistic state immediately - should have temp item
      expect(collectionStore.rawItems.length).toBe(1);
      expect(collectionStore.rawItems[0].id).toMatch(/^temp-/);
      expect(collectionStore.rawItems[0].title).toBe('New Loco');

      await createPromise;

      // After completion, temp item should be replaced with real item
      expect(collectionStore.rawItems.length).toBe(1);
      expect(collectionStore.rawItems[0].id).toBe('real-id-123');
      expect(collectionStore.rawItems[0].title).toBe('New Loco');
    });

    it('should revert optimistic update on error', async () => {
      const input: CreateCollectionItemInput = {
        brand: 'Piko',
        catalogNumber: '51234',
        title: 'New Loco',
        scale: 'H0',
        powerSystem: 'DC',
        description: null,
        tags: []
      };

      const error = { ValidationError: { catalogNumber: 'Already exists' } };
      tauriMock.mockCommandErrorWithDelay('create_collection_item', 50, error);

      const createPromise = collectionStore.createItem(input);

      // Assert optimistic state
      expect(collectionStore.rawItems.length).toBe(1);

      const result = await createPromise;

      // After error, should revert to empty
      expect(result).toBeNull();
      expect(collectionStore.rawItems).toEqual([]);
    });
  });

  describe('updateItem - optimistic updates', () => {
    beforeEach(() => {
      collectionStore.rawItems = [mockItems[0]];
    });

    it('should optimistically update item before backend confirmation', async () => {
      const updatedItem: CollectionItemLite = {
        ...mockItems[0],
        title: 'Updated Title'
      };

      tauriMock.mockCommandWithDelay('update_collection_item', 50, updatedItem);

      const updatePromise = collectionStore.updateItem({
        id: '1',
        brand: mockItems[0].brand,
        catalogNumber: mockItems[0].catalogNumber,
        title: 'Updated Title',
        scale: mockItems[0].scale,
        powerSystem: mockItems[0].powerSystem,
        description: mockItems[0].description,
        tags: mockItems[0].tags
      });

      // Assert optimistic state
      expect(collectionStore.rawItems[0].title).toBe('Updated Title');

      await updatePromise;

      // Should still have updated title after backend confirms
      expect(collectionStore.rawItems[0].title).toBe('Updated Title');
    });

    it('should revert to snapshot on update error', async () => {
      const originalTitle = mockItems[0].title;
      const error = { DatabaseError: 'Update failed' };

      tauriMock.mockCommandErrorWithDelay('update_collection_item', 50, error);

      const updatePromise = collectionStore.updateItem({
        id: '1',
        brand: mockItems[0].brand,
        catalogNumber: mockItems[0].catalogNumber,
        title: 'Failed Update',
        scale: mockItems[0].scale,
        powerSystem: mockItems[0].powerSystem,
        description: mockItems[0].description,
        tags: mockItems[0].tags
      });

      // Assert optimistic state
      expect(collectionStore.rawItems[0].title).toBe('Failed Update');

      const result = await updatePromise;

      // After error, should revert
      expect(result).toBeNull();
      expect(collectionStore.rawItems[0].title).toBe(originalTitle);
    });
  });

  describe('deleteItem - optimistic updates', () => {
    beforeEach(() => {
      collectionStore.rawItems = [mockItems[0], mockItems[1]];
    });

    it('should optimistically remove item before backend confirmation', async () => {
      tauriMock.mockCommandWithDelay('delete_collection_item', 50, undefined);

      const deletePromise = collectionStore.deleteItem('1');

      // Assert optimistic state - item removed immediately
      expect(collectionStore.rawItems).toHaveLength(1);
      expect(collectionStore.rawItems[0].id).toBe('2');

      const result = await deletePromise;

      expect(result).toBe(true);
      expect(collectionStore.rawItems).toHaveLength(1);
    });

    it('should revert delete on error', async () => {
      const error = { PermissionDenied: 'Cannot delete' };

      tauriMock.mockCommandErrorWithDelay('delete_collection_item', 50, error);

      const deletePromise = collectionStore.deleteItem('1');

      // Assert optimistic state
      expect(collectionStore.rawItems).toHaveLength(1);

      const result = await deletePromise;

      // After error, should restore item
      expect(result).toBe(false);
      expect(collectionStore.rawItems).toHaveLength(2);
      expect(collectionStore.rawItems.find((i) => i.id === '1')).toBeDefined();
    });
  });

  describe('derived values', () => {
    it('should compute totalCount', () => {
      collectionStore.rawItems = mockItems;

      expect(collectionStore.totalCount).toBe(3);
    });

    it('should compute availableTags from items', () => {
      collectionStore.rawItems = mockItems;

      const tags = collectionStore.availableTags;
      expect(tags).toContain('electric');
      expect(tags).toContain('freight');
      expect(tags).toContain('passenger');
    });
  });
});
