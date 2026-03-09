import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import WishlistItemSidebar from '$lib/features/wishlists/components/WishlistItemSidebar.svelte';
import type { WishlistItem } from '$lib/bindings';

// ── Mocks ────────────────────────────────────────────────────────────────────

const { mockUpdateWishlistItem, mockToasterError } = vi.hoisted(() => ({
  mockUpdateWishlistItem: vi.fn(),
  mockToasterError: vi.fn()
}));

vi.mock('$lib/bindings', () => ({
  commands: {
    updateWishlistItem: mockUpdateWishlistItem
  }
}));

vi.mock('$lib/toaster', () => ({
  toaster: {
    error: mockToasterError,
    success: vi.fn(),
    loading: vi.fn(),
    dismiss: vi.fn()
  }
}));

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
  wishlist_item_status_ignored: () => 'Ignored',
  wishlist_item_price_invalid_format: () => 'Price must be a number',
  wishlist_item_price_negative: () => 'Price must be zero or greater',
  wishlist_item_edit_field_label: ({ field }: { field: string }) => `Edit ${field}`,
  wishlist_item_edit_cancel_label: () => 'Cancel editing',
  wishlist_item_error: () => 'Failed to load item'
}));

// ── Fixtures ─────────────────────────────────────────────────────────────────

const TEST_WISHLIST_ID = 'trn:wishlist:11111111-1111-1111-1111-111111111111';
const TEST_ITEM_ID = 'trn:wishlist-item:22222222-2222-2222-2222-222222222222';

const baseItem: WishlistItem = {
  id: TEST_ITEM_ID,
  railwayModelId: 'trn:railway-model:test:0001',
  priority: 'NORMAL',
  status: 'WANTED',
  addedDate: '2026-02-23',
  removedDate: null,
  notes: null,
  desiredPrice: null,
  purchasedPrice: null
};

function renderSidebar(item: WishlistItem = baseItem, overrides: object = {}) {
  return render(WishlistItemSidebar, {
    item,
    wishlistId: TEST_WISHLIST_ID,
    wishlistName: 'My Test List',
    defaultCurrency: 'EUR',
    ...overrides
  });
}

// ── Base rendering tests ──────────────────────────────────────────────────────

describe('WishlistItemSidebar — rendering', () => {
  it('renders wishlist name', () => {
    renderSidebar();
    expect(screen.getByText('My Test List')).toBeTruthy();
  });

  it('renders correct priority label for NORMAL', () => {
    renderSidebar();
    expect(screen.getByText('Normal')).toBeTruthy();
  });

  it('renders correct priority label for HIGH', () => {
    renderSidebar({ ...baseItem, priority: 'HIGH' });
    expect(screen.getByText('High')).toBeTruthy();
  });

  it('renders desired price when set', () => {
    renderSidebar({ ...baseItem, desiredPrice: { amount: BigInt(9900), currency: 'EUR' } });
    expect(screen.queryByText('Not set')).toBeNull();
  });

  it('renders "Not set" when desired price is null', () => {
    renderSidebar({ ...baseItem, desiredPrice: null });
    expect(screen.getByText('Not set')).toBeTruthy();
  });

  it('hides purchased price row when purchasedPrice is null', () => {
    renderSidebar({ ...baseItem, purchasedPrice: null });
    expect(screen.queryByText('Purchased Price')).toBeNull();
  });

  it('shows purchased price row when purchasedPrice is set', () => {
    renderSidebar({ ...baseItem, purchasedPrice: { amount: BigInt(8500), currency: 'GBP' } });
    expect(screen.getByText('Purchased Price')).toBeTruthy();
  });
});

// ── US5: List field is permanently read-only (T024/T025) ─────────────────────

describe('WishlistItemSidebar — US5 List field read-only', () => {
  it('List field renders as plain text (not a button)', () => {
    renderSidebar();
    const listDd = screen.getByText('My Test List');
    expect(listDd.tagName.toLowerCase()).not.toBe('button');
    // No aria-label for Edit on the list name cell
    expect(listDd.closest('[aria-label]')).toBeNull();
  });

  it('clicking List field does not open any input or combobox', async () => {
    renderSidebar();
    const listCell = screen.getByText('My Test List');
    await fireEvent.click(listCell);
    // No text input should appear
    expect(screen.queryByRole('textbox')).toBeNull();
  });
});

