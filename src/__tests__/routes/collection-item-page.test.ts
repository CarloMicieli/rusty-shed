import { cleanup, render, screen, waitFor } from '@testing-library/svelte';
import { beforeEach, describe, expect, it, vi } from 'vitest';

const mockCollectionStore = vi.hoisted(() => ({
  fetch: vi.fn().mockResolvedValue(undefined),
  refresh: vi.fn().mockResolvedValue(undefined),
  getItemById: vi.fn()
}));
const mockCommands = vi.hoisted(() => ({
  getCollectionItemDetails: vi.fn()
}));

vi.mock('$lib/state/collection.svelte', () => ({ collectionStore: mockCollectionStore }));
vi.mock('$lib/bindings', () => ({ commands: mockCommands }));
vi.mock('$lib/paraglide/runtime.js', () => ({ getLocale: () => 'en' }));
vi.mock('$lib/features/collection/utils/modelViewMapper', () => ({
  toRailwayModel: vi.fn(() => ({ id: 'model-1' }))
}));
vi.mock('$app/navigation', () => ({ goto: vi.fn().mockResolvedValue(undefined) }));
vi.mock('$lib/paraglide/messages.js', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return Object.fromEntries(
    Object.entries(actual).map(([key, value]) => [
      key,
      typeof value === 'function' ? () => key : value
    ])
  );
});
vi.mock('$lib/components/RailwayModelCard.svelte', () => ({
  default: function RailwayModelCardStub() {}
}));
vi.mock('$lib/components/DetailBackLink.svelte', () => ({
  default: function DetailBackLinkStub() {}
}));
vi.mock('$lib/features/collection/components/CollectionItemSidebar.svelte', () => ({
  default: function CollectionItemSidebarStub() {}
}));

import { page } from '$app/stores';
import CollectionItemPage from '../../routes/collection/[itemId]/+page.svelte';

const collectionItem = {
  id: 'trn:collection-item:item-1',
  railwayModel: {
    railwayModelId: 'trn:railway-model:model-1',
    description: 'Test locomotive'
  },
  purchaseInfo: null
};

describe('routes/collection/[itemId]/+page.svelte', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
    (page as unknown as { set: (value: unknown) => void }).set({
      url: new URL('http://localhost/collection/item-1'),
      params: { itemId: 'item-1' },
      route: { id: '/collection/[itemId]' },
      status: 200,
      error: null,
      data: {},
      state: {},
      form: undefined
    });
  });

  it('shows the loading state until collection data resolves', () => {
    mockCollectionStore.fetch.mockResolvedValue(undefined);
    render(CollectionItemPage);
    expect(screen.getByText('collection_item_loading')).toBeInTheDocument();
  });

  it('shows not-found state when the collection item is missing', async () => {
    mockCollectionStore.fetch.mockResolvedValue(undefined);
    mockCollectionStore.getItemById.mockReturnValue(undefined);
    render(CollectionItemPage);

    await waitFor(() => {
      expect(screen.getByText('collection_item_not_found')).toBeInTheDocument();
    });
  });

  it('shows unexpected load errors', async () => {
    mockCollectionStore.fetch.mockRejectedValue(new Error('Collection unavailable'));
    render(CollectionItemPage);

    await waitFor(() => {
      expect(screen.getByText('Collection unavailable')).toBeInTheDocument();
    });
  });

  it('loads item details and renders the detail layout', async () => {
    mockCollectionStore.fetch.mockResolvedValue(undefined);
    mockCollectionStore.getItemById.mockReturnValue(collectionItem);
    mockCommands.getCollectionItemDetails.mockResolvedValue({
      model: { id: 'model-1' },
      image: null,
      seller: null
    });
    render(CollectionItemPage);

    await waitFor(() => {
      expect(mockCommands.getCollectionItemDetails).toHaveBeenCalledWith(
        'trn:railway-model:model-1',
        null,
        'en'
      );
    });
  });
});
