import { vi } from 'vitest';
import { writable, type Writable } from 'svelte/store';
import type { Page } from '@sveltejs/kit';

// Mock $app/navigation
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

// Mock $app/paths
export const base = '';
export const assets = '';
export const resolve = vi.fn((path: string) => path);

// Mock $app/stores
export const page: Writable<Page> = writable({
  url: new URL('http://localhost/'),
  params: {},
  route: { id: null },
  status: 200,
  error: null,
  data: {},
  state: {},
  form: undefined
} as unknown as Page);

export const navigating: Writable<any> = writable(null);
export const updated: Writable<boolean> = writable(false);
