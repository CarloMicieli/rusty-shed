import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import WishlistItemSidebar from '$lib/features/wishlists/components/WishlistItemSidebar.svelte';
import type { WishlistItem } from '$lib/bindings';

vi.mock('$lib/paraglide/messages.js', () => ({
  wishlist_item_section_details: () => 'Wish List Details',
  wishlist_item_wishlist_name: () => 'List',
  wishlist_field_priority: () => 'Priority',
  wishlist_item_status: () => 'Status',
  wishlist_field_desired_price: () => 'Desired Price',
  wishlist_item_price_not_set: () => 'Not set',
  wishlist_item_purchased_price: () => 'Purchased Price',
  wishlist_item_section_personal_context: () => 'Personal Context',
  wishlist_item_added_date: () => 'Added',
  wishlist_item_notes: () => 'Notes',
  wishlist_priority_low: () => 'Low',
  wishlist_priority_normal: () => 'Normal',
  wishlist_priority_high: () => 'High',
  wishlist_item_status_wanted: () => 'Wanted',
  wishlist_item_status_on_order: () => 'On Order',
  wishlist_item_status_purchased: () => 'Purchased',
  wishlist_item_status_ignored: () => 'Ignored'
}));

const baseItem: WishlistItem = {
  id: 'trn:wishlist-item:test-id',
  railwayModelId: 'trn:railway-model:test-model',
  priority: 'NORMAL',
  status: 'WANTED',
  addedDate: '2026-02-23',
  removedDate: null,
  notes: null,
  desiredPrice: null,
  purchasedPrice: null
};

describe('WishlistItemSidebar', () => {
  it('renders wishlist name', () => {
    render(WishlistItemSidebar, { item: baseItem, wishlistName: 'My Test List' });
    expect(screen.getByText('My Test List')).toBeTruthy();
  });

  it('renders correct priority label for NORMAL', () => {
    render(WishlistItemSidebar, { item: baseItem, wishlistName: 'Test' });
    expect(screen.getByText('Normal')).toBeTruthy();
  });

  it('renders correct priority label for HIGH', () => {
    render(WishlistItemSidebar, {
      item: { ...baseItem, priority: 'HIGH' },
      wishlistName: 'Test'
    });
    expect(screen.getByText('High')).toBeTruthy();
  });

  it('renders desired price when set', () => {
    render(WishlistItemSidebar, {
      item: { ...baseItem, desiredPrice: { amount: BigInt(9900), currency: 'EUR' } },
      wishlistName: 'Test'
    });
    // Should show a formatted price, not "Not set"
    expect(screen.queryByText('Not set')).toBeNull();
  });

  it('renders "Not set" when desired price is null', () => {
    render(WishlistItemSidebar, {
      item: { ...baseItem, desiredPrice: null },
      wishlistName: 'Test'
    });
    expect(screen.getByText('Not set')).toBeTruthy();
  });

  it('hides purchased price row when purchasedPrice is null', () => {
    render(WishlistItemSidebar, {
      item: { ...baseItem, purchasedPrice: null },
      wishlistName: 'Test'
    });
    expect(screen.queryByText('Purchased Price')).toBeNull();
  });

  it('shows purchased price row when purchasedPrice is set', () => {
    render(WishlistItemSidebar, {
      item: { ...baseItem, purchasedPrice: { amount: BigInt(8500), currency: 'GBP' } },
      wishlistName: 'Test'
    });
    expect(screen.getByText('Purchased Price')).toBeTruthy();
  });
});
