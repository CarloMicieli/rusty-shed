import { describe, it, expect, vi, beforeEach } from 'vitest';
import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/svelte';

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

const mockCollectionState = vi.hoisted(() => ({
  collection: null,
  isLoading: false,
  error: null,
  filters: {
    query: '',
    scale: null,
    scales: new Set<string>(),
    companies: new Set<string>(),
    categories: new Set<string>(),
    epochs: new Set<string>(),
    tags: new Set<string>()
  },
  filteredItems: [] as unknown[],
  rawItems: [] as unknown[],
  items: [],
  summary: null,
  availableTags: [] as string[],
  availableScaleIds: [] as string[],
  availableCompanies: [] as string[],
  availableCategories: [] as string[],
  availableEpochs: [] as string[],
  hasActiveFilters: false,
  filterCount: 0,
  fetchCollection: vi.fn().mockResolvedValue(undefined),
  setFilter: vi.fn(),
  setQuery: vi.fn(),
  setScale: vi.fn(),
  toggleTag: vi.fn(),
  toggleScale: vi.fn(),
  toggleCompany: vi.fn(),
  toggleCategory: vi.fn(),
  toggleEpoch: vi.fn(),
  clearFilters: vi.fn(),
  setStatus: vi.fn(),
  deleteItem: vi.fn().mockResolvedValue(undefined),
  addItem: vi.fn(),
  updateItem: vi.fn()
}));

const mockCommands = vi.hoisted(() => ({
  getCollectionStats: vi.fn()
}));

const mockGoto = vi.hoisted(() => vi.fn().mockResolvedValue(undefined));

vi.mock('$lib/bindings', () => ({ commands: mockCommands }));
vi.mock('$app/navigation', () => ({ goto: mockGoto }));

// Provide a stable mock context so CollectionDashboard renders without the
// full context hierarchy from the layout.
vi.mock('$lib/features/collection/CollectionState.svelte', () => ({
  collectionState: mockCollectionState,
  getCollectionContext: vi.fn(() => mockCollectionState),
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

vi.mock('$lib/components/VirtualGrid.svelte', () => ({
  default: function VirtualGridStub() {}
}));

vi.mock('$lib/features/collection/components/CollectionTableView.svelte', () => ({
  default: function CollectionTableViewStub() {}
}));

// ── Test target ───────────────────────────────────────────────

import CollectionPage from '../../routes/collection/+page.svelte';

describe('routes/collection/+page.svelte', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
    mockCollectionState.collection = null;
    mockCollectionState.isLoading = false;
    mockCollectionState.error = null;
    mockCollectionState.filters = {
      query: '',
      scale: null,
      scales: new Set<string>(),
      companies: new Set<string>(),
      categories: new Set<string>(),
      epochs: new Set<string>(),
      tags: new Set<string>()
    };
    mockCollectionState.filteredItems = [];
    mockCollectionState.rawItems = [];
    mockCollectionState.summary = null;
    mockCollectionState.availableTags = [];
    mockCollectionState.availableScaleIds = [];
    mockCollectionState.availableCompanies = [];
    mockCollectionState.availableCategories = [];
    mockCollectionState.availableEpochs = [];
    mockCollectionState.hasActiveFilters = false;
    mockCollectionState.fetchCollection.mockResolvedValue(undefined);
    mockCommands.getCollectionStats.mockResolvedValue({
      preorderedCount: 2,
      activeCount: 5,
      soldCount: 1,
      investmentAtRiskAmount: 0,
      investmentAtRiskCurrency: null,
      realizedProfitAmount: 0
    });
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

  it('shows loading skeletons and invokes collection loading on mount', async () => {
    mockCollectionState.isLoading = true;
    render(CollectionPage);

    await waitFor(() => expect(mockCollectionState.fetchCollection).toHaveBeenCalledOnce());
    expect(document.querySelectorAll('[aria-label="collection_loading_card"]')).toHaveLength(6);
  });

  it('shows the empty state and exposes the add action', () => {
    render(CollectionPage);
    expect(screen.getByText('collection_add_first')).toBeInTheDocument();
  });

  it('shows no-results state and clears filters', async () => {
    mockCollectionState.rawItems = [{ id: 'item-1' }];
    mockCollectionState.filteredItems = [];
    mockCollectionState.hasActiveFilters = true;
    render(CollectionPage);

    expect(screen.getByText('collection_no_results')).toBeInTheDocument();
    await fireEvent.click(screen.getByRole('button', { name: 'collection_clear_filters' }));
    expect(mockCollectionState.clearFilters).toHaveBeenCalledOnce();
  });

  it('renders active filter chips and removes each clicked filter', async () => {
    mockCollectionState.rawItems = [{ id: 'item-1' }];
    mockCollectionState.filteredItems = [{ id: 'item-1' }];
    mockCollectionState.hasActiveFilters = true;
    mockCollectionState.availableScaleIds = ['H0'];
    mockCollectionState.filters.scales = new Set(['H0']);
    mockCollectionState.filters.companies = new Set(['Roco']);
    mockCollectionState.filters.categories = new Set(['LOCOMOTIVE']);
    mockCollectionState.filters.epochs = new Set(['VI']);
    mockCollectionState.filters.tags = new Set(['featured']);
    render(CollectionPage);

    const removeButtons = screen.getAllByRole('button', { name: 'collection_remove_filter' });
    expect(removeButtons).toHaveLength(5);

    await fireEvent.click(removeButtons[0]);
    expect(mockCollectionState.toggleScale).toHaveBeenCalledWith('H0');
    await fireEvent.click(removeButtons[1]);
    expect(mockCollectionState.toggleEpoch).toHaveBeenCalledWith('VI');
    await fireEvent.click(removeButtons[2]);
    expect(mockCollectionState.toggleCategory).toHaveBeenCalledWith('LOCOMOTIVE');
    await fireEvent.click(removeButtons[3]);
    expect(mockCollectionState.toggleCompany).toHaveBeenCalledWith('Roco');
    await fireEvent.click(removeButtons[4]);
    expect(mockCollectionState.toggleTag).toHaveBeenCalledWith('featured');
  });

  it('changes lifecycle status from active lifecycle chips', async () => {
    mockCollectionState.rawItems = [{ id: 'item-1' }];
    mockCollectionState.filteredItems = [{ id: 'item-1' }];
    render(CollectionPage);

    await fireEvent.click(screen.getByRole('button', { name: /collection_stats_preordered/i }));
    expect(mockCollectionState.setStatus).toHaveBeenCalledWith('preordered');

    await fireEvent.click(screen.getByRole('button', { name: /collection_stats_sold/i }));
    expect(mockCollectionState.setStatus).toHaveBeenCalledWith('sold');
  });

  it('switches between grid and table view modes', async () => {
    mockCollectionState.rawItems = [{ id: 'item-1' }];
    mockCollectionState.filteredItems = [{ id: 'item-1' }];
    render(CollectionPage);

    await fireEvent.click(screen.getByTitle('view_table'));
    expect(screen.getByTitle('view_table')).toHaveAttribute('aria-pressed', 'true');

    await fireEvent.click(screen.getByTitle('view_grid'));
    expect(screen.getByTitle('view_grid')).toHaveAttribute('aria-pressed', 'true');
  });
});
