import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/svelte';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import type { CollectionItemView, RailwayModelView } from '$lib/bindings';

const mockedGoto = vi.hoisted(() => vi.fn());

const mockCommands = vi.hoisted(() => ({
  searchRailwayModels: vi.fn(),
  getRailwayModelById: vi.fn()
}));

const mockCollectionStore = vi.hoisted(() => ({
  items: [] as CollectionItemView[]
}));

vi.mock('$app/navigation', () => ({
  goto: mockedGoto
}));

vi.mock('$lib/paraglide/messages.js', () => ({
  app_search_placeholder: () => 'app_search_placeholder',
  app_search_instruction: () => 'app_search_instruction',
  app_search_mobile_placeholder: () => 'app_search_mobile_placeholder'
}));

vi.mock('$lib/paraglide/runtime.js', () => ({
  getLocale: vi.fn(() => 'en')
}));

vi.mock('$lib/bindings', () => ({
  commands: mockCommands
}));

vi.mock('$lib/state/collection.svelte', () => ({
  collectionStore: mockCollectionStore
}));

import SearchBar from '$lib/components/SearchBar.svelte';

function makeModelView(overrides: Partial<RailwayModelView> = {}): RailwayModelView {
  return {
    id: 'rm-1',
    manufacturer: {
      manufacturerId: 'manufacturer-1',
      display: 'Roco'
    },
    productCode: 'R1234',
    description: 'BR 101 Red',
    descriptionLang: 'en',
    details: null,
    detailsLang: null,
    powerMethod: 'DC',
    scale: 'H0',
    epoch: 'IV',
    category: 'LOCOMOTIVES',
    deliveryDate: null,
    availabilityStatus: null,
    metadata: {
      version: 1,
      created_at: '2026-01-01T00:00:00Z',
      updated_at: '2026-01-01T00:00:00Z'
    },
    rollingStock: [],
    ...overrides
  };
}

describe('SearchBar', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
    vi.useFakeTimers();

    mockCollectionStore.items = [];
    mockCommands.searchRailwayModels.mockResolvedValue({ status: 'ok', data: [] });
    mockCommands.getRailwayModelById.mockResolvedValue({ status: 'ok', data: null });
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it('does not run search for queries shorter than 2 characters', async () => {
    render(SearchBar);

    const input = screen.getByPlaceholderText('app_search_placeholder');
    await fireEvent.input(input, { target: { value: 'a' } });

    vi.advanceTimersByTime(350);

    await Promise.resolve();

    expect(mockCommands.searchRailwayModels).not.toHaveBeenCalled();
  });

  it('debounces input and searches only with the latest query', async () => {
    render(SearchBar);

    const input = screen.getByPlaceholderText('app_search_placeholder');

    await fireEvent.input(input, { target: { value: 'br' } });
    await fireEvent.input(input, { target: { value: 'br 101' } });

    vi.advanceTimersByTime(299);
    expect(mockCommands.searchRailwayModels).not.toHaveBeenCalled();

    vi.advanceTimersByTime(1);

    await waitFor(() => {
      expect(mockCommands.searchRailwayModels).toHaveBeenCalledTimes(1);
      expect(mockCommands.searchRailwayModels).toHaveBeenCalledWith({ query: 'br 101' });
    });
  });

  it('navigates to full search page when Enter is pressed with valid query', async () => {
    render(SearchBar);

    const input = screen.getByPlaceholderText('app_search_placeholder');
    await fireEvent.input(input, { target: { value: '  br 101  ' } });
    await fireEvent.keyDown(input, { key: 'Enter' });

    expect(mockedGoto).toHaveBeenCalledWith('/search?q=br%20101');
  });

  it('shows no-results state when backend search returns error status', async () => {
    mockCommands.searchRailwayModels.mockResolvedValue({
      status: 'error',
      error: { Unknown: 'boom' }
    });

    render(SearchBar);

    const input = screen.getByPlaceholderText('app_search_placeholder');
    await fireEvent.input(input, { target: { value: 'steam' } });

    vi.advanceTimersByTime(300);

    expect(await screen.findByText('No results found')).toBeInTheDocument();
    expect(mockCommands.getRailwayModelById).not.toHaveBeenCalled();
  });

  it('opens matching collection detail when a search result is selected', async () => {
    const modelId = 'rm-88';

    mockCollectionStore.items = [
      {
        id: 'col-55',
        collectionId: 'collection-1',
        railwayModel: {
          railwayModelId: modelId,
          productCode: 'R88',
          manufacturer: 'Roco',
          description: 'BR 88',
          category: 'Locomotive',
          railcarType: null,
          serviceLevel: null,
          scale: 'H0',
          epoch: 'IV',
          gauge: 'Standard',
          powerMethod: 'DC'
        },
        metadata: {
          version: 1,
          createdAt: '2026-01-01T00:00:00Z',
          updatedAt: '2026-01-01T00:00:00Z'
        },
        customLabel: null,
        itemCondition: null,
        purchase: null,
        ownershipStatus: 'Owned',
        rollingStocks: []
      }
    ] as unknown as CollectionItemView[];

    mockCommands.searchRailwayModels.mockResolvedValue({ status: 'ok', data: [modelId] });
    mockCommands.getRailwayModelById.mockResolvedValue({
      status: 'ok',
      data: makeModelView({ id: modelId, description: 'BR 88', productCode: 'R88' })
    });

    const assignSpy = vi.spyOn(window.location, 'assign').mockImplementation(() => undefined);

    render(SearchBar);

    const input = screen.getByPlaceholderText('app_search_placeholder');
    await fireEvent.input(input, { target: { value: 'br 88' } });

    vi.advanceTimersByTime(300);

    const resultButton = await screen.findByRole('button', { name: /br 88/i });
    await fireEvent.click(resultButton);

    expect(assignSpy).toHaveBeenCalledWith('/collection/col-55');
    expect(input).toHaveValue('');

    assignSpy.mockRestore();
  });
});
