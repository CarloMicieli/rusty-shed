import { writable, type Writable } from 'svelte/store';
import type { Page } from '@sveltejs/kit';

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

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const navigating: Writable<any> = writable(null);
export const updated: Writable<boolean> = writable(false);
