import { describe, it, expect } from 'vitest';
import { render } from '@testing-library/svelte';

import DetailBackLink from '../DetailBackLink.svelte';

describe('DetailBackLink', () => {
  it('builds href with query parameters when provided', () => {
    const { container } = render(DetailBackLink, {
      props: {
        path: '/wishlists',
        ariaLabel: 'Back to wishlists',
        query: { view: 'table' }
      }
    });

    const link = container.querySelector('a');
    expect(link?.getAttribute('href')).toBe('/wishlists?view=table');
  });

  it('omits empty query values', () => {
    const { container } = render(DetailBackLink, {
      props: {
        path: '/wishlists',
        ariaLabel: 'Back to wishlists',
        query: { view: '' }
      }
    });

    const link = container.querySelector('a');
    expect(link?.getAttribute('href')).toBe('/wishlists');
  });
});
