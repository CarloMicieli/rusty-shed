import { invoke } from '@tauri-apps/api/core';
import type {
  NewTrackInventoryArgs,
  RenameTrackInventoryArgs,
  CreateTrackProductArgs,
  TrackInventoryListItem,
  TrackInventoryView,
  TrackProductView,
  Currency
} from '$lib/bindings';
import { getContext, setContext } from 'svelte';

const TRACK_INVENTORY_CONTEXT_KEY = Symbol('track-inventory-service');

/**
 * Service for managing track inventories
 */
export class TrackInventoryService {
  /**
   * Fetch all track inventories
   */
  async fetchInventories(): Promise<TrackInventoryListItem[]> {
    return await invoke<TrackInventoryListItem[]>('get_track_inventories');
  }

  /**
   * Fetch a single track inventory by ID
   */
  async fetchInventory(id: string): Promise<TrackInventoryView> {
    return await invoke<TrackInventoryView>('get_track_inventory', { id });
  }

  /**
   * Fetch all track products
   */
  async fetchProducts(): Promise<TrackProductView[]> {
    return await invoke<TrackProductView[]>('get_track_products');
  }

  /**
   * Create a new track inventory
   */
  async createInventory(input: NewTrackInventoryArgs): Promise<string> {
    return await invoke<string>('create_track_inventory', { input });
  }

  /**
   * Rename an existing track inventory
   */
  async renameInventory(input: RenameTrackInventoryArgs): Promise<void> {
    await invoke('rename_track_inventory', { input });
  }

  /**
   * Delete a track inventory
   */
  async deleteInventory(id: string): Promise<void> {
    await invoke('delete_track_inventory', { id });
  }

  /**
   * Add a purchase to a track inventory.
   * Uses plain numbers instead of bigint to avoid JSON.stringify serialization issues.
   */
  async addPurchase(input: {
    id: string;
    trackId: string;
    quantity: number;
    price: { amount: number; currency: Currency };
    sellerId: string | null;
    purchaseDate: string;
  }): Promise<void> {
    await invoke('add_track_purchase', { input });
  }

  /**
   * Create a new track product
   */
  async createProduct(input: CreateTrackProductArgs): Promise<string> {
    return await invoke<string>('create_track_product', { input });
  }

  /**
   * Set the required quantity for a track item in an inventory
   */
  async setItemRequired(inventoryId: string, trackId: string, required: number): Promise<void> {
    await invoke('set_item_required', {
      input: {
        inventoryId,
        trackId,
        required
      }
    });
  }
}

/**
 * Set the TrackInventoryService in Svelte context
 */
export function setTrackInventoryContext(service: TrackInventoryService): void {
  setContext(TRACK_INVENTORY_CONTEXT_KEY, service);
}

/**
 * Get the TrackInventoryService from Svelte context
 */
export function getTrackInventoryContext(): TrackInventoryService {
  const service = getContext<TrackInventoryService>(TRACK_INVENTORY_CONTEXT_KEY);
  if (!service) {
    throw new Error('TrackInventoryService not found in context. Did you forget to initialize it?');
  }
  return service;
}
