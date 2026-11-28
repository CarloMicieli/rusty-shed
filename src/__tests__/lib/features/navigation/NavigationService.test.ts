import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { NavigationItem } from '$lib/features/navigation/types';

const { contextStore, svelteMocks } = vi.hoisted(() => {
  const store = new Map<symbol, unknown>();
  return {
    contextStore: store,
    svelteMocks: {
      setContext: vi.fn((key: symbol, value: unknown) => {
        store.set(key, value);
      }),
      getContext: vi.fn((key: symbol) => store.get(key))
    }
  };
});

vi.mock('svelte', () => svelteMocks);

import {
  NavigationService,
  getNavigationService,
  setNavigationService
} from '$lib/features/navigation/services/NavigationService.svelte';

const navItems: NavigationItem[] = [
  { id: 'home', label: 'Home', icon: 'house', href: '/' },
  { id: 'collection', label: 'Collection', icon: 'box', href: '/collection', badge: 2 }
];

describe('NavigationService', () => {
  beforeEach(() => {
    contextStore.clear();
    vi.clearAllMocks();
  });

  it('tracks active route and active item', () => {
    const service = new NavigationService();
    service.setItems(navItems);

    service.setActiveRoute('/collection');

    expect(service.activeRoute).toBe('/collection');
    expect(service.activeItem?.id).toBe('collection');
    expect(service.isActive('/collection')).toBe(true);
    expect(service.isActive('/')).toBe(false);
  });

  it('handles collapsed state toggles', () => {
    const service = new NavigationService();

    expect(service.isCollapsed).toBe(false);
    service.toggleCollapsed();
    expect(service.isCollapsed).toBe(true);

    service.setCollapsed(false);
    expect(service.isCollapsed).toBe(false);
  });

  it('navigates through navigateTo and context registration with config', () => {
    const service = setNavigationService({ items: navItems, collapsed: true });

    service.navigateTo('/collection');

    expect(service.items).toEqual(navItems);
    expect(service.isCollapsed).toBe(true);
    expect(service.activeRoute).toBe('/collection');
    expect(getNavigationService()).toBe(service);
  });

  it('accepts an injected service instance and throws when context is missing', () => {
    const injected = new NavigationService();

    const registered = setNavigationService(undefined, injected);
    expect(registered).toBe(injected);
    expect(getNavigationService()).toBe(injected);

    contextStore.clear();
    expect(() => getNavigationService()).toThrow('NavigationService not found in context');
  });
});
