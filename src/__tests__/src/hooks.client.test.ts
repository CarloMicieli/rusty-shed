import { beforeEach, describe, expect, it, vi } from 'vitest';

const mockAttachConsole = vi.fn();
const mockLogError = vi.fn();

vi.mock('@tauri-apps/plugin-log', () => ({
  attachConsole: mockAttachConsole
}));

vi.mock('$lib/tauri-logger', () => ({
  log: {
    error: mockLogError
  }
}));

describe('hooks.client init', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    delete (window as Window & { __TAURI_INTERNALS__?: unknown }).__TAURI_INTERNALS__;
  });

  it('does not attach tauri console when tauri internals are absent', async () => {
    const addEventListenerSpy = vi.spyOn(window, 'addEventListener');
    const { init } = await import('../../hooks.client');

    await init();

    expect(mockAttachConsole).not.toHaveBeenCalled();
    expect(addEventListenerSpy).toHaveBeenCalledWith('unhandledrejection', expect.any(Function));
    expect(addEventListenerSpy).toHaveBeenCalledWith('error', expect.any(Function));
  });

  it('attaches tauri console when running in tauri', async () => {
    (window as Window & { __TAURI_INTERNALS__?: unknown }).__TAURI_INTERNALS__ = {};
    const { init } = await import('../../hooks.client');

    await init();

    expect(mockAttachConsole).toHaveBeenCalledOnce();
  });

  it('logs failures when attachConsole throws and still registers handlers', async () => {
    (window as Window & { __TAURI_INTERNALS__?: unknown }).__TAURI_INTERNALS__ = {};
    mockAttachConsole.mockRejectedValueOnce(new Error('attach failed'));
    const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
    const addEventListenerSpy = vi.spyOn(window, 'addEventListener');
    const { init } = await import('../../hooks.client');

    await init();

    expect(consoleErrorSpy).toHaveBeenCalled();
    expect(addEventListenerSpy).toHaveBeenCalledWith('unhandledrejection', expect.any(Function));
    expect(addEventListenerSpy).toHaveBeenCalledWith('error', expect.any(Function));
  });

  it('forwards unhandled rejection and uncaught error details to the logger', async () => {
    const handlers: Partial<Record<'unhandledrejection' | 'error', EventListener>> = {};
    vi.spyOn(window, 'addEventListener').mockImplementation(((
      type: string,
      listener: EventListenerOrEventListenerObject
    ) => {
      if (type === 'unhandledrejection' || type === 'error') {
        handlers[type] = listener as EventListener;
      }
    }) as typeof window.addEventListener);

    const { init } = await import('../../hooks.client');
    await init();

    handlers.unhandledrejection?.({ reason: 'network down' } as PromiseRejectionEvent);
    handlers.error?.({
      message: 'boom',
      filename: 'app.ts',
      lineno: 12,
      colno: 3
    } as ErrorEvent);

    expect(mockLogError).toHaveBeenCalledWith('Unhandled promise rejection: network down');
    expect(mockLogError).toHaveBeenCalledWith('Uncaught error: boom at app.ts:12:3');
  });
});
