import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/svelte';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import AddStockDrawer from '$lib/features/train-formations/components/AddStockDrawer.svelte';
import type { TrainFormationState } from '$lib/features/train-formations/TrainFormationState.svelte';
import { makePrototype, makePrototypeGroup } from '../fixtures';

vi.mock('$lib/paraglide/messages.js', () => ({
  formations_add_stock: () => 'Add stock',
  formations_search_placeholder: () => 'Search prototypes',
  formations_add_prototype_action: () => 'Add prototype',
  formations_prototype_series_code: () => 'Series code',
  formations_prototype_specification_type: () => 'Specification type',
  formations_prototype_service_level: () => 'Service level',
  formations_save: () => 'Save',
  formations_cancel: () => 'Cancel'
}));

function makeState(overrides: Partial<Record<string, unknown>> = {}): TrainFormationState {
  return {
    prototypeGroups: [makePrototypeGroup()],
    isPrototypesLoading: false,
    searchPrototypes: vi.fn().mockResolvedValue(undefined),
    addElement: vi.fn().mockResolvedValue(true),
    createCustomPrototype: vi.fn().mockResolvedValue(makePrototype({ id: 'custom-proto' })),
    ...overrides
  } as unknown as TrainFormationState;
}

describe('AddStockDrawer.svelte', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it('renders the drawer title and search input when open', () => {
    render(AddStockDrawer, {
      props: {
        state: makeState(),
        formationId: 'formation-1',
        open: true
      }
    });

    expect(screen.getByText('Add stock')).toBeInTheDocument();
    expect(screen.getByPlaceholderText('Search prototypes')).toBeInTheDocument();
  });

  it('debounces prototype searches on mount', async () => {
    const state = makeState();
    render(AddStockDrawer, {
      props: { state, formationId: 'formation-1', open: true }
    });

    await vi.advanceTimersByTimeAsync(200);
    expect(state.searchPrototypes).toHaveBeenCalledWith('');
  });

  it('debounces prototype searches when the query changes', async () => {
    const state = makeState();
    render(AddStockDrawer, {
      props: { state, formationId: 'formation-1', open: true }
    });

    await fireEvent.input(screen.getByPlaceholderText('Search prototypes'), {
      target: { value: 'Re 4/4' }
    });

    await vi.advanceTimersByTimeAsync(200);
    expect(state.searchPrototypes).toHaveBeenLastCalledWith('Re 4/4');
  });

  it('adds a selected prototype to the active formation', async () => {
    const state = makeState({
      prototypeGroups: [
        makePrototypeGroup({
          prototypes: [makePrototype({ id: 'proto-1', series_code: 'Re 4/4 II' })]
        })
      ]
    });
    render(AddStockDrawer, {
      props: { state, formationId: 'formation-1', open: true }
    });

    await fireEvent.click(screen.getByText('Re 4/4 II'));

    expect(state.addElement).toHaveBeenCalledWith('formation-1', {
      prototype_id: 'proto-1',
      owned_rolling_stock_id: null
    });
  });

  it('shows the add prototype action when results are idle', () => {
    render(AddStockDrawer, {
      props: {
        state: makeState(),
        formationId: 'formation-1',
        open: true
      }
    });

    expect(screen.getByRole('button', { name: 'Add prototype' })).toBeInTheDocument();
  });

  it('hides the add prototype action while prototypes are loading', () => {
    render(AddStockDrawer, {
      props: {
        state: makeState({ isPrototypesLoading: true }),
        formationId: 'formation-1',
        open: true
      }
    });

    expect(screen.queryByRole('button', { name: 'Add prototype' })).toBeNull();
  });

  it('opens the inline prototype form when requested', async () => {
    render(AddStockDrawer, {
      props: {
        state: makeState(),
        formationId: 'formation-1',
        open: true
      }
    });

    await fireEvent.click(screen.getByRole('button', { name: 'Add prototype' }));

    expect(screen.getByLabelText('Series code')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Save' })).toBeInTheDocument();
  });

  it('creates a custom prototype and adds it to the formation', async () => {
    const state = makeState();
    render(AddStockDrawer, {
      props: {
        state,
        formationId: 'formation-1',
        open: true
      }
    });

    await fireEvent.click(screen.getByRole('button', { name: 'Add prototype' }));
    await fireEvent.input(screen.getByLabelText('Series code'), {
      target: { value: 'EW IV Bistro' }
    });
    await fireEvent.submit(
      screen.getByRole('button', { name: 'Save' }).closest('form') as HTMLFormElement
    );

    await waitFor(() => {
      expect(state.createCustomPrototype).toHaveBeenCalledWith(
        expect.objectContaining({
          series_code: 'EW IV Bistro',
          specification_type: 'PASSENGER_CAR',
          friendly_name: null,
          locomotive_type: null,
          passenger_car_type: null,
          freight_car_type: null
        })
      );
      expect(state.addElement).toHaveBeenCalledWith('formation-1', {
        prototype_id: 'custom-proto',
        owned_rolling_stock_id: null
      });
    });
  });

  it('returns to the search results when the inline form is cancelled', async () => {
    render(AddStockDrawer, {
      props: {
        state: makeState(),
        formationId: 'formation-1',
        open: true
      }
    });

    await fireEvent.click(screen.getByRole('button', { name: 'Add prototype' }));
    await fireEvent.click(screen.getByRole('button', { name: 'Cancel' }));

    expect(screen.getByRole('button', { name: 'Add prototype' })).toBeInTheDocument();
  });
});
