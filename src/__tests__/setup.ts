import { vi, beforeEach, afterEach } from 'vitest';
import '@testing-library/jest-dom/vitest';

// Ensure BROWSER is set so bits-ui knows we're in a browser context
const globalWithBrowser = globalThis as typeof globalThis & { BROWSER?: boolean };
globalWithBrowser.BROWSER = true;

type TauriInternals = {
  invoke: ReturnType<typeof vi.fn>;
  convertFileSrc: ReturnType<typeof vi.fn>;
  transformCallback: ReturnType<typeof vi.fn>;
  metadata: {
    windows: unknown[];
    webviews: unknown[];
    currentWindow: { label: string };
    currentWebview: { label: string };
  };
};

declare global {
  var __TAURI_INTERNALS__: TauriInternals;
}

const globalWithTauri = globalThis as typeof globalThis & { __TAURI_INTERNALS__: TauriInternals };

// Mock Tauri globals
globalWithTauri.__TAURI_INTERNALS__ = {
  invoke: vi.fn(),
  convertFileSrc: vi.fn((filePath: string) => `asset://localhost/${filePath}`),
  transformCallback: vi.fn(),
  metadata: {
    windows: [],
    webviews: [],
    currentWindow: { label: 'main' },
    currentWebview: { label: 'main' }
  }
};

// happy-dom doesn't implement the Web Animations API used by Svelte transitions.
// Provide a minimal stub so `element.animate()` doesn't crash and transitions
// complete immediately (required for loading-state tests).
if (!Element.prototype.animate) {
  Element.prototype.animate = function () {
    const listeners: Record<string, EventListenerOrEventListenerObject[]> = {};
    const animation = {
      play: vi.fn(),
      pause: vi.fn(),
      cancel: vi.fn(),
      finish: vi.fn(),
      onfinish: null as (() => void) | null,
      oncancel: null as (() => void) | null,
      finished: Promise.resolve({} as Animation),
      addEventListener(type: string, listener: EventListenerOrEventListenerObject) {
        (listeners[type] ??= []).push(listener);
      },
      removeEventListener(type: string, listener: EventListenerOrEventListenerObject) {
        listeners[type] = (listeners[type] ?? []).filter((l) => l !== listener);
      },
      dispatchEvent: vi.fn()
    };
    // Fire 'finish' asynchronously so Svelte's out-transitions clean up the DOM.
    Promise.resolve().then(() => {
      if (animation.onfinish) animation.onfinish();
      (listeners['finish'] ?? []).forEach((l) =>
        typeof l === 'function' ? l({} as Event) : l.handleEvent({} as Event)
      );
    });
    return animation as unknown as Animation;
  };
}

// Reset all mocks between tests
beforeEach(() => {
  // Use fake timers so vi.clearAllTimers() can cancel bits-ui's deferred DOM
  // cleanup (body-scroll-lock uses setTimeout). shouldAdvanceTime keeps the
  // fake clock in sync with wall-clock time so userEvent/waitFor still work.
  vi.useFakeTimers({ shouldAdvanceTime: true });
  vi.clearAllMocks();
});

afterEach(() => {
  // Clear any pending timers (bits-ui cleanup) — works because fake timers are active
  vi.clearAllTimers();
  vi.useRealTimers();
  vi.restoreAllMocks();

  // Clean up any document/body styles set by bits-ui
  if (typeof document !== 'undefined' && document.body) {
    document.body.removeAttribute('style');
    document.body.style.removeProperty('--scrollbar-width');
  }
});
