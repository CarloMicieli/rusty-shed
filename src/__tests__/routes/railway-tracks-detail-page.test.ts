import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/svelte';
import { beforeEach, describe, expect, it, vi } from 'vitest';

const mockService = vi.hoisted(() => ({
  fetchInventory: vi.fn(),
  renameInventory: vi.fn(),
  deleteInventory: vi.fn()
}));
const mockGoto = vi.hoisted(() => vi.fn().mockResolvedValue(undefined));

vi.mock('$lib/features/track-inventory', () => ({
  getTrackInventoryContext: () => mockService
}));
vi.mock('$app/navigation', () => ({ goto: mockGoto }));
vi.mock('$app/paths', () => ({ resolve: (path: string) => path }));
vi.mock('$lib/paraglide/messages.js', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return Object.fromEntries(
    Object.entries(actual).map(([key, value]) => [
      key,
      typeof value === 'function' ? () => key : value
    ])
  );
});
vi.mock('$lib/features/track-inventory/components/InventoryDetail.svelte', () => ({
  default: function InventoryDetailStub() {}
}));
vi.mock('$lib/features/track-inventory/components/RenameInventoryDialog.svelte', () => ({
  default: function RenameInventoryDialogStub() {}
}));
vi.mock('$lib/features/track-inventory/components/DeleteInventoryDialog.svelte', () => ({
  default: function DeleteInventoryDialogStub() {}
}));
vi.mock('$lib/features/track-inventory/components/AddTracksPurchaseDrawer.svelte', () => ({
  default: function AddTracksPurchaseDrawerStub() {}
}));
vi.mock('$lib/components/DetailBackLink.svelte', () => ({
  default: function DetailBackLinkStub() {}
}));

import { page } from '$app/stores';
import RailwayTracksDetailPage from '../../routes/railway-tracks/[id]/+page.svelte';

describe('routes/railway-tracks/[id]/+page.svelte', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
    (page as unknown as { set: (value: unknown) => void }).set({
      url: new URL('http://localhost/railway-tracks/inventory-1'),
      params: { id: 'inventory-1' },
      route: { id: '/railway-tracks/[id]' },
      status: 200,
      error: null,
      data: {},
      state: {},
      form: undefined
    });
  });

  it('loads the inventory detail', async () => {
    mockService.fetchInventory.mockResolvedValue({
      id: 'inventory-1',
      name: 'Main Layout',
      description: 'Oval',
      items: [],
      purchases: []
    });
    render(RailwayTracksDetailPage);

    await waitFor(() => expect(screen.getByText('Main Layout')).toBeInTheDocument());
    expect(screen.getByText('Oval')).toBeInTheDocument();
  });

  it('shows not-found state for a null inventory', async () => {
    mockService.fetchInventory.mockResolvedValue(null);
    render(RailwayTracksDetailPage);

    await waitFor(() => {
      expect(screen.getByText('track_inventories_not_found')).toBeInTheDocument();
    });
  });

  it('shows load errors', async () => {
    mockService.fetchInventory.mockRejectedValue(new Error('Inventory read failed'));
    render(RailwayTracksDetailPage);

    await waitFor(() => {
      expect(screen.getByText('Inventory read failed')).toBeInTheDocument();
    });
  });

  it('opens management actions and add-purchase action', async () => {
    mockService.fetchInventory.mockResolvedValue({
      id: 'inventory-1',
      name: 'Main Layout',
      description: null,
      items: [],
      purchases: []
    });
    render(RailwayTracksDetailPage);
    await waitFor(() => expect(screen.getByText('Main Layout')).toBeInTheDocument());

    await fireEvent.click(
      screen.getByRole('button', { name: 'track_inventory_management_button' })
    );
    expect(screen.getByText('track_inventory_rename_button')).toBeInTheDocument();
  });
});
