import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor, cleanup } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';

vi.mock('$lib/paraglide/messages.js', () => ({
  drawer_section_wishlist: () => 'Wishlist Preferences',
  wishlist_modal_choose_or_create: () => 'Choose or Create Wishlist',
  wishlist_modal_select_list: () => 'Select a wishlist',
  wishlist_modal_select_placeholder: () => 'Select a wishlist',
  wishlist_modal_new_list_placeholder: () => 'Or create new list...',
  wishlist_modal_priority: () => 'Priority',
  wishlist_modal_desired_price: () => 'Desired Price',
  wishlist_field_notes: () => 'Notes',
  wishlist_priority_low: () => 'Low',
  wishlist_priority_normal: () => 'Normal',
  wishlist_priority_high: () => 'High'
}));

vi.mock('$lib/features/settings/RegionalManager.svelte', () => ({
  regionalManager: { getCurrencySymbol: (currency: string) => (currency === 'EUR' ? '€' : '$') }
}));

import WishlistSection from '$lib/components/drawer/sections/WishlistSection.svelte';
import type { WishlistPreview } from '$lib/bindings';

const wishlists: WishlistPreview[] = [
  { id: 'wl-1', name: 'My Wishlist', isDefault: true, count: 0n } as WishlistPreview,
  { id: 'wl-2', name: 'Future Buys', isDefault: false, count: 0n } as WishlistPreview
];

const defaultProps = {
  wishlistId: 'wl-1',
  newListName: '',
  wishlists,
  priority: 'NORMAL' as const,
  desiredPrice: null,
  currency: 'EUR'
};

describe('WishlistSection', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
  });

  // ── Section header ──────────────────────────────────────────────────────────

  it('renders section header label', () => {
    render(WishlistSection, { props: defaultProps });
    expect(screen.getByText('Wishlist Preferences')).toBeInTheDocument();
  });

  // ── Wishlist selector ───────────────────────────────────────────────────────

  it('renders wishlist selector and new list input', () => {
    render(WishlistSection, { props: defaultProps });
    expect(screen.getByRole('button', { name: /select a wishlist/i })).toBeInTheDocument();
    expect(screen.getByPlaceholderText('Or create new list...')).toBeInTheDocument();
  });

  it('shows available wishlists in dropdown', async () => {
    const user = userEvent.setup();
    render(WishlistSection, { props: defaultProps });

    await user.click(screen.getByRole('button', { name: /select a wishlist/i }));

    await waitFor(() => {
      expect(screen.getByRole('option', { name: /my wishlist/i })).toBeInTheDocument();
      expect(screen.getByRole('option', { name: /future buys/i })).toBeInTheDocument();
    });
  });

  // ── Priority toggle ─────────────────────────────────────────────────────────

  it('renders all three priority buttons', () => {
    render(WishlistSection, { props: defaultProps });
    expect(screen.getByRole('button', { name: 'Low' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Normal' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'High' })).toBeInTheDocument();
  });

  it('NORMAL active button has amber bg class', () => {
    render(WishlistSection, { props: { ...defaultProps, priority: 'NORMAL' } });
    const normalBtn = screen.getByRole('button', { name: 'Normal' });
    expect(normalBtn.className).toContain('bg-[#D48A42]');
    expect(normalBtn.className).toContain('text-black');
  });

  it('LOW active button has amber border/text (not solid bg)', () => {
    render(WishlistSection, { props: { ...defaultProps, priority: 'LOW' } });
    const lowBtn = screen.getByRole('button', { name: 'Low' });
    expect(lowBtn.className).toContain('bg-[#D48A42]/10');
    expect(lowBtn.className).toContain('text-[#D48A42]');
  });

  it('HIGH active button has amber border/text (not solid bg)', () => {
    render(WishlistSection, { props: { ...defaultProps, priority: 'HIGH' } });
    const highBtn = screen.getByRole('button', { name: 'High' });
    expect(highBtn.className).toContain('bg-[#D48A42]/10');
    expect(highBtn.className).toContain('text-[#D48A42]');
  });

  it('inactive buttons have neutral text colour', () => {
    render(WishlistSection, { props: { ...defaultProps, priority: 'NORMAL' } });
    const lowBtn = screen.getByRole('button', { name: 'Low' });
    // Inactive buttons have neutral zinc text, not amber
    expect(lowBtn.className).toContain('text-[#808080]');
    expect(lowBtn.className).not.toContain('text-[#D48A42]');
    expect(lowBtn.className).not.toContain('text-black');
  });

  // ── Desired price ───────────────────────────────────────────────────────────

  it('renders desired price label', () => {
    render(WishlistSection, { props: defaultProps });
    expect(screen.getByText('Desired Price')).toBeInTheDocument();
  });

  // ── Notes field conditional render ──────────────────────────────────────────

  it('renders notes field when notes prop is an empty string', () => {
    render(WishlistSection, { props: { ...defaultProps, notes: '' } });
    expect(screen.getByText('Notes')).toBeInTheDocument();
    expect(screen.getByRole('textbox', { name: /notes/i })).toBeInTheDocument();
  });

  it('does not render notes field when notes prop is undefined', () => {
    render(WishlistSection, { props: { ...defaultProps, notes: undefined } });
    expect(screen.queryByText('Notes')).toBeNull();
  });

  // ── Error messages ───────────────────────────────────────────────────────────

  it('shows wishlistId error when provided', () => {
    render(WishlistSection, {
      props: { ...defaultProps, errors: { wishlistId: 'Please select a wishlist' } }
    });
    expect(screen.getByText('Please select a wishlist')).toBeInTheDocument();
  });

  it('shows desiredPrice error when provided', () => {
    render(WishlistSection, {
      props: { ...defaultProps, errors: { desiredPrice: 'Invalid price' } }
    });
    expect(screen.getByText('Invalid price')).toBeInTheDocument();
  });

  // ── Priority click ───────────────────────────────────────────────────────────

  it('clicking a priority button updates active styling', async () => {
    const user = userEvent.setup();

    // Render with NORMAL selected; click HIGH and verify class changes
    render(WishlistSection, { props: { ...defaultProps, priority: 'NORMAL' } });

    const highBtn = screen.getByRole('button', { name: 'High' });
    // Initially HIGH is inactive — has neutral text colour
    expect(highBtn.className).toContain('text-[#808080]');

    await user.click(highBtn);

    // After click, HIGH should be active — gains amber text
    await waitFor(() => {
      expect(screen.getByRole('button', { name: 'High' }).className).toContain('text-[#D48A42]');
    });
  });
});
