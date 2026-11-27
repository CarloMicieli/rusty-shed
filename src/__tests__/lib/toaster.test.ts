import { describe, expect, it, vi, beforeEach } from 'vitest';

const toastMocks = vi.hoisted(() => ({
  success: vi.fn().mockReturnValue('success-id'),
  error: vi.fn().mockReturnValue('error-id'),
  info: vi.fn().mockReturnValue('info-id'),
  warning: vi.fn().mockReturnValue('warning-id'),
  loading: vi.fn().mockReturnValue('loading-id'),
  promise: vi.fn(),
  custom: vi.fn(),
  message: vi.fn(),
  dismiss: vi.fn(),
  base: vi.fn().mockReturnValue('signal-id')
}));

vi.mock('svelte-sonner', () => ({
  toast: Object.assign(toastMocks.base, {
    success: toastMocks.success,
    error: toastMocks.error,
    info: toastMocks.info,
    warning: toastMocks.warning,
    loading: toastMocks.loading,
    promise: toastMocks.promise,
    custom: toastMocks.custom,
    message: toastMocks.message,
    dismiss: toastMocks.dismiss
  })
}));

import { toaster } from '$lib/toaster';

describe('toaster', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('normalizes string and object messages for standard toast methods', () => {
    toaster.success('ok');
    toaster.error({ title: 'err', duration: 1234 });
    toaster.info({ duration: 2000 });
    toaster.warning('warn', { id: 'w1' });
    toaster.loading({ title: 'loading', id: 'l1' });

    expect(toastMocks.success).toHaveBeenCalledWith('ok', undefined);
    expect(toastMocks.error).toHaveBeenCalledWith('err', { duration: 1234 });
    expect(toastMocks.info).toHaveBeenCalledWith('', { duration: 2000 });
    expect(toastMocks.warning).toHaveBeenCalledWith('warn', { id: 'w1' });
    expect(toastMocks.loading).toHaveBeenCalledWith('loading', { id: 'l1' });
  });

  it('emits a signal toast with toast-signal class', () => {
    const id = toaster.signal('Background sync failed', { description: 'Retrying' });

    expect(id).toBe('signal-id');
    expect(toastMocks.base).toHaveBeenCalledWith('Background sync failed', {
      description: 'Retrying',
      class: 'toast-signal'
    });
  });

  it('exposes passthrough helpers', () => {
    const p = Promise.resolve('ok');
    toaster.promise(p, { loading: 'loading' });
    toaster.custom(() => 'x');
    toaster.message('hi');
    toaster.dismiss('id-1');

    expect(toastMocks.promise).toHaveBeenCalled();
    expect(toastMocks.custom).toHaveBeenCalled();
    expect(toastMocks.message).toHaveBeenCalledWith('hi');
    expect(toastMocks.dismiss).toHaveBeenCalledWith('id-1');
  });
});
