import { beforeEach, describe, expect, it, vi } from 'vitest';

const { contextStore, svelteMocks, safeInvokeMock } = vi.hoisted(() => {
  const store = new Map<symbol, unknown>();
  return {
    contextStore: store,
    svelteMocks: {
      setContext: vi.fn((key: symbol, value: unknown) => {
        store.set(key, value);
      }),
      getContext: vi.fn((key: symbol) => store.get(key))
    },
    safeInvokeMock: vi.fn()
  };
});

vi.mock('svelte', () => svelteMocks);
vi.mock('$lib/shared/services/TauriAdapter', () => ({
  safeInvoke: safeInvokeMock
}));

import {
  getImageService,
  ImageService,
  setImageService
} from '$lib/shared/services/ImageService.svelte';

describe('ImageService', () => {
  beforeEach(() => {
    contextStore.clear();
    vi.clearAllMocks();
  });

  it('resolves and caches backend image paths', async () => {
    safeInvokeMock.mockResolvedValueOnce({ ok: true, data: '/img/model-1.png' });
    const service = new ImageService();

    const first = await service.resolveImagePath('model-1', 'railway_model');
    const second = await service.resolveImagePath('model-1', 'railway_model');

    expect(first).toBe('/img/model-1.png');
    expect(second).toBe('/img/model-1.png');
    expect(service.getCacheSize()).toBe(1);
    expect(safeInvokeMock).toHaveBeenCalledTimes(1);
  });

  it('returns null when backend lookup fails and does not cache', async () => {
    safeInvokeMock.mockResolvedValueOnce({
      ok: false,
      error: { kind: 'not_found', message: 'Missing image' }
    });
    const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
    const service = new ImageService();

    const result = await service.resolveImagePath('missing', 'manufacturer');

    expect(result).toBeNull();
    expect(service.getCacheSize()).toBe(0);
    expect(consoleSpy).toHaveBeenCalled();
    consoleSpy.mockRestore();
  });

  it('clears cache and supports context helpers', async () => {
    safeInvokeMock.mockResolvedValueOnce({ ok: true, data: '/img/model-2.png' });
    const created = setImageService();

    await created.resolveImagePath('model-2', 'railway_model');
    expect(created.getCacheSize()).toBe(1);

    created.clearCache();
    expect(created.getCacheSize()).toBe(0);
    expect(getImageService()).toBe(created);
  });

  it('registers injected instance and throws when context is missing', () => {
    const injected = new ImageService();
    setImageService(injected);
    expect(getImageService()).toBe(injected);

    contextStore.clear();
    expect(() => getImageService()).toThrow('ImageService not found in context');
  });
});
