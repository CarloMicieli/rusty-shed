import { describe, it, expect } from 'vitest';
import { NAVIGATION_ITEMS } from '$lib/components/navigation/config';
import { isActive } from '$lib/components/navigation/utils';

/**
 * Tests for navigation config changes introduced in feature 027-wishlist-item-detail.
 * Verifies that the wishlists nav item stays highlighted on /wishlists/* paths.
 */
describe('Navigation config - wishlists additionalPrefixes', () => {
  const wishlistsItem = NAVIGATION_ITEMS.find((item) => item.id === 'wishlists');

  it('wishlists nav item uses prefix match for /wishlists', () => {
    expect(wishlistsItem?.usePrefixMatch).toBe(true);
    expect(wishlistsItem?.href).toBe('/wishlists');
  });

  it('isActive returns true for /wishlists/abc/items/xyz', () => {
    expect(wishlistsItem).toBeDefined();
    expect(isActive(wishlistsItem!, '/wishlists/abc/items/xyz')).toBe(true);
  });

  it('isActive returns true for exact /wishlists match', () => {
    expect(wishlistsItem).toBeDefined();
    expect(isActive(wishlistsItem!, '/wishlists')).toBe(true);
  });

  it('isActive returns false for unrelated paths', () => {
    expect(wishlistsItem).toBeDefined();
    expect(isActive(wishlistsItem!, '/collection')).toBe(false);
    expect(isActive(wishlistsItem!, '/dashboard')).toBe(false);
    expect(isActive(wishlistsItem!, '/finance')).toBe(false);
  });
});
