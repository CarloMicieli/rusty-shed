import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, cleanup } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';

vi.mock('$lib/features/settings/RegionalManager.svelte', () => ({
  regionalManager: {
    locale: 'en-US',
    currency: 'EUR',
    formatCurrencyWith: vi.fn((cents: number, currency: string) => {
      return `${currency} ${(cents / 100).toFixed(2)}`;
    })
  }
}));

vi.mock('$lib/paraglide/messages', () => ({
  track_inventory_active_label: () => 'ACTIVE INVENTORY',
  track_inventory_select_placeholder: () => 'Select an inventory',
  track_inventory_management_button: () => 'Management',
  track_inventory_rename_button: () => 'Rename',
  inventory_delete_action: () => 'Delete',
  track_inventory_detail_add_purchase: () => 'Add Purchase',
  track_inventories_card_total_quantity: () => 'Total Pieces',
  track_inventory_value_label: () => 'Inventory Value',
  track_inventory_last_purchase: () => 'Last Purchase'
}));

import TrackCommandBar from '../TrackCommandBar.svelte';
import type { TrackInventoryListItem, TrackInventoryView, TrackInventoryId } from '$lib/bindings';

function makeListItem(shortId: string, name: string, quantity = 0): TrackInventoryListItem {
  return {
    id: `trn:track-inventory:${shortId}` as TrackInventoryId,
    name,
    description: null,
    total_items: Number(0),
    total_quantity: Number(quantity)
  };
}

function makeInventoryView(shortId: string, name: string): TrackInventoryView {
  return {
    id: `trn:track-inventory:${shortId}` as TrackInventoryId,
    name,
    description: null,
    items: [
      {
        track_id: 'trn:track:acme:r1' as never,
        track_product: {} as never,
        quantity: Number(10),
        required: Number(0)
      },
      {
        track_id: 'trn:track:acme:r2' as never,
        track_product: {} as never,
        quantity: Number(5),
        required: Number(0)
      }
    ],
    purchases: [
      {
        id: 'trn:track-purchase:p1' as never,
        track_product: {} as never,
        quantity: Number(10),
        price: { amount: Number(1999), currency: 'EUR' as never },
        seller_name: 'Acme Shop',
        purchase_date: '2026-01-15'
      }
    ]
  } as unknown as TrackInventoryView;
}

describe('TrackCommandBar', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
  });

  // ── Zone A: label & selector ──────────────────────────────────────────────────

  it('renders "ACTIVE INVENTORY" label, placeholder when no inventory active, and active name when selected', () => {
    const inventories = [makeListItem('aaa', 'Main Layout'), makeListItem('bbb', 'Staging Yard')];

    const { unmount } = render(TrackCommandBar, {
      props: {
        inventories: [],
        activeInventoryId: null,
        activeInventory: null,
        onSelect: vi.fn()
      }
    });
    expect(screen.getByText('ACTIVE INVENTORY')).toBeInTheDocument();
    unmount();

    // With no active selection — placeholder shown
    const { unmount: u2 } = render(TrackCommandBar, {
      props: {
        inventories: [makeListItem('aaa', 'Main Layout')],
        activeInventoryId: null,
        activeInventory: null,
        onSelect: vi.fn()
      }
    });
    expect(screen.getByText('Select an inventory')).toBeInTheDocument();
    u2();

    // With active inventory — name shown in selector trigger
    render(TrackCommandBar, {
      props: {
        inventories,
        activeInventoryId: 'trn:track-inventory:aaa',
        activeInventory: null,
        onSelect: vi.fn()
      }
    });
    expect(screen.getByText('Main Layout')).toBeInTheDocument();
  });

  // ── Zone B: Metrics ───────────────────────────────────────────────────────────

  it('renders metric column labels and computed stats with active inventory; shows zeros/dash without inventory', () => {
    const inventory = makeInventoryView('aaa', 'Main Layout');

    const { unmount } = render(TrackCommandBar, {
      props: {
        inventories: [makeListItem('aaa', 'Main Layout')],
        activeInventoryId: 'trn:track-inventory:aaa',
        activeInventory: inventory,
        onSelect: vi.fn()
      }
    });
    expect(screen.getByText('Total Pieces')).toBeInTheDocument();
    expect(screen.getByText('Inventory Value')).toBeInTheDocument();
    expect(screen.getByText('Last Purchase')).toBeInTheDocument();
    // total pieces: 10 + 5 = 15
    expect(screen.getByText('15')).toBeInTheDocument();
    // formatted value from mock: EUR 19.99
    expect(screen.getByText('EUR 19.99')).toBeInTheDocument();
    unmount();

    render(TrackCommandBar, {
      props: {
        inventories: [],
        activeInventoryId: null,
        activeInventory: null,
        onSelect: vi.fn()
      }
    });
    expect(screen.getByText('0')).toBeInTheDocument();
    expect(screen.getByText('EUR 0.00')).toBeInTheDocument();
    expect(screen.getByText('—')).toBeInTheDocument();
  });

  // ── Zone C: Actions ───────────────────────────────────────────────────────────

  it('renders Management button and Add Purchase button', () => {
    render(TrackCommandBar, {
      props: {
        inventories: [],
        activeInventoryId: null,
        activeInventory: null,
        onSelect: vi.fn(),
        onRename: vi.fn(),
        onDelete: vi.fn(),
        onAddPurchase: vi.fn()
      }
    });
    expect(screen.getByText('Management')).toBeInTheDocument();
    expect(screen.getByText('Add Purchase')).toBeInTheDocument();
  });

  it('shows Rename and Delete options when Management is clicked', async () => {
    const user = userEvent.setup();
    render(TrackCommandBar, {
      props: {
        inventories: [],
        activeInventoryId: null,
        activeInventory: null,
        onSelect: vi.fn(),
        onRename: vi.fn(),
        onDelete: vi.fn()
      }
    });

    await user.click(screen.getByText('Management'));

    expect(screen.getByText('Rename')).toBeInTheDocument();
    expect(screen.getByText('Delete')).toBeInTheDocument();
  });

  it('calls onRename when Rename menu item is clicked', async () => {
    const user = userEvent.setup();
    const onRename = vi.fn();
    render(TrackCommandBar, {
      props: {
        inventories: [],
        activeInventoryId: null,
        activeInventory: null,
        onSelect: vi.fn(),
        onRename
      }
    });

    await user.click(screen.getByText('Management'));
    await user.click(screen.getByText('Rename'));

    expect(onRename).toHaveBeenCalledOnce();
  });

  it('calls onDelete when Delete menu item is clicked', async () => {
    const user = userEvent.setup();
    const onDelete = vi.fn();
    render(TrackCommandBar, {
      props: {
        inventories: [],
        activeInventoryId: null,
        activeInventory: null,
        onSelect: vi.fn(),
        onDelete
      }
    });

    await user.click(screen.getByText('Management'));
    await user.click(screen.getByText('Delete'));

    expect(onDelete).toHaveBeenCalledOnce();
  });

  it('calls onAddPurchase when Add Purchase button is clicked', async () => {
    const user = userEvent.setup();
    const onAddPurchase = vi.fn();
    render(TrackCommandBar, {
      props: {
        inventories: [],
        activeInventoryId: null,
        activeInventory: null,
        onSelect: vi.fn(),
        onAddPurchase
      }
    });

    await user.click(screen.getByText('Add Purchase'));

    expect(onAddPurchase).toHaveBeenCalledOnce();
  });
});