// ── US1: Priority inline select — trigger tests (T015/T016) ──────────────────

describe('WishlistItemSidebar — US1 Priority inline edit', () => {
  beforeEach(() => {
    mockUpdateWishlistItem.mockResolvedValue({ status: 'ok', data: baseItem });
  });

  it('renders Priority as an editable trigger with aria-label', () => {
    renderSidebar();
    const trigger = screen.getByLabelText(/Edit Priority/i);
    expect(trigger).toBeTruthy();
    expect(trigger.textContent).toContain('Normal');
  });

  it('Priority trigger has correct aria-label', () => {
    renderSidebar();
    const trigger = screen.getByLabelText(/Edit Priority/i);
    expect(trigger.getAttribute('aria-label')).toContain('Priority');
  });

  it('Priority trigger shows current value (HIGH)', () => {
    renderSidebar({ ...baseItem, priority: 'HIGH' });
    const trigger = screen.getByLabelText(/Edit Priority/i);
    expect(trigger.textContent).toContain('High');
  });

  it('Priority trigger shows current value (LOW)', () => {
    renderSidebar({ ...baseItem, priority: 'LOW' });
    const trigger = screen.getByLabelText(/Edit Priority/i);
    expect(trigger.textContent).toContain('Low');
  });

  it('Priority Select.Root is rendered and wired up correctly', () => {
    renderSidebar();
    const trigger = screen.getByLabelText(/Edit Priority/i);
    // The trigger should have aria-haspopup indicating it controls a listbox
    expect(trigger.getAttribute('aria-haspopup')).toBe('listbox');
    expect(trigger.getAttribute('aria-expanded')).toBe('false');
  });
});

// ── US2: Status inline select — trigger tests (T017/T018) ────────────────────

describe('WishlistItemSidebar — US2 Status inline edit', () => {
  beforeEach(() => {
    mockUpdateWishlistItem.mockResolvedValue({ status: 'ok', data: baseItem });
  });

  it('renders Status as an editable trigger with aria-label', () => {
    renderSidebar();
    const trigger = screen.getByLabelText(/Edit Status/i);
    expect(trigger).toBeTruthy();
    expect(trigger.textContent).toContain('Wanted');
  });

  it('Status trigger has correct aria-label', () => {
    renderSidebar();
    const trigger = screen.getByLabelText(/Edit Status/i);
    expect(trigger.getAttribute('aria-label')).toContain('Status');
  });

  it('Status trigger shows current value (ON_ORDER)', () => {
    renderSidebar({ ...baseItem, status: 'ON_ORDER' });
    const trigger = screen.getByLabelText(/Edit Status/i);
    expect(trigger.textContent).toContain('On Order');
  });

  it('Status trigger shows current value (PURCHASED)', () => {
    renderSidebar({ ...baseItem, status: 'PURCHASED' });
    const trigger = screen.getByLabelText(/Edit Status/i);
    expect(trigger.textContent).toContain('Purchased');
  });

  it('Status Select.Root is rendered and wired up correctly', () => {
    renderSidebar();
    const trigger = screen.getByLabelText(/Edit Status/i);
    expect(trigger.getAttribute('aria-haspopup')).toBe('listbox');
    expect(trigger.getAttribute('aria-expanded')).toBe('false');
  });
});

// ── US3: Desired Price inline input (T019/T020/T021) ─────────────────────────

