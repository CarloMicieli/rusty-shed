import { cleanup, render, screen, waitFor } from '@testing-library/svelte';
import { beforeEach, describe, expect, it, vi } from 'vitest';

const mockService = vi.hoisted(() => ({
  fetchInventories: vi.fn(),
  fetchInventory: vi.fn(),
  createInventory: vi.fn(),
  renameInventory: vi.fn(),
  deleteInventory: vi.fn()
}));

vi.mock('$lib/features/track-inventory', () => ({
  getTrackInventoryContext: () => mockService
}));
vi.mock('$lib/paraglide/messages', async (importOriginal) => {
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
vi.mock('$lib/features/track-inventory/components/TrackCommandBar.svelte', () => ({
  default: function TrackCommandBarStub() {}
}));
vi.mock('$lib/features/track-inventory/components/CreateInventoryDialog.svelte', () => ({
  default: function CreateInventoryDialogStub() {}
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

import RailwayTracksPage from '../../routes/railway-tracks/+page.svelte';

describe('routes/railway-tracks/+page.svelte', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
  });

  it('loads inventories and auto-selects the first inventory', async () => {
    mockService.fetchInventories.mockResolvedValue([
      { id: 'inventory-1', name: 'Main Layout', track_count: 2 }
    ]);
    mockService.fetchInventory.mockResolvedValue({
      id: 'inventory-1',
      name: 'Main Layout',
      items: [],
      purchases: []
    });
    render(RailwayTracksPage);

    await waitFor(() => expect(mockService.fetchInventory).toHaveBeenCalledWith('inventory-1'));
  });

  it('shows the empty state when no inventories exist', async () => {
    mockService.fetchInventories.mockResolvedValue([]);
    render(RailwayTracksPage);

    await waitFor(() => {
      expect(screen.getByText('track_inventories_empty_title')).toBeInTheDocument();
    });
  });

  it('shows load errors without auto-selecting detail', async () => {
    mockService.fetchInventories.mockRejectedValue(new Error('Track database unavailable'));
    render(RailwayTracksPage);

    await waitFor(() => {
      expect(screen.getByText('Track database unavailable')).toBeInTheDocument();
    });
    expect(mockService.fetchInventory).not.toHaveBeenCalled();
  });
});
