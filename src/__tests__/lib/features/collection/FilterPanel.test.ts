import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent, cleanup } from '@testing-library/svelte';
import FilterPanel from '$lib/features/collection/components/FilterPanel.svelte';
import type { FilterState } from '$lib/features/collection/CollectionState.svelte';
import { SvelteSet } from 'svelte/reactivity';

// ── Paraglide messages ───────────────────────────────────────────────────────
vi.mock('$lib/paraglide/messages.js', () => ({
  collection_filters_title: () => 'Filters',
  collection_search_placeholder: () => 'Search your collection...',
  collection_search_hint: () =>
    'Search manufacturer, product code, or description while keeping your filters applied.',
  collection_filter_scales: () => 'Scales',
  collection_filter_tags: () => 'Tags',
  collection_clear_filters: () => 'Clear all filters',
  filter_panel_close_title: () => 'Close filters',
  collection_filter_all: () => 'All'
}));

// ── Tags config ──────────────────────────────────────────────────────────────
vi.mock('$lib/config/tags', () => ({
  resolveTagMeta: (_tag: string) => ({ label: () => _tag }),
  tagIcon: (_tag: string) => null,
  FIXED_TAG_META: {},
  sortAvailableTags: (tags: string[]) => tags
}));

// ── Shared UI components ─────────────────────────────────────────────────────
vi.mock('$lib/components', async (importOriginal) => {
  const actual = await importOriginal<typeof import('$lib/components')>();
  return {
    ...actual
  };
});

function makeFilters(overrides?: Partial<FilterState>): FilterState {
  return {
    query: '',
    scale: null,
    scales: new SvelteSet<string>(),
    companies: new SvelteSet<string>(),
    categories: new SvelteSet<string>(),
    epochs: new SvelteSet<string>(),
    tags: new SvelteSet<string>(),
    status: 'active',
    ...overrides
  };
}

describe('FilterPanel.svelte', () => {
  const scales = [
    { id: 'HO', display: 'HO (1:87)' },
    { id: 'N', display: 'N (1:160)' }
  ];

  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
  });

  it('renders the filters title', () => {
    render(FilterPanel, {
      props: {
        filters: makeFilters(),
        availableTags: [],
        availableScales: scales
      }
    });
    expect(screen.getByText('Filters')).toBeInTheDocument();
  });

  it('renders the clear-all-filters button', () => {
    render(FilterPanel, {
      props: {
        filters: makeFilters(),
        availableTags: [],
        availableScales: scales
      }
    });
    expect(screen.getByText('Clear all filters')).toBeInTheDocument();
  });

  it('calls onClear when clear button is clicked', async () => {
    const onClear = vi.fn();
    render(FilterPanel, {
      props: {
        filters: makeFilters(),
        availableTags: [],
        availableScales: scales,
        onClear
      }
    });
    await fireEvent.click(screen.getByText('Clear all filters'));
    expect(onClear).toHaveBeenCalledTimes(1);
  });

  it('renders scale badges from availableScales', () => {
    render(FilterPanel, {
      props: {
        filters: makeFilters(),
        availableTags: [],
        availableScales: scales
      }
    });
    expect(screen.getByText('HO (1:87)')).toBeInTheDocument();
    expect(screen.getByText('N (1:160)')).toBeInTheDocument();
  });

  it('calls onSetScale with the selected scale when a scale badge is clicked', async () => {
    const onSetScale = vi.fn();
    render(FilterPanel, {
      props: {
        filters: makeFilters(),
        availableTags: [],
        availableScales: scales,
        onSetScale
      }
    });
    await fireEvent.click(screen.getByText('HO (1:87)'));
    expect(onSetScale).toHaveBeenCalledWith('HO');
  });

  it('calls onSetScale with null when "All" scale badge is clicked', async () => {
    const onSetScale = vi.fn();
    render(FilterPanel, {
      props: {
        filters: makeFilters({ scale: 'HO' }),
        availableTags: [],
        availableScales: scales,
        onSetScale
      }
    });
    await fireEvent.click(screen.getByText('All'));
    expect(onSetScale).toHaveBeenCalledWith(null);
  });

  it('renders tag badges from availableTags', () => {
    render(FilterPanel, {
      props: {
        filters: makeFilters(),
        availableTags: ['steam', 'diesel'],
        availableScales: scales
      }
    });
    expect(screen.getByText('steam')).toBeInTheDocument();
    expect(screen.getByText('diesel')).toBeInTheDocument();
  });

  it('calls onToggleTag when a tag badge is clicked', async () => {
    const onToggleTag = vi.fn();
    render(FilterPanel, {
      props: {
        filters: makeFilters(),
        availableTags: ['steam'],
        availableScales: scales,
        onToggleTag
      }
    });
    await fireEvent.click(screen.getByText('steam'));
    expect(onToggleTag).toHaveBeenCalledWith('steam');
  });

  it('calls onToggleSidebar when close button is clicked', async () => {
    const onToggleSidebar = vi.fn();
    render(FilterPanel, {
      props: {
        filters: makeFilters(),
        availableTags: [],
        availableScales: scales,
        onToggleSidebar
      }
    });
    const closeBtn = screen.getByTitle('Close filters');
    await fireEvent.click(closeBtn);
    expect(onToggleSidebar).toHaveBeenCalledTimes(1);
  });

  it('renders search input with placeholder', () => {
    render(FilterPanel, {
      props: {
        filters: makeFilters(),
        availableTags: [],
        availableScales: scales
      }
    });
    expect(screen.getByPlaceholderText('Search your collection...')).toBeInTheDocument();
  });

  it('renders the persistent search hint', () => {
    render(FilterPanel, {
      props: {
        filters: makeFilters(),
        availableTags: [],
        availableScales: scales
      }
    });
    expect(
      screen.getByText(
        'Search manufacturer, product code, or description while keeping your filters applied.'
      )
    ).toBeInTheDocument();
  });

  it('renders with active scale filter highlighted (All badge when scale is null)', () => {
    render(FilterPanel, {
      props: {
        filters: makeFilters({ scale: null }),
        availableTags: [],
        availableScales: scales
      }
    });
    // "All" badge is rendered when scale is null
    expect(screen.getByText('All')).toBeInTheDocument();
  });
});
