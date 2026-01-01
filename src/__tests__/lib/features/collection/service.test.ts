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
import type { CollectionItemLite, CreateCollectionItemInput } from '$lib/bindings';
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

describe('CollectionService', () => {
  beforeEach(async () => {
    tauriMock.reset();
    vi.clearAllMocks();
    // Ensure we start with empty state by mocking an empty fetch
    tauriMock.mockCommand('list_collection_items', []);
    collectionService.clearFilters();
    await collectionService.fetchCollection();
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

      await collectionService.fetchCollection();

      expect(collectionService.rawItems).toEqual(mockItems);
      expect(collectionService.isLoading).toBe(false);
    });

    it('should set loading state during fetch', async () => {
      tauriMock.mockCommandWithDelay('list_collection_items', 50, mockItems);

      const fetchPromise = collectionService.fetchCollection();

      expect(collectionService.isLoading).toBe(true);

      await fetchPromise;

      expect(collectionService.isLoading).toBe(false);
    });

    it('should handle fetch errors gracefully', async () => {
      const error = { DatabaseError: 'Connection failed' };
      tauriMock.mockCommandError('list_collection_items', error);

      await collectionService.fetchCollection();

      expect(collectionService.rawItems).toEqual([]);
      expect(collectionService.isLoading).toBe(false);
    });

    it('should update query filter when provided', async () => {
      tauriMock.mockCommand('list_collection_items', mockItems);

      await collectionService.fetchCollection('BR 185');

      expect(collectionService.filters.query).toBe('BR 185');
    });
  });

  describe('filtering', () => {
    beforeEach(async () => {
      tauriMock.mockCommand('list_collection_items', mockItems);
      await collectionService.fetchCollection();
    });

    it('should filter by query text', () => {
      collectionService.setQuery('ICE');

      expect(collectionService.filteredItems).toHaveLength(1);
      expect(collectionService.filteredItems[0].title).toBe('ICE 3');
    });

    it('should filter by scale', () => {
      collectionService.setScale('N');

      expect(collectionService.filteredItems).toHaveLength(1);
      expect(collectionService.filteredItems[0].scale).toBe('N');
    });

    it('should filter by tags', () => {
      collectionService.toggleTag('electric');

      expect(collectionService.filteredItems).toHaveLength(2);
      expect(collectionService.filteredItems.every((item) => item.tags.includes('electric'))).toBe(
        true
      );
    });

    it('should combine multiple filters', () => {
      collectionService.setQuery('Roco');
      collectionService.setScale('H0');
      collectionService.toggleTag('electric');

      expect(collectionService.filteredItems).toHaveLength(1);
      expect(collectionService.filteredItems[0].id).toBe('1');
    });

    it('should clear all filters', () => {
      collectionService.setQuery('test');
      collectionService.setScale('H0');
      collectionService.toggleTag('electric');

      collectionService.clearFilters();

      expect(collectionService.filters.query).toBe('');
      expect(collectionService.filters.scale).toBeNull();
      expect(collectionService.filters.tags.size).toBe(0);
      expect(collectionService.filteredItems).toEqual(mockItems);
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

      const createdItem: CollectionItemLite = {
        id: 'real-id-123',
        createdAt: '2024-01-04T00:00:00Z',
        ...input,
        description: 'Test'
      };
      tauriMock.mockCommandWithDelay('create_collection_item', 50, createdItem);

      const createPromise = collectionService.createItem(input);

      expect(collectionService.rawItems.length).toBe(1);
      expect(collectionService.rawItems[0].id).toMatch(/^temp-/);
      expect(collectionService.rawItems[0].title).toBe('New Loco');

      await createPromise;

      expect(collectionService.rawItems.length).toBe(1);
      expect(collectionService.rawItems[0].id).toBe('real-id-123');
      expect(collectionService.rawItems[0].title).toBe('New Loco');
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

      const createPromise = collectionService.createItem(input);

      expect(collectionService.rawItems.length).toBe(1);

      const result = await createPromise;

      expect(result).toBeNull();
      expect(collectionService.rawItems).toEqual([]);
    });
  });

  describe('updateItem - optimistic updates', () => {
    beforeEach(async () => {
      tauriMock.mockCommand('list_collection_items', [mockItems[0]]);
      await collectionService.fetchCollection();
    });

    it('should optimistically update item before backend confirmation', async () => {
      const updatedItem: CollectionItemLite = {
        ...mockItems[0],
        title: 'Updated Title'
      };

      tauriMock.mockCommandWithDelay('update_collection_item', 50, updatedItem);

      const updatePromise = collectionService.updateItem({
        id: '1',
        brand: mockItems[0].brand,
        catalogNumber: mockItems[0].catalogNumber,
        title: 'Updated Title',
        scale: mockItems[0].scale,
        powerSystem: mockItems[0].powerSystem,
        description: mockItems[0].description,
        tags: mockItems[0].tags
      });

      expect(collectionService.rawItems[0].title).toBe('Updated Title');

      await updatePromise;

      expect(collectionService.rawItems[0].title).toBe('Updated Title');
    });

    it('should revert to snapshot on update error', async () => {
      const originalTitle = mockItems[0].title;
      const error = { DatabaseError: 'Update failed' };

      tauriMock.mockCommandErrorWithDelay('update_collection_item', 50, error);

      const updatePromise = collectionService.updateItem({
        id: '1',
        brand: mockItems[0].brand,
        catalogNumber: mockItems[0].catalogNumber,
        title: 'Failed Update',
        scale: mockItems[0].scale,
        powerSystem: mockItems[0].powerSystem,
        description: mockItems[0].description,
        tags: mockItems[0].tags
      });

      expect(collectionService.rawItems[0].title).toBe('Failed Update');

      const result = await updatePromise;

      expect(result).toBeNull();
      expect(collectionService.rawItems[0].title).toBe(originalTitle);
    });
  });

  describe('deleteItem - optimistic updates', () => {
    beforeEach(async () => {
      tauriMock.mockCommand('list_collection_items', [mockItems[0], mockItems[1]]);
      await collectionService.fetchCollection();
    });

    it('should optimistically remove item before backend confirmation', async () => {
      tauriMock.mockCommandWithDelay('delete_collection_item', 50, undefined);

      const deletePromise = collectionService.deleteItem('1');

      expect(collectionService.rawItems).toHaveLength(1);
      expect(collectionService.rawItems[0].id).toBe('2');

      const result = await deletePromise;

      expect(result).toBe(true);
      expect(collectionService.rawItems).toHaveLength(1);
    });

    it('should revert delete on error', async () => {
      const error = { PermissionDenied: 'Cannot delete' };

      tauriMock.mockCommandErrorWithDelay('delete_collection_item', 50, error);

      const deletePromise = collectionService.deleteItem('1');

      expect(collectionService.rawItems).toHaveLength(1);

      const result = await deletePromise;

      expect(result).toBe(false);
      expect(collectionService.rawItems).toHaveLength(2);
      expect(collectionService.rawItems.find((i) => i.id === '1')).toBeDefined();
    });
  });

  describe('derived values', () => {
    it('should compute totalCount', async () => {
      tauriMock.mockCommand('list_collection_items', mockItems);
      await collectionService.fetchCollection();

      expect(collectionService.totalCount).toBe(3);
    });

    it('should compute availableTags from items', async () => {
      tauriMock.mockCommand('list_collection_items', mockItems);
      await collectionService.fetchCollection();

      const tags = collectionService.availableTags;
      expect(tags).toContain('electric');
      expect(tags).toContain('freight');
      expect(tags).toContain('passenger');
    });
  });
});
