import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

describe('tauri logger', () => {
  beforeEach(() => {
    vi.resetModules();
    vi.clearAllMocks();
    delete (window as Window & { __TAURI_INTERNALS__?: unknown }).__TAURI_INTERNALS__;
  });

  afterEach(() => {
    delete (window as Window & { __TAURI_INTERNALS__?: unknown }).__TAURI_INTERNALS__;
  });

  it('falls back to browser console when Tauri logger is unavailable', async () => {
    const infoSpy = vi.spyOn(console, 'info').mockImplementation(() => {});
    const warnSpy = vi.spyOn(console, 'warn').mockImplementation(() => {});
    const debugSpy = vi.spyOn(console, 'debug').mockImplementation(() => {});

    const { log } = await import('$lib/tauri-logger');
    log.info('hello', { feature: 'fallback' });
    log.warn('attention');
    log.trace('trace-line');

    expect(infoSpy).toHaveBeenCalledWith('hello', { feature: 'fallback' });
    expect(warnSpy).toHaveBeenCalledWith('attention');
    expect(debugSpy).toHaveBeenCalledWith('trace-line');
  });
});