describe('WishlistItemSidebar — US3 Desired Price inline edit', () => {
  beforeEach(() => {
    mockUpdateWishlistItem.mockResolvedValue({ status: 'ok', data: baseItem });
  });

  it('shows clickable "Not set" button with aria-label when no price', () => {
    renderSidebar();
    const btn = screen.getByRole('button', { name: /Edit Desired Price/i });
    expect(btn.textContent).toContain('Not set');
  });

  it('clicking price field reveals text input', async () => {
    const user = userEvent.setup();
    renderSidebar();
    await user.click(screen.getByRole('button', { name: /Edit Desired Price/i }));
    await waitFor(() => {
      expect(screen.getByRole('textbox')).toBeTruthy();
    });
  });

  it('entering valid amount and pressing Enter calls updateWishlistItem', async () => {
    const user = userEvent.setup();
    renderSidebar();
    await user.click(screen.getByRole('button', { name: /Edit Desired Price/i }));
    const input = await waitFor(() => screen.getByRole('textbox'));
    await user.clear(input);
    await user.type(input, '49.99');
    await user.keyboard('{Enter}');

    await waitFor(() => {
      expect(mockUpdateWishlistItem).toHaveBeenCalledWith(
        expect.objectContaining({
          wishlistId: TEST_WISHLIST_ID,
          itemId: TEST_ITEM_ID,
          desiredPriceAmount: BigInt(4999),
          desiredPriceCurrency: 'EUR'
        })
      );
    });
  });

  it('entering non-numeric value shows inline error', async () => {
    const user = userEvent.setup();
    renderSidebar();
    await user.click(screen.getByRole('button', { name: /Edit Desired Price/i }));
    const input = await waitFor(() => screen.getByRole('textbox'));
    await user.clear(input);
    await user.type(input, 'abc');
    await user.keyboard('{Enter}');

    await waitFor(() => {
      expect(screen.getByText('Price must be a number')).toBeTruthy();
    });
    expect(mockUpdateWishlistItem).not.toHaveBeenCalled();
  });

  it('entering negative value shows inline error', async () => {
    const user = userEvent.setup();
    renderSidebar();
    await user.click(screen.getByRole('button', { name: /Edit Desired Price/i }));
    const input = await waitFor(() => screen.getByRole('textbox'));
    await user.clear(input);
    await user.type(input, '-5');
    await user.keyboard('{Enter}');

    await waitFor(() => {
      expect(screen.getByText('Price must be zero or greater')).toBeTruthy();
    });
    expect(mockUpdateWishlistItem).not.toHaveBeenCalled();
  });

  it('clearing input and pressing Enter calls updateWishlistItem with null price', async () => {
    const user = userEvent.setup();
    renderSidebar({
      ...baseItem,
      desiredPrice: { amount: BigInt(9900), currency: 'EUR' }
    });
    await user.click(screen.getByRole('button', { name: /Edit Desired Price/i }));
    const input = await waitFor(() => screen.getByRole('textbox'));
    await user.clear(input);
    await user.keyboard('{Enter}');

    await waitFor(() => {
      expect(mockUpdateWishlistItem).toHaveBeenCalledWith(
        expect.objectContaining({
          wishlistId: TEST_WISHLIST_ID,
          itemId: TEST_ITEM_ID,
          desiredPriceAmount: null,
          desiredPriceCurrency: null
        })
      );
    });
  });

  it('pressing Escape cancels without calling updateWishlistItem', async () => {
    const user = userEvent.setup();
    renderSidebar();
    await user.click(screen.getByRole('button', { name: /Edit Desired Price/i }));
    await waitFor(() => screen.getByRole('textbox'));
    await user.keyboard('{Escape}');

    await waitFor(() => {
      expect(screen.queryByRole('textbox')).toBeNull();
    });
    expect(mockUpdateWishlistItem).not.toHaveBeenCalled();
  });

  it('failure path: shows toaster error when update fails', async () => {
    mockUpdateWishlistItem.mockResolvedValue({ status: 'error', error: 'fail' });
    const user = userEvent.setup();
    renderSidebar();
    await user.click(screen.getByRole('button', { name: /Edit Desired Price/i }));
    const input = await waitFor(() => screen.getByRole('textbox'));
    await user.clear(input);
    await user.type(input, '10');
    await user.keyboard('{Enter}');

    await waitFor(() => {
      expect(mockToasterError).toHaveBeenCalled();
    });
  });
});

// ── US4: Added Date calendar (T022/T023) ─────────────────────────────────────

describe('WishlistItemSidebar — US4 Added Date calendar', () => {
  beforeEach(() => {
    mockUpdateWishlistItem.mockResolvedValue({ status: 'ok', data: baseItem });
  });

  it('renders Added date as a button with an Edit aria-label', () => {
    renderSidebar();
    const btn = screen.getByRole('button', { name: /Edit Added/i });
    expect(btn).toBeTruthy();
  });

  it('Added date button shows formatted date text', () => {
    renderSidebar();
    const btn = screen.getByRole('button', { name: /Edit Added/i });
    // Date is present in the button
    expect(btn.textContent?.trim().length).toBeGreaterThan(0);
  });
});
