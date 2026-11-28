import { beforeEach, describe, expect, it, vi } from 'vitest';

const svelteContext = vi.hoisted(() => ({
  setContext: vi.fn(),
  getContext: vi.fn()
}));

vi.mock('svelte', () => ({
  setContext: svelteContext.setContext,
  getContext: svelteContext.getContext
}));

import {
  AppService,
  getAppService,
  setAppService,
  setAppVersion
} from '$lib/shared/services/AppService.svelte';

describe('AppService', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('manages version and readiness state', () => {
    const service = new AppService();

    expect(service.version).toBe('');
    expect(service.isReady).toBe(false);

    service.setVersion('1.2.3');
    service.markAsReady();

    expect(service.version).toBe('1.2.3');
    expect(service.isReady).toBe(true);

    service.reset();
    expect(service.version).toBe('');
    expect(service.isReady).toBe(false);
  });

  it('setAppService stores and returns the provided instance', () => {
    const service = new AppService();
    const returned = setAppService(service);

    expect(returned).toBe(service);
    expect(svelteContext.setContext).toHaveBeenCalledOnce();
  });

  it('setAppService creates a new instance when omitted', () => {
    const returned = setAppService();

    expect(returned).toBeInstanceOf(AppService);
    expect(svelteContext.setContext).toHaveBeenCalledOnce();
  });

  it('getAppService returns instance from context', () => {
    const service = new AppService();
    svelteContext.getContext.mockReturnValue(service);

    expect(getAppService()).toBe(service);
  });

  it('getAppService throws when context is missing', () => {
    svelteContext.getContext.mockReturnValue(undefined);

    expect(() => getAppService()).toThrow('AppService not found in context');
  });

  it('setAppVersion logs deprecation warning', () => {
    const warn = vi.spyOn(console, 'warn').mockImplementation(() => undefined);

    setAppVersion('2.0.0');

    expect(warn).toHaveBeenCalledWith(
      'setAppVersion is deprecated. Use AppService.setVersion() instead.'
    );
  });
});
