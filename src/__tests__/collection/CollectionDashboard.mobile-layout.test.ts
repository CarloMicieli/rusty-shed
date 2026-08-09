import { describe, expect, it } from 'vitest';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

describe('CollectionDashboard mobile layout contracts', () => {
  it('enforces mobile grid minimum width and hides table toggle on mobile', () => {
    const source = readFileSync(
      resolve('src/lib/features/collection/CollectionDashboard.svelte'),
      'utf8'
    );

    expect(source).toContain('itemMinWidth={isMobileViewport ? 320 : 240}');
    expect(source).toContain('class="mb-4 hidden items-center justify-end md:flex"');
    expect(source).toContain('safe-area-inset-bottom fixed right-4 bottom-4');
  });

  it('renders mobile filters as bottom sheet overlay with anti-bleed backdrop', () => {
    const source = readFileSync(
      resolve('src/lib/features/collection/CollectionDashboard.svelte'),
      'utf8'
    );

    expect(source).toContain('id="collection-mobile-filter-sheet"');
    expect(source).toContain('bg-black/80 backdrop-blur-md');
    expect(source).toContain('max-h-[85dvh]');
    expect(source).toContain('bg-card');
    expect(source).toContain('border border-border');
    expect(source).not.toContain('id="collection-mobile-filter-panel"');
    expect(source).not.toContain('FilterPanel');
  });

  it('keeps a single primary mobile search input in dashboard controls', () => {
    const source = readFileSync(
      resolve('src/lib/features/collection/CollectionDashboard.svelte'),
      'utf8'
    );

    expect(source).toContain('id="collection-mobile-search"');
    expect(source).not.toContain('search-input');
  });
});
