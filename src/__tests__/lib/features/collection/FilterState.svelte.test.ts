import { describe, it, expect } from 'vitest';
import {
  createEmptyFilterState,
  hasActiveFilters
} from '$lib/features/collection/domain/FilterState';

// FilterState uses SvelteSet from svelte/reactivity — requires browser/svelte runtime.

describe('createEmptyFilterState', () => {
  it('creates a filter state with empty query', () => {
    const state = createEmptyFilterState();
    expect(state.query).toBe('');
  });

  it('creates a filter state with null scale', () => {
    const state = createEmptyFilterState();
    expect(state.scale).toBeNull();
  });

  it('creates a filter state with an empty tags set', () => {
    const state = createEmptyFilterState();
    expect(state.tags.size).toBe(0);
  });

  it('creates independent instances (no shared state)', () => {
    const a = createEmptyFilterState();
    const b = createEmptyFilterState();

    a.tags.add('FEATURED');

    expect(a.tags.size).toBe(1);
    expect(b.tags.size).toBe(0);
  });
});

describe('hasActiveFilters', () => {
  it('returns false for a freshly created empty state', () => {
    const state = createEmptyFilterState();
    expect(hasActiveFilters(state)).toBe(false);
  });

  it('returns true when query is non-empty', () => {
    const state = createEmptyFilterState();
    state.query = 'Roco';
    expect(hasActiveFilters(state)).toBe(true);
  });

  it('returns false when query contains only whitespace', () => {
    const state = createEmptyFilterState();
    state.query = '   ';
    expect(hasActiveFilters(state)).toBe(false);
  });

  it('returns true when scale is set', () => {
    const state = createEmptyFilterState();
    state.scale = 'H0';
    expect(hasActiveFilters(state)).toBe(true);
  });

  it('returns false when scale is null', () => {
    const state = createEmptyFilterState();
    state.scale = null;
    expect(hasActiveFilters(state)).toBe(false);
  });

  it('returns true when tags set has at least one entry', () => {
    const state = createEmptyFilterState();
    state.tags.add('FEATURED');
    expect(hasActiveFilters(state)).toBe(true);
  });

  it('returns false when tags set is empty', () => {
    const state = createEmptyFilterState();
    expect(hasActiveFilters(state)).toBe(false);
  });

  it('returns true when multiple filters are active simultaneously', () => {
    const state = createEmptyFilterState();
    state.query = 'Märklin';
    state.scale = 'N';
    state.tags.add('WISHLIST');
    expect(hasActiveFilters(state)).toBe(true);
  });

  it('returns false after clearing all filters', () => {
    const state = createEmptyFilterState();
    state.query = 'test';
    state.scale = 'H0';
    state.tags.add('FEATURED');

    // Reset all
    state.query = '';
    state.scale = null;
    state.tags.clear();

    expect(hasActiveFilters(state)).toBe(false);
  });

  it('returns false after removing the only tag', () => {
    const state = createEmptyFilterState();
    state.tags.add('FEATURED');
    state.tags.delete('FEATURED');
    expect(hasActiveFilters(state)).toBe(false);
  });
});
