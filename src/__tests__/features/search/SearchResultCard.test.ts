import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import SearchResultCard from '$lib/features/search/components/SearchResultCard.svelte';
import type { GlobalSearchResultView } from '$lib/bindings';

vi.mock('$lib/paraglide/messages', () => ({
  search_source_collection: () => 'Collection',
  search_source_wishlist: () => 'Wishlist'
}));

const makeResult = (overrides: Partial<GlobalSearchResultView> = {}): GlobalSearchResultView => ({
  railwayModelId: 'trn:railway-model:acme:60100',
  source: 'collection',
  itemId: 'col-item-001',
  parentId: null,
  displayName: 'Locomotiva E.444 Tartaruga',
  manufacturerName: 'ACME',
  ...overrides
});

describe('SearchResultCard', () => {
  it('renders the display name', () => {
    render(SearchResultCard, { props: { result: makeResult() } });
    expect(screen.getByText('Locomotiva E.444 Tartaruga')).toBeInTheDocument();
  });

  it('renders the manufacturer name', () => {
    render(SearchResultCard, { props: { result: makeResult() } });
    expect(screen.getByText('ACME')).toBeInTheDocument();
  });

  it('shows "Collection" badge for collection results', () => {
    render(SearchResultCard, { props: { result: makeResult({ source: 'collection' }) } });
    expect(screen.getByText('Collection')).toBeInTheDocument();
  });

  it('shows "Wishlist" badge for wishlist results', () => {
    render(SearchResultCard, {
      props: { result: makeResult({ source: 'wishlist', parentId: 'wl-001' }) }
    });
    expect(screen.getByText('Wishlist')).toBeInTheDocument();
  });

  it('calls onclick when clicked', async () => {
    const onclick = vi.fn();
    render(SearchResultCard, { props: { result: makeResult(), onclick } });
    await fireEvent.click(screen.getByRole('button'));
    expect(onclick).toHaveBeenCalledOnce();
  });

  it('falls back to railwayModelId when displayName is empty', () => {
    render(SearchResultCard, { props: { result: makeResult({ displayName: '' }) } });
    expect(screen.getByText('trn:railway-model:acme:60100')).toBeInTheDocument();
  });
});
