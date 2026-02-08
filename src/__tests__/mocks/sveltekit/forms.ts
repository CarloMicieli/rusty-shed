/**
 * Mock for $app/forms
 * Used by Superforms and other SvelteKit-dependent components in tests
 */
import { vi } from 'vitest';

// eslint-disable-next-line @typescript-eslint/no-explicit-any
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

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const enhance = vi.fn((form: HTMLFormElement, _options?: any) => {
  console.log(`[Mock] enhance form:`, form.id || 'unnamed');
  return {
    destroy: vi.fn()
  };
});
