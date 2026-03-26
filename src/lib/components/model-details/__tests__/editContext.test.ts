import { describe, it, expect, beforeEach } from 'vitest';
import { getContext, setContext } from 'svelte';
import { vi } from 'vitest';

// Mock Svelte context API so we can control it in unit tests
const contextStore = new Map<symbol | string, unknown>();

vi.mock('svelte', async (importOriginal) => {
  const actual = await importOriginal<typeof import('svelte')>();
  return {
    ...actual,
    setContext: vi.fn((key: symbol, value: unknown) => {
      contextStore.set(key, value);
    }),
    getContext: vi.fn((key: symbol) => {
      return contextStore.get(key);
    })
  };
});

import { setEditContext, getEditContext } from '../editContext.svelte';

describe('editContext', () => {
  beforeEach(() => {
    contextStore.clear();
    vi.clearAllMocks();
  });

  describe('setEditContext', () => {
    it('returns a context object with null activeEditId', () => {
      const ctx = setEditContext();
      expect(ctx.activeEditId).toBeNull();
    });

    it('calls setContext with the context instance', () => {
      const ctx = setEditContext();
      expect(setContext).toHaveBeenCalledOnce();
      // The context stored should be the same object
      const [, stored] = (setContext as ReturnType<typeof vi.fn>).mock.calls[0] as [
        symbol,
        unknown
      ];
      expect(stored).toBe(ctx);
    });
  });

  describe('getEditContext', () => {
    it('returns the context previously set by setEditContext', () => {
      const ctx = setEditContext();
      const retrieved = getEditContext();
      expect(retrieved).toBe(ctx);
    });

    it('returns a fallback context when no provider is present', () => {
      // getContext returns undefined when no provider
      (getContext as ReturnType<typeof vi.fn>).mockReturnValueOnce(undefined);
      const ctx = getEditContext();
      expect(ctx).toBeDefined();
      expect(ctx.activeEditId).toBeNull();
    });
  });

  describe('RollingStockEditContext state', () => {
    it('setActive sets the activeEditId', () => {
      const ctx = setEditContext();
      ctx.setActive('rs-123');
      expect(ctx.activeEditId).toBe('rs-123');
    });

    it('clearActive resets activeEditId to null', () => {
      const ctx = setEditContext();
      ctx.setActive('rs-456');
      ctx.clearActive();
      expect(ctx.activeEditId).toBeNull();
    });

    it('setActive replaces a previously active id', () => {
      const ctx = setEditContext();
      ctx.setActive('rs-1');
      ctx.setActive('rs-2');
      expect(ctx.activeEditId).toBe('rs-2');
    });
  });
});
