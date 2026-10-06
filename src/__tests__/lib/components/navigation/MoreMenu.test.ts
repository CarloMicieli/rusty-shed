import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import type { SvelteComponent } from 'svelte';
import type { NavigationItem } from '$lib/components/navigation/types';
import MoreMenu from '$lib/components/navigation/MoreMenu.svelte';
import IconStub from '../../../stubs/IconStub.svelte';

const iconStub = IconStub as unknown as typeof SvelteComponent<Record<string, never>>;

let currentPathname = '/maintenance';

vi.mock('$app/stores', () => ({
  page: {
    subscribe: (cb: (value: { url: { pathname: string } }) => void) => {
      cb({ url: { pathname: currentPathname } });
      return () => {};
    }
  }
}));

// Mock $app/paths
vi.mock('$app/paths', () => ({
  resolve: (path: string) => path
}));

vi.mock('$lib/paraglide/messages.js', () => ({
  app_settings: () => 'Settings',
  app_debug: () => 'Debug',
  mobile_more_top_actions: () => 'Quick actions'
}));

vi.mock('$lib/components/ui/sheet', async () => ({
  Sheet: (await import('../../../stubs/SheetStub.svelte')).default
}));

const mockSecondaryItems: NavigationItem[] = [
  {
    id: 'maintenance',
    label: () => 'Maintenance',
    icon: iconStub,
    href: '/maintenance',
    isPrimary: false,
    usePrefixMatch: true
  },
  {
    id: 'depot',
    label: () => 'Depot',
    icon: iconStub,
    href: '/depot',
    isPrimary: false
  },
  {
    id: 'digital-dcc',
    label: () => 'Digital (DCC)',
    icon: iconStub,
    href: '/digital-dcc',
    isPrimary: false
  },
  {
    id: 'railway-tracks',
    label: () => 'Railway Tracks',
    icon: iconStub,
    href: '/railway-tracks',
    isPrimary: false,
    usePrefixMatch: true
  }
];

describe('MoreMenu', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    currentPathname = '/maintenance';
  });

  it('marks active route link with aria-current', () => {
    const onClose = vi.fn();

    render(MoreMenu, {
      props: {
        open: true,
        onClose,
        items: mockSecondaryItems
      }
    });

    const maintenanceLink = screen.getByRole('link', { name: /Maintenance/i });
    const depotLink = screen.getByRole('link', { name: /Depot/i });

    expect(maintenanceLink).toHaveAttribute('aria-current', 'page');
    expect(depotLink).not.toHaveAttribute('aria-current');
  });

  it('uses prefix matching when pathname is nested under a section', () => {
    currentPathname = '/railway-tracks/layout-a';
    const onClose = vi.fn();

    render(MoreMenu, {
      props: {
        open: true,
        onClose,
        items: mockSecondaryItems
      }
    });

    const tracksLink = screen.getByRole('link', { name: /Railway Tracks/i });
    expect(tracksLink).toHaveAttribute('aria-current', 'page');
  });

  it('calls onClose when a menu item is clicked', async () => {
    const onClose = vi.fn();

    render(MoreMenu, {
      props: {
        open: true,
        onClose,
        items: mockSecondaryItems
      }
    });

    await fireEvent.click(screen.getByRole('link', { name: /Depot/i }));
    expect(onClose).toHaveBeenCalledTimes(1);
  });

  it('closes when sheet emits open=false and ignores open=true', async () => {
    const onClose = vi.fn();

    render(MoreMenu, {
      props: {
        open: true,
        onClose,
        items: mockSecondaryItems
      }
    });

    await fireEvent.click(screen.getByTestId('sheet-open'));
    expect(onClose).not.toHaveBeenCalled();

    await fireEvent.click(screen.getByTestId('sheet-close'));
    expect(onClose).toHaveBeenCalledTimes(1);
  });
});
