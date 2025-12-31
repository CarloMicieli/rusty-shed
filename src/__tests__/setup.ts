import { vi, beforeEach, afterEach } from 'vitest';
import '@testing-library/jest-dom/vitest';

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

// Reset all mocks between tests
beforeEach(() => {
  vi.clearAllMocks();
});

afterEach(() => {
  vi.restoreAllMocks();
});
