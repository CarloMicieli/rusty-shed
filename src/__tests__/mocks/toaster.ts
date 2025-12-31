import { vi } from 'vitest';

// Mock toaster instance matching the structure from @skeletonlabs/skeleton-svelte
export const toaster = {
  toast: vi.fn(
    (options: {
      id?: string;
      message?: string;
      variant?: 'info' | 'success' | 'warning' | 'error';
      duration?: number;
      action?: { label: string; onClick: () => void };
    }) => {
      console.log(`[Mock Toast] ${options.variant}: ${options.message}`);
    }
  ),
  close: vi.fn((id: string) => {
    console.log(`[Mock Toast] Closed: ${id}`);
  }),
  clear: vi.fn(() => {
    console.log(`[Mock Toast] Cleared all`);
  })
};

// Helper to reset toast mocks between tests
export function resetToasterMocks() {
  toaster.toast.mockClear();
  toaster.close.mockClear();
  toaster.clear.mockClear();
}
