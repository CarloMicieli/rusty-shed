import { describe, expect, it, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import BottomNavigation from '$lib/components/BottomNavigation.svelte';

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

vi.mock('$app/paths', () => ({
  resolve: (path: string) => path
}));

vi.mock('$lib/paraglide/messages.js', () => ({
  app_home: () => 'Home',
  app_collection: () => 'Collezione ferroviaria con etichetta molto lunga',
  app_finance: () => 'Finanze e bilanci estesi',
  app_wishlists: () => 'Liste desideri personalizzate molto lunghe',
  app_more: () => 'Altro con etichetta lunghissima',
  app_more_aria: () => 'Apri menu funzionalita aggiuntive'
}));

vi.mock('$lib/components/ui/badge', () => ({
  Badge: { name: 'Badge' }
}));

vi.mock('$lib/features/wishlists/WishlistState.svelte', () => ({
  getWishlistContext: () => ({
    defaultWishlist: null
  })
}));

describe('BottomNavigation mobile label constraints', () => {
  it('applies truncation classes for long localized labels', () => {
    const { container } = render(BottomNavigation);

    const labels = Array.from(container.querySelectorAll('span')).filter((el) =>
      el.className.includes('tracking-wider')
    );

    expect(labels.length).toBeGreaterThanOrEqual(5);
    for (const label of labels) {
      expect(label.className).toContain('truncate');
      expect(label.className).toContain('max-w-[5.5rem]');
    }

    expect(screen.getByText('Altro con etichetta lunghissima')).toBeInTheDocument();
  });
});
