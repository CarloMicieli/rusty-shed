import type { Page } from '@sveltejs/kit';

export const page: Page = {
  url: new URL('http://localhost/'),
  params: {},
  route: { id: null },
  status: 200,
  error: null,
  data: {},
  state: {},
  form: undefined
} as unknown as Page;

export const navigating = null;
export const updated = false;
