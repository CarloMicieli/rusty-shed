import { vi, beforeEach, afterEach } from 'vitest';
import '@testing-library/jest-dom/vitest';

// Mock Tauri globals
globalThis.__TAURI_INTERNALS__ = {
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
