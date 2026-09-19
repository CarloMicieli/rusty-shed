import { describe, expect, it, vi } from 'vitest';
import { fireEvent, render, screen } from '@testing-library/svelte';
import type { SvelteComponent } from 'svelte';
import type { NavigationItem } from '$lib/components/navigation/types';
import MoreMenu from '$lib/components/navigation/MoreMenu.svelte';
import IconStub from '../../../stubs/IconStub.svelte';

const iconStub = IconStub as unknown as typeof SvelteComponent<Record<string, never>>;

vi.mock('$app/stores', () => ({
  page: {
    subscribe: (cb: (value: { url: { pathname: string } }) => void) => {
      cb({ url: { pathname: '/maintenance' } });
      return () => {};
    }
  }
}));

vi.mock('$app/paths', () => ({
  resolve: (path: string) => path
}));

vi.mock('$lib/components/ui/sheet', async () => ({
  Sheet: (await import('../../../stubs/SheetStub.svelte')).default
}));

vi.mock('$lib/paraglide/messages.js', () => ({
  app_settings: () => 'Settings',
  app_debug: () => 'Debug',
  mobile_more_top_actions: () => 'Quick actions'
}));

const mockSecondaryItems: NavigationItem[] = [
  {
    id: 'maintenance',
    label: () => 'Maintenance',
    icon: iconStub,
    href: '/maintenance',
    isPrimary: false,
    usePrefixMatch: true
  }
];

describe('MoreMenu mobile top actions', () => {
  it('shows Settings and Debug in top actions and keeps them reachable', async () => {
    const onClose = vi.fn();

    render(MoreMenu, {
      props: {
        open: true,
        onClose,
        items: mockSecondaryItems
      }
    });

    expect(screen.getByText('Quick actions')).toBeInTheDocument();

    const settings = screen.getByRole('link', { name: /Settings/i });
    const debug = screen.getByRole('link', { name: /Debug/i });

    expect(settings).toHaveAttribute('href', '/settings');
    expect(debug).toHaveAttribute('href', '/debug');

    await fireEvent.click(settings);
    expect(onClose).toHaveBeenCalledTimes(1);
  });
});
