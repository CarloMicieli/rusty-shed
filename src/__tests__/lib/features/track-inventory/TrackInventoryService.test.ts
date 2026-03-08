import { describe, it, expect, vi, beforeEach } from 'vitest';
import type {
  TrackInventoryListItem,
  TrackInventoryView,
  TrackProductView,
  NewTrackInventoryArgs,
  RenameTrackInventoryArgs,
  AddTrackPurchaseArgs,
  CreateTrackProductArgs,
  Currency
} from '$lib/bindings';

// Mock @tauri-apps/api/core
vi.mock('@tauri-apps/api/core', () => {
  return {
    invoke: vi.fn() as any
  };
});

// Mock svelte context functions
vi.mock('svelte', async () => {
  const actual = await vi.importActual('svelte');
  return {
    ...actual,
    getContext: vi.fn() as any,
    setContext: vi.fn() as any
  };
});

// Now import the service
import { TrackInventoryService } from '$lib/features/track-inventory/services/TrackInventoryService.svelte';
import { invoke as mockInvoke } from '@tauri-apps/api/core';

describe('TrackInventoryService', () => {
  let service: TrackInventoryService;

  beforeEach(() => {
    service = new TrackInventoryService();
    vi.clearAllMocks();
  });

  describe('fetchInventories', () => {
    it('should fetch all track inventories', async () => {
      const mockInventories: TrackInventoryListItem[] = [
        {
          id: 'inv-1',
          name: 'Main Layout',
          description: 'Primary track layout',
          track_count: 5,
          purchase_count: 3,
          total_value: { amount: 15000n, currency: 'EUR' as Currency }
        } as any,
        {
          id: 'inv-2',
          name: 'Storage',
          description: null,
          track_count: 8,
          purchase_count: 0,
          total_value: { amount: 0n, currency: 'EUR' as Currency }
        } as any
      ];

      (mockInvoke as any).mockResolvedValue(mockInventories);

      const result = await service.fetchInventories();

      expect(result).toEqual(mockInventories);
      expect(mockInvoke).toHaveBeenCalledWith('get_track_inventories');
    });

    it('should handle empty inventory list', async () => {
      (mockInvoke as any).mockResolvedValue([]);

      const result = await service.fetchInventories();

      expect(result).toEqual([]);
      expect(mockInvoke).toHaveBeenCalledWith('get_track_inventories');
    });

    it('should throw error when fetch fails', async () => {
      const error = new Error('Database connection failed');
      (mockInvoke as any).mockRejectedValue(error);

      await expect(service.fetchInventories()).rejects.toThrow('Database connection failed');
    });
  });

  describe('fetchInventory', () => {
    it('should fetch a single track inventory', async () => {
      const mockInventory: TrackInventoryView = {
        id: 'inv-1',
        name: 'Main Layout',
        description: 'Primary track layout',
        track_count: 5,
        purchase_count: 3,
        total_value: { amount: 15000n, currency: 'EUR' as Currency },
        items: []
      } as any;

      (mockInvoke as any).mockResolvedValue(mockInventory);

      const result = await service.fetchInventory('inv-1');

      expect(result).toEqual(mockInventory);
      expect(mockInvoke).toHaveBeenCalledWith('get_track_inventory', { id: 'inv-1' });
    });

    it('should throw error when inventory not found', async () => {
      const error = new Error('Inventory not found');
      (mockInvoke as any).mockRejectedValue(error);

      await expect(service.fetchInventory('invalid-id')).rejects.toThrow('Inventory not found');
    });
  });

  describe('fetchProducts', () => {
    it('should fetch all track products', async () => {
      const mockProducts: TrackProductView[] = [
        {
          track_id: 'track-1',
          manufacturer_name: 'Märklin',
          product_code: 'C-100',
          description: 'Straight Track',
          scale: 'H0',
          power_method: 'AC',
          track_type: 'STRAIGHT',
          total_length_mm: 1000,
          purchase_count: 5,
          total_value: { amount: 2500n, currency: 'EUR' as Currency }
        } as any
      ];

      (mockInvoke as any).mockResolvedValue(mockProducts);

      const result = await service.fetchProducts();

      expect(result).toEqual(mockProducts);
      expect(mockInvoke).toHaveBeenCalledWith('get_track_products');
    });

    it('should handle empty product list', async () => {
      (mockInvoke as any).mockResolvedValue([]);

      const result = await service.fetchProducts();

      expect(result).toEqual([]);
    });

    it('should throw error on fetch failure', async () => {
      (mockInvoke as any).mockRejectedValue(new Error('API error'));

      await expect(service.fetchProducts()).rejects.toThrow('API error');
    });
  });

  describe('createInventory', () => {
    it('should create a new track inventory and return ID', async () => {
      const input: NewTrackInventoryArgs = {
        name: 'New Layout',
        description: 'Test layout'
      };

      (mockInvoke as any).mockResolvedValue('inv-new-123');

      const result = await service.createInventory(input);

      expect(result).toBe('inv-new-123');
      expect(mockInvoke).toHaveBeenCalledWith('create_track_inventory', { input });
    });

    it('should handle null description', async () => {
      const input: NewTrackInventoryArgs = {
        name: 'Simple Layout',
        description: null
      };

      (mockInvoke as any).mockResolvedValue('inv-simple-456');

      const result = await service.createInventory(input);

      expect(result).toBe('inv-simple-456');
    });

    it('should throw error on creation failure', async () => {
      const input: NewTrackInventoryArgs = {
        name: 'Layout',
        description: null
      };

      (mockInvoke as any).mockRejectedValue(new Error('Name already exists'));

      await expect(service.createInventory(input)).rejects.toThrow('Name already exists');
    });
  });

  describe('renameInventory', () => {
    it('should rename an existing inventory', async () => {
      const input: RenameTrackInventoryArgs = {
        id: 'inv-1',
        name: 'Updated Name'
      } as any;

      (mockInvoke as any).mockResolvedValue(undefined);

      await service.renameInventory(input);

      expect(mockInvoke).toHaveBeenCalledWith('rename_track_inventory', { input });
    });

    it('should throw error when renaming fails', async () => {
      const input: RenameTrackInventoryArgs = {
        id: 'invalid-id',
        name: 'New Name'
      } as any;

      (mockInvoke as any).mockRejectedValue(new Error('Inventory not found'));

      await expect(service.renameInventory(input)).rejects.toThrow('Inventory not found');
    });

    it('should handle duplicate names', async () => {
      const input: RenameTrackInventoryArgs = {
        id: 'inv-1',
        name: 'Existing Name'
      } as any;

      (mockInvoke as any).mockRejectedValue(new Error('Name already exists'));

      await expect(service.renameInventory(input)).rejects.toThrow('Name already exists');
    });
  });

  describe('deleteInventory', () => {
    it('should delete an inventory', async () => {
      (mockInvoke as any).mockResolvedValue(undefined);

      await service.deleteInventory('inv-1');

      expect(mockInvoke).toHaveBeenCalledWith('delete_track_inventory', { id: 'inv-1' });
    });

    it('should throw error when deletion fails', async () => {
      (mockInvoke as any).mockRejectedValue(new Error('Inventory not found'));

      await expect(service.deleteInventory('invalid-id')).rejects.toThrow('Inventory not found');
    });

    it('should throw error on constraint violation', async () => {
      (mockInvoke as any).mockRejectedValue(new Error('Cannot delete: inventory has items'));

      await expect(service.deleteInventory('inv-with-items')).rejects.toThrow(
        'Cannot delete: inventory has items'
      );
    });
  });

  describe('addPurchase', () => {
    it('should add a purchase to an inventory', async () => {
      const input: AddTrackPurchaseArgs = {
        inventory_id: 'inv-1',
        track_id: 'track-1',
        quantity: 5,
        price: { amount: BigInt(500), currency: 'EUR' as Currency },
        seller_id: 'seller-1',
        purchase_date: '2025-03-08'
      } as any;

      (mockInvoke as any).mockResolvedValue(undefined);

      await service.addPurchase(input);

      expect(mockInvoke).toHaveBeenCalledWith('add_track_purchase', { input });
    });

    it('should handle null seller_id', async () => {
      const input: AddTrackPurchaseArgs = {
        inventory_id: 'inv-1',
        track_id: 'track-1',
        quantity: 3,
        price: { amount: BigInt(300), currency: 'USD' as Currency },
        seller_id: null,
        purchase_date: '2025-03-08'
      } as any;

      (mockInvoke as any).mockResolvedValue(undefined);

      await service.addPurchase(input);

      expect(mockInvoke).toHaveBeenCalledWith('add_track_purchase', { input });
    });

    it('should throw error when adding purchase fails', async () => {
      const input: AddTrackPurchaseArgs = {
        inventory_id: 'invalid',
        track_id: 'track-1',
        quantity: 1,
        price: { amount: BigInt(100), currency: 'EUR' as Currency },
        seller_id: null,
        purchase_date: '2025-03-08'
      } as any;

      (mockInvoke as any).mockRejectedValue(new Error('Inventory not found'));

      await expect(service.addPurchase(input)).rejects.toThrow('Inventory not found');
    });
  });

  describe('createProduct', () => {
    it('should create a new track product and return ID', async () => {
      const input: CreateTrackProductArgs = {
        manufacturerId: 'mfg-1',
        productCode: 'T-100',
        description: 'New Track',
        power_method: 'AC',
        track_type: 'STRAIGHT',
        total_length_mm: 1000
      } as any;

      (mockInvoke as any).mockResolvedValue('track-new-789');

      const result = await service.createProduct(input);

      expect(result).toBe('track-new-789');
      expect(mockInvoke).toHaveBeenCalledWith('create_track_product', { input });
    });

    it('should throw error on product creation failure', async () => {
      const input: CreateTrackProductArgs = {
        manufacturerId: 'mfg-1',
        productCode: 'DUPLICATE',
        description: 'Track',
        power_method: 'DC',
        track_type: 'CURVED',
        total_length_mm: 500
      } as any;

      (mockInvoke as any).mockRejectedValue(new Error('Product code already exists'));

      await expect(service.createProduct(input)).rejects.toThrow('Product code already exists');
    });
  });

  describe('setItemRequired', () => {
    it('should set required quantity for a track item', async () => {
      (mockInvoke as any).mockResolvedValue(undefined);

      await service.setItemRequired('inv-1', 'track-1', 10);

      expect(mockInvoke).toHaveBeenCalledWith('set_item_required', {
        input: {
          inventory_id: 'inv-1',
          track_id: 'track-1',
          required: 10
        }
      });
    });

    it('should handle zero required quantity', async () => {
      (mockInvoke as any).mockResolvedValue(undefined);

      await service.setItemRequired('inv-1', 'track-2', 0);

      expect(mockInvoke).toHaveBeenCalledWith('set_item_required', {
        input: {
          inventory_id: 'inv-1',
          track_id: 'track-2',
          required: 0
        }
      });
    });

    it('should throw error when setting required fails', async () => {
      (mockInvoke as any).mockRejectedValue(new Error('Item not found in inventory'));

      await expect(service.setItemRequired('inv-1', 'invalid-track', 5)).rejects.toThrow(
        'Item not found in inventory'
      );
    });

    it('should throw error on invalid inventory', async () => {
      (mockInvoke as any).mockRejectedValue(new Error('Inventory not found'));

      await expect(service.setItemRequired('invalid-inv', 'track-1', 3)).rejects.toThrow(
        'Inventory not found'
      );
    });
  });

  describe('Tauri invoke calls', () => {
    it('should invoke correct command names', async () => {
      (mockInvoke as any).mockResolvedValue(null);

      await service.fetchInventories();
      expect(mockInvoke).toHaveBeenCalledWith('get_track_inventories');

      await service.fetchInventory('id');
      expect(mockInvoke).toHaveBeenCalledWith('get_track_inventory', { id: 'id' });

      await service.fetchProducts();
      expect(mockInvoke).toHaveBeenCalledWith('get_track_products');

      await service.deleteInventory('id');
      expect(mockInvoke).toHaveBeenCalledWith('delete_track_inventory', { id: 'id' });
    });
  });

  describe('Error handling', () => {
    it('should propagate Tauri errors correctly', async () => {
      const tauriError = {
        code: 'EINVAL',
        message: 'Invalid argument'
      };

      (mockInvoke as any).mockRejectedValue(new Error(JSON.stringify(tauriError)));

      await expect(service.fetchInventories()).rejects.toThrow();
    });

    it('should handle timeout errors', async () => {
      (mockInvoke as any).mockRejectedValue(new Error('Timeout: operation took too long'));

      await expect(service.fetchInventories()).rejects.toThrow('Timeout');
    });
  });
});
