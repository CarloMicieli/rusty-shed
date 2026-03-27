/**
 * Mock for $app/forms
 * Used by Superforms and other SvelteKit-dependent components in tests
 */
import { vi } from 'vitest';

export const applyAction = vi.fn(async (result: any) => {
  console.log(`[Mock] applyAction:`, result);
});

export const deserialize = vi.fn((data: string) => {
  try {
    return JSON.parse(data);
  } catch {
    return { type: 'success' as const, status: 200, data: {} };
  }
});

export const enhance = vi.fn((_form: HTMLFormElement, _options?: any) => {
  return {
    destroy: vi.fn()
  };
});
