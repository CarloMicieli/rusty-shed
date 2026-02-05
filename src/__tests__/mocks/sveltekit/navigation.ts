import { vi } from 'vitest';

export const goto = vi.fn(async (url: string) => {
  console.log(`[Mock] goto: ${url}`);
});

export const invalidate = vi.fn(async (url: string) => {
  console.log(`[Mock] invalidate: ${url}`);
});

export const invalidateAll = vi.fn(async () => {
  console.log(`[Mock] invalidateAll`);
});

export const preloadData = vi.fn(async (url: string) => {
  console.log(`[Mock] preloadData: ${url}`);
  return { type: 'loaded' as const, status: 200, data: {} };
});

export const preloadCode = vi.fn(async (url: string) => {
  console.log(`[Mock] preloadCode: ${url}`);
});

export const beforeNavigate = vi.fn(() => {});
export const afterNavigate = vi.fn(() => {});
export const onNavigate = vi.fn(() => {});
export const disableScrollHandling = vi.fn(() => {});
