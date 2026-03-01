import { vi } from 'vitest';

// Mock toaster instance matching the svelte-sonner-based API from $lib/toaster
export const toaster = {
  success: vi.fn(),
  error: vi.fn(),
  info: vi.fn(),
  warning: vi.fn(),
  loading: vi.fn(),
  promise: vi.fn(),
  custom: vi.fn(),
  message: vi.fn(),
  dismiss: vi.fn()
};

// Helper to reset toast mocks between tests
export function resetToasterMocks() {
  toaster.success.mockClear();
  toaster.error.mockClear();
  toaster.info.mockClear();
  toaster.warning.mockClear();
  toaster.loading.mockClear();
  toaster.promise.mockClear();
  toaster.custom.mockClear();
  toaster.message.mockClear();
  toaster.dismiss.mockClear();
}
