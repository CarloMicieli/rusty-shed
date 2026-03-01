import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import { flushSync } from 'svelte';

// ── Mocks ────────────────────────────────────────────────────

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }));

vi.mock('$lib/paraglide/messages.js', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return Object.fromEntries(
    Object.entries(actual).map(([k, v]) => [k, typeof v === 'function' ? () => k : v])
  );
});

vi.mock('$lib/paraglide/runtime.js', () => ({
  getLocale: vi.fn(() => 'en')
}));

// Create a mutable fake service so individual tests can control its state
const fakeService = vi.hoisted(() => ({
  results: [] as {
    itemId: string;
    source: string;
    displayName: string;
    manufacturerName: string;
    railwayModelId: string;
    parentId: null;
  }[],
  isLoading: false,
  error: null as string | null,
  search: vi.fn().mockResolvedValue(undefined),
  reset: vi.fn()
}));

vi.mock('$lib/features/search', () => ({
  setSearchContext: vi.fn(() => fakeService),
  getSearchContext: vi.fn(() => fakeService)
}));

vi.mock('$lib/features/search/components/SearchResultCard.svelte', () => ({
  default: function SearchResultCardStub() {}
}));
vi.mock('$lib/features/search/components/SearchEmptyState.svelte', () => ({
  default: function SearchEmptyStateStub() {}
}));

// ── $app/stores: use the alias mock directly (vitest.config resolves $app →
// src/__tests__/mocks/sveltekit which exports writable page/navigating stores)
import { page as mockPageStoreRaw } from '$app/stores';

// Cast to any to bypass strict SvelteKit types in tests

const mockPageStore = mockPageStoreRaw as any;

// ── Helpers ───────────────────────────────────────────────────

function setSearchQuery(query: string) {
  mockPageStore.set({
    url: new URL(`http://localhost/search?q=${encodeURIComponent(query)}`),
    params: {},
    route: { id: '/search' },
    status: 200,
    error: null,
    data: {},
    state: {},
    form: undefined
  });
}

function clearSearchQuery() {
  mockPageStore.set({
    url: new URL('http://localhost/search'),
    params: {},
    route: { id: '/search' },
    status: 200,
    error: null,
    data: {},
    state: {},
    form: undefined
  });
}

// ── Test target ───────────────────────────────────────────────

import SearchPage from '../../routes/search/+page.svelte';

describe('routes/search/+page.svelte', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    fakeService.results = [];
    fakeService.isLoading = false;
    fakeService.error = null;
    clearSearchQuery();
  });

  it('renders without throwing', () => {
    expect(() => render(SearchPage)).not.toThrow();
  });

  it('shows the default heading when there is no query', () => {
    render(SearchPage);
    expect(screen.getByText('search_page_title')).toBeInTheDocument();
  });

  it('shows the hint text when there is no query', () => {
    render(SearchPage);
    expect(screen.getByText('search_min_length_hint')).toBeInTheDocument();
  });

  it('shows results-for heading when query is present', () => {
    setSearchQuery('steam');
    const { container } = render(SearchPage);
    flushSync();
    expect(container.textContent).toContain('search_results_for');
  });

  it('shows a loading spinner when isLoading is true', () => {
    fakeService.isLoading = true;
    setSearchQuery('loco');
    const { container } = render(SearchPage);
    flushSync();
    const spinner = container.querySelector('.animate-spin');
    expect(spinner).not.toBeNull();
  });

  it('shows error text when service has an error', () => {
    fakeService.error = 'Search backend unavailable';
    setSearchQuery('test');
    render(SearchPage);
    flushSync();
    expect(screen.getByText('Search backend unavailable')).toBeInTheDocument();
  });

  it('calls service.search after rendering with a long-enough query', () => {
    setSearchQuery('BR');
    render(SearchPage);
    // flushSync drains the Svelte 5 $effect queue synchronously
    flushSync();
    expect(fakeService.search).toHaveBeenCalledWith('BR', 'en');
  });

  it('calls service.reset after rendering with no query', () => {
    clearSearchQuery();
    render(SearchPage);
    flushSync();
    expect(fakeService.reset).toHaveBeenCalled();
  });

  it('calls service.reset after rendering with a short query', () => {
    setSearchQuery('a');
    render(SearchPage);
    flushSync();
    expect(fakeService.reset).toHaveBeenCalled();
  });
});
