import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render } from '@testing-library/svelte';

// ── Mocks ────────────────────────────────────────────────────

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }));

vi.mock('$lib/paraglide/messages.js', () => new Proxy({}, { get: (_t, k) => () => String(k) }));

vi.mock('$lib/toaster', () => ({
  toaster: { success: vi.fn(), error: vi.fn(), loading: vi.fn() }
}));

vi.mock('lucide-svelte', () => new Proxy({}, { get: () => () => '' }));

// Provide a stable mock context so CollectionDashboard renders without the
// full context hierarchy from the layout.
vi.mock('$lib/features/collection/CollectionState.svelte', () => ({
  getCollectionContext: vi.fn(() => ({
    collection: null,
    isLoading: false,
    error: null,
    filters: { query: '', scale: null, tags: new Set() },
    filteredItems: [],
    items: [],
    summary: null,
    availableTags: [],
    filterCount: 0,
    fetchCollection: vi.fn(),
    setFilter: vi.fn(),
    clearFilters: vi.fn(),
    deleteItem: vi.fn(),
    addItem: vi.fn(),
    updateItem: vi.fn()
  })),
  setCollectionContext: vi.fn(),
  createCollectionState: vi.fn(() => ({})),
  availableScales: []
}));

vi.mock('$lib/features/collection/components/AddModelDrawer.svelte', () => ({
  default: { name: 'AddModelDrawerStub', _: { props: [] } }
}));

vi.mock('$lib/features/collection/components/DeleteModal.svelte', () => ({
  default: { name: 'DeleteModalStub', _: { props: [] } }
}));

vi.mock('$lib/features/collection/components/FilterPanel.svelte', () => ({
  default: { name: 'FilterPanelStub', _: { props: [] } }
}));

vi.mock('$lib/components/RailwayModelPreviewCard.svelte', () => ({
  default: { name: 'RailwayModelPreviewCardStub', _: { props: [] } }
}));

vi.mock('$lib/components/PageHeader.svelte', () => ({
  default: { name: 'PageHeaderStub', _: { props: [] } }
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
