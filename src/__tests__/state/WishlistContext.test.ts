import { beforeEach, describe, expect, it, vi } from 'vitest';

const { contextStore, svelteMocks } = vi.hoisted(() => {
  const store = new Map<symbol, unknown>();
  return {
    contextStore: store,
    svelteMocks: {
      setContext: vi.fn((key: symbol, value: unknown) => {
        store.set(key, value);
      }),
      getContext: vi.fn((key: symbol) => store.get(key))
    }
  };
});

vi.mock('svelte', () => svelteMocks);
vi.mock('$lib/toaster', () => ({
  toaster: {
    loading: vi.fn(),
    success: vi.fn(),
    error: vi.fn()
  }
}));
vi.mock('$lib/paraglide/messages.js', () => ({
  collection_toast_loading: () => 'Loading...',
  collection_toast_success: () => 'Success',
  collection_toast_error: () => 'Error',
  collection_toast_retry: () => 'Retry',
  purchase_dialog_success: () => 'Purchase recorded'
}));

import {
  createWishlistState,
  getWishlistContext,
  setWishlistContext
} from '$lib/features/wishlists/WishlistState.svelte';

describe('Wishlist context helpers', () => {
  beforeEach(() => {
    contextStore.clear();
    vi.clearAllMocks();
  });

  it('registers and retrieves wishlist state from context', () => {
    const state = createWishlistState();

    setWishlistContext(state);

    expect(getWishlistContext()).toBe(state);
  });

  it('throws when context is requested before initialization', () => {
    expect(() => getWishlistContext()).toThrow('WishlistContext not provided');
  });
});
