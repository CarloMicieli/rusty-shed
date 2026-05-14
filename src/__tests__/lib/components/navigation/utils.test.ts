import { describe, expect, it } from 'vitest';
import type { SvelteComponent } from 'svelte';
import type { NavigationItem } from '$lib/components/navigation/types';
import { isActive, isMoreButtonActive } from '$lib/components/navigation/utils';

const IconStub = class {} as unknown as typeof SvelteComponent<Record<string, never>>;

function makeItem(overrides: Partial<NavigationItem>): NavigationItem {
  return {
    id: 'item',
    label: () => 'Item',
    icon: IconStub,
    href: '/dashboard',
    isPrimary: false,
    ...overrides
  };
}

describe('navigation/utils', () => {
  it('matches exact href when no prefix options are configured', () => {
    const item = makeItem({ href: '/dashboard' });

    expect(isActive(item, '/dashboard')).toBe(true);
    expect(isActive(item, '/dashboard/details')).toBe(false);
  });

  it('matches by href prefix when usePrefixMatch is enabled', () => {
    const item = makeItem({ href: '/railway-tracks', usePrefixMatch: true });

    expect(isActive(item, '/railway-tracks/layouts')).toBe(true);
  });

  it('matches by additionalPrefixes when configured', () => {
    const item = makeItem({ href: '/collection', additionalPrefixes: ['/model-details'] });

    expect(isActive(item, '/model-details/trn:model:123')).toBe(true);
  });

  it('reports More button active when at least one secondary item is active', () => {
    const items: NavigationItem[] = [
      makeItem({ href: '/maintenance' }),
      makeItem({ href: '/depot', usePrefixMatch: true })
    ];

    expect(isMoreButtonActive(items, '/depot/shelf-a')).toBe(true);
    expect(isMoreButtonActive(items, '/unknown')).toBe(false);
  });
});