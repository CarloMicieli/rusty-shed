import { describe, it, expect, vi, beforeEach } from 'vitest';

// Mock @tauri-apps/api/core
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

// Mock paraglide
vi.mock('$lib/paraglide/messages.js', () => ({
  collection_toast_loading: () => 'Loading...',
  collection_toast_success: () => 'Success',
  collection_toast_error: () => 'Error',
  collection_toast_retry: () => 'Retry'
}));

import { wishlistService } from '$lib/features/wishlists/service.svelte';
import { invoke, type InvokeArgs } from '@tauri-apps/api/core';

const mockInvoke = vi.mocked(invoke);
type InvokeArgType = InvokeArgs | undefined;
type Handler = (args?: InvokeArgType) => unknown;

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
    mockInvoke.mockImplementation(async (command, args) => {
      const handler = this.handlers.get(command);
      const delay = this.delays.get(command) || 0;
      if (!handler) throw new Error(`Unmocked command: ${command}`);
      if (delay > 0) await new Promise((r) => setTimeout(r, delay));
      return handler(args);
    });
  }
};

describe('WishlistService', () => {
  beforeEach(async () => {
    tauriMock.reset();
    vi.clearAllMocks();
    tauriMock.mockCommand('get_wishlists', []);
    // Reset state
    // We can't access private fields to reset, but fetching empty list helps?
    // BUT WishlistService state relies on wishlists array.
    // We should probably rely on `createWishlistService` if we want fresh instances,
    // but the app uses singleton.
    // For now, mock empty fetch to clear state if possible.
    await wishlistService.fetchWishlists();
  });

  describe('fetchWishlists', () => {
    it('should load wishlists', async () => {
      const mockList = [
        { id: '1', name: 'Main', is_default: true, count: 0n, updated_at: '', total_value: {} }
      ];
      tauriMock.mockCommand('get_wishlists', mockList);

      await wishlistService.fetchWishlists();

      expect(wishlistService.wishlists).toEqual(mockList);
      expect(wishlistService.activeWishlistId).toBe('1');
    });
  });

  describe('createWishlist', () => {
    it('should optimistically create wishlist', async () => {
      const inputName = 'New List';
      const result = {
        id: 'real-id',
        name: inputName,
        is_default: false,
        count: 0n,
        updated_at: '',
        total_value: {}
      };

      tauriMock.mockCommandWithDelay('create_wishlist', 50, result);

      const promise = wishlistService.createWishlist(inputName);

      expect(wishlistService.wishlists.length).toBe(1);
      expect(wishlistService.wishlists[0].id).toMatch(/^temp-/);

      await promise;

      expect(wishlistService.wishlists.length).toBe(1);
      expect(wishlistService.wishlists[0].id).toBe('real-id');
    });
  });
});
