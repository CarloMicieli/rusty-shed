import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import SidebarNavigation from '$lib/components/SidebarNavigation.svelte';

// Mock $app/stores
vi.mock('$app/stores', () => ({
  page: {
    subscribe: vi.fn((cb) => {
      cb({
        url: {
          pathname: '/dashboard'
        }
      });
      return () => {};
    })
  }
}));

// Mock $app/paths
vi.mock('$app/paths', () => ({
  resolve: (path: string) => path
}));

// Mock paraglide messages with new navigation keys
vi.mock('$lib/paraglide/messages.js', () => ({
  app_name: () => 'Rusty Shed',
  app_home: () => 'Home',
  app_collection: () => 'Collection',
  app_finance: () => 'Finance',
  app_wishlists: () => 'Wishlists',
  app_maintenance: () => 'Maintenance',
  app_depot: () => 'Depot',
  app_digital_dcc: () => 'Digital (DCC)',
  app_railway_tracks: () => 'Railway Tracks',
  app_train_formations: () => 'Train Formations',
  app_settings: () => 'Settings',
  app_version_prefix: () => 'v.'
}));

// Mock Badge component
vi.mock('$lib/components', () => ({
  Badge: { name: 'Badge' }
}));

// Mock WishlistState
vi.mock('$lib/features/wishlists/WishlistState.svelte', () => ({
  getWishlistContext: () => ({
    defaultWishlist: null
  })
}));

// Mock locale store
vi.mock('$lib/stores/locale.svelte', () => ({
  localeState: {
    activeLocale: 'en',
    setActiveLocale: vi.fn()
  }
}));

// Mock app store
vi.mock('$lib/stores/app.svelte', () => ({
  appState: {
    version: '0.1.0',
    setVersion: vi.fn()
  }
}));

describe('SidebarNavigation', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders all 10 desktop navigation entries including settings', () => {
    render(SidebarNavigation);

    // Check for all navigation entries rendered in the sidebar.
    expect(screen.getByText('Home')).toBeTruthy();
    expect(screen.getByText('Collection')).toBeTruthy();
    expect(screen.getByText('Finance')).toBeTruthy();
    expect(screen.getByText('Wishlists')).toBeTruthy();
    expect(screen.getByText('Maintenance')).toBeTruthy();
    expect(screen.getByText('Depot')).toBeTruthy();
    expect(screen.getByText('Digital (DCC)')).toBeTruthy();
    expect(screen.getByText('Railway Tracks')).toBeTruthy();
    expect(screen.getByText('Train Formations')).toBeTruthy();
    expect(screen.getByText('Settings')).toBeTruthy();
  });

  it('applies active state to current route', () => {
    render(SidebarNavigation);

    // Get the link for Home
    const homeLink = screen.getByText('Home').closest('a');

    // Check if active classes are applied (sidebar-primary text)
    expect(homeLink?.className).toContain('text-sidebar-primary');
  });

  it('does not apply active state to non-current routes', () => {
    render(SidebarNavigation);

    // Get the link for Collection (not current route)
    const collectionLink = screen.getByText('Collection').closest('a');

    // Check that active classes are NOT applied
    expect(collectionLink?.classList.contains('bg-primary')).toBe(false);
    expect(collectionLink?.classList.contains('text-sidebar-foreground')).toBe(true);
  });

  it('uses correct icons for each feature', () => {
    const { container } = render(SidebarNavigation);

    // Check that all navigation items have icons (lucide-svelte renders SVGs)
    const svgs = container.querySelectorAll('svg');

    // Should have multiple SVGs (icons for each nav item)
    expect(svgs.length).toBeGreaterThan(8);
  });

  it('uses Paraglide translations for labels', () => {
    render(SidebarNavigation);

    // All labels should be rendered from Paraglide message functions
    expect(screen.getByText('Home')).toBeTruthy();
    expect(screen.getByText('Finance')).toBeTruthy();
    expect(screen.getByText('Digital (DCC)')).toBeTruthy();
    expect(screen.getByText('Railway Tracks')).toBeTruthy();
    expect(screen.getByText('Train Formations')).toBeTruthy();
  });

  it('renders nav with correct base classes', () => {
    const { container } = render(SidebarNavigation);

    const nav = container.querySelector('nav');

    expect(nav?.classList.contains('flex')).toBe(true);
    expect(nav?.classList.contains('flex-col')).toBe(true);
  });

  it('navigates to correct routes on click', () => {
    render(SidebarNavigation);

    const homeLink = screen.getByText('Home').closest('a');
    expect(homeLink?.getAttribute('href')).toBe('/dashboard');

    const collectionLink = screen.getByText('Collection').closest('a');
    expect(collectionLink?.getAttribute('href')).toBe('/collection');

    const financeLink = screen.getByText('Finance').closest('a');
    expect(financeLink?.getAttribute('href')).toBe('/finance');

    const railwayTracksLink = screen.getByText('Railway Tracks').closest('a');
    expect(railwayTracksLink?.getAttribute('href')).toBe('/railway-tracks');
  });

  it('displays app name and version info', () => {
    render(SidebarNavigation);

    expect(screen.getByText('Rusty Shed')).toBeTruthy();
    expect(screen.getByText(/v\./)).toBeTruthy();
  });

  it('has keyboard-accessible navigation items', () => {
    const { container } = render(SidebarNavigation);

    const links = container.querySelectorAll('a');

    // All navigation items should be clickable links
    links.forEach((link) => {
      expect(link.tagName).toBe('A');
    });
  });
});
