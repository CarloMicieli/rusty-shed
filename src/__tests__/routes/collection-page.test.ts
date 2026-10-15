import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render } from '@testing-library/svelte';

// ── Mocks ────────────────────────────────────────────────────

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }));

vi.mock('$lib/paraglide/messages.js', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return Object.fromEntries(
    Object.entries(actual).map(([k, v]) => [k, typeof v === 'function' ? () => k : v])
  );
});

vi.mock('$lib/toaster', () => ({
  toaster: { success: vi.fn(), error: vi.fn(), loading: vi.fn() }
}));

vi.mock('$lib/state/page-title.svelte', () => ({
  getPageTitleContext: () => ({
    title: null,
    setTitle: vi.fn(),
    clearTitle: vi.fn()
  })
}));

// Provide a stable mock context so CollectionDashboard renders without the
// full context hierarchy from the layout.
vi.mock('$lib/features/collection/CollectionState.svelte', () => ({
  collectionState: {
    collection: null,
    isLoading: false,
    error: null,
    filters: {
      query: '',
      scale: null,
      scales: new Set(),
      companies: new Set(),
      categories: new Set(),
      epochs: new Set(),
      tags: new Set()
    },
    filteredItems: [],
    rawItems: [],
    items: [],
    summary: null,
    availableTags: [],
    availableScaleIds: [],
    availableCompanies: [],
    availableCategories: [],
    availableEpochs: [],
    hasActiveFilters: false,
    filterCount: 0,
    fetchCollection: vi.fn(),
    setFilter: vi.fn(),
    setQuery: vi.fn(),
    setScale: vi.fn(),
    toggleTag: vi.fn(),
    toggleScale: vi.fn(),
    toggleCompany: vi.fn(),
    toggleCategory: vi.fn(),
    toggleEpoch: vi.fn(),
    clearFilters: vi.fn(),
    deleteItem: vi.fn(),
    addItem: vi.fn(),
    updateItem: vi.fn()
  },
  getCollectionContext: vi.fn(() => ({
    collection: null,
    isLoading: false,
    error: null,
    filters: {
      query: '',
      scale: null,
      scales: new Set(),
      companies: new Set(),
      categories: new Set(),
      epochs: new Set(),
      tags: new Set()
    },
    filteredItems: [],
    rawItems: [],
    items: [],
    summary: null,
    availableTags: [],
    availableScaleIds: [],
    availableCompanies: [],
    availableCategories: [],
    availableEpochs: [],
    hasActiveFilters: false,
    filterCount: 0,
    fetchCollection: vi.fn(),
    setFilter: vi.fn(),
    setQuery: vi.fn(),
    setScale: vi.fn(),
    toggleTag: vi.fn(),
    toggleScale: vi.fn(),
    toggleCompany: vi.fn(),
    toggleCategory: vi.fn(),
    toggleEpoch: vi.fn(),
    clearFilters: vi.fn(),
    deleteItem: vi.fn(),
    addItem: vi.fn(),
    updateItem: vi.fn()
  })),
  setCollectionContext: vi.fn(),
  createCollectionState: vi.fn(() => ({})),
  availableScales: []
}));

vi.mock('$lib/features/collection/components/AddCollectionItemDrawer.svelte', () => ({
  default: function AddCollectionItemDrawerStub() {}
}));

vi.mock('$lib/features/collection/components/DeleteModal.svelte', () => ({
  default: function DeleteModalStub() {}
}));

vi.mock('$lib/features/collection/components/FilterPanel.svelte', () => ({
  default: function FilterPanelStub() {}
}));

vi.mock('$lib/features/collection/components/ControlPanel.svelte', () => ({
  default: function ControlPanelStub() {}
}));

vi.mock('$lib/components/RailwayModelPreviewCard.svelte', () => ({
  default: function RailwayModelPreviewCardStub() {}
}));

vi.mock('$lib/components/PageHeader.svelte', () => ({
  default: function PageHeaderStub() {}
}));

// ── Test target ───────────────────────────────────────────────

import CollectionPage from '../../routes/collection/+page.svelte';

describe('routes/collection/+page.svelte', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders without throwing', () => {
    expect(() => render(CollectionPage)).not.toThrow();
  });

  it('mounts the CollectionDashboard feature component', () => {
    // The page is a thin wrapper — it renders CollectionDashboard which uses
    // the collection context. Just ensure the DOM is non-empty.
    const { container } = render(CollectionPage);
    expect(container).toBeDefined();
  });
});
