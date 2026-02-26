import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import SearchEmptyState from '$lib/features/search/components/SearchEmptyState.svelte';

vi.mock('$lib/paraglide/messages', () => ({
  search_no_results_title: () => 'No models found',
  search_no_results_body: ({ query }: { query: string }) =>
    `No items in your collection or wishlist match "${query}".`
}));

describe('SearchEmptyState', () => {
  it('renders the "no results" title', () => {
    render(SearchEmptyState, { props: { query: 'Tartaruga' } });
    expect(screen.getByText('No models found')).toBeInTheDocument();
  });

  it('includes the query in the body text', () => {
    render(SearchEmptyState, { props: { query: 'Tartaruga' } });
    expect(
      screen.getByText('No items in your collection or wishlist match "Tartaruga".')
    ).toBeInTheDocument();
  });

  it('handles an empty query gracefully', () => {
    render(SearchEmptyState, { props: { query: '' } });
    expect(screen.getByText('No models found')).toBeInTheDocument();
  });
});
