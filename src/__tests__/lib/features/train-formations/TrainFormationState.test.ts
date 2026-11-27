import { beforeEach, describe, expect, it, vi } from 'vitest';
import { TrainFormationState } from '$lib/features/train-formations/TrainFormationState.svelte';
import { makeCategory, makeDetail, makeElement, makePrototypeGroup, makeSummary } from './fixtures';

const mockedToaster = vi.hoisted(() => ({
  error: vi.fn(),
  success: vi.fn()
}));

vi.mock('$lib/toaster', () => ({ toaster: mockedToaster }));

vi.mock('$lib/paraglide/messages.js', () => ({
  formations_element_removed: () => 'Element removed'
}));

vi.mock('$lib/features/train-formations/services/formations.service.js', () => ({
  getTrainFormations: vi.fn(),
  getTrainFormation: vi.fn(),
  createTrainFormation: vi.fn(),
  updateTrainFormation: vi.fn(),
  deleteTrainFormation: vi.fn(),
  addFormationElement: vi.fn(),
  removeFormationElement: vi.fn(),
  reorderFormationElements: vi.fn(),
  getFormationCategories: vi.fn(),
  createFormationCategory: vi.fn(),
  getPrototypes: vi.fn(),
  createCustomPrototype: vi.fn(),
  assignRollingStockToElement: vi.fn(),
  setTractionOverride: vi.fn()
}));

import * as svc from '$lib/features/train-formations/services/formations.service.js';

describe('TrainFormationState', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('starts with empty state', () => {
    const state = new TrainFormationState();

    expect(state.summaries).toEqual([]);
    expect(state.detail).toBeNull();
    expect(state.categories).toEqual([]);
    expect(state.prototypeGroups).toEqual([]);
    expect(state.hasTraction).toBe(false);
  });

  it('loads summaries on success', async () => {
    vi.mocked(svc.getTrainFormations).mockResolvedValueOnce({
      ok: true,
      data: [makeSummary({ id: 'formation-1' })]
    });

    const state = new TrainFormationState();
    await state.load();

    expect(state.summaries).toHaveLength(1);
    expect(state.summaries[0].id).toBe('formation-1');
  });

  it('shows a toast when loading summaries fails', async () => {
    vi.mocked(svc.getTrainFormations).mockResolvedValueOnce({
      ok: false,
      error: { kind: 'unknown', message: 'Load failed' }
    });

    const state = new TrainFormationState();
    await state.load();

    expect(mockedToaster.error).toHaveBeenCalledWith('Load failed');
  });

  it('loads detail and updates hasTraction', async () => {
    vi.mocked(svc.getTrainFormation).mockResolvedValueOnce({
      ok: true,
      data: makeDetail({
        elements: [makeElement({ id: 'loco-1' })],
        has_traction: true
      })
    });

    const state = new TrainFormationState();
    await state.loadDetail('trn:formation:1');

    expect(state.detail?.id).toBe('trn:formation:1');
    expect(state.hasTraction).toBe(true);
  });

  it('deletes a formation optimistically on success', async () => {
    vi.mocked(svc.deleteTrainFormation).mockResolvedValueOnce({ ok: true, data: null });
    vi.mocked(svc.getTrainFormations).mockResolvedValueOnce({
      ok: true,
      data: [makeSummary({ id: 'remove-me' })]
    });
    vi.mocked(svc.getTrainFormation).mockResolvedValueOnce({
      ok: true,
      data: makeDetail({ id: 'remove-me' })
    });

    const state = new TrainFormationState();
    await state.load();
    await state.loadDetail('remove-me');
    const result = await state.delete('remove-me');

    expect(result).toBe(true);
    expect(state.summaries).toEqual([]);
    expect(state.detail).toBeNull();
  });

  it('reverts optimistic reorder changes when persistence fails', async () => {
    const originalDetail = makeDetail({
      id: 'formation-1',
      elements: [makeElement({ id: 'first' }), makeElement({ id: 'second', position_order: 1 })]
    });
    const reordered = [originalDetail.elements[1], originalDetail.elements[0]];

    vi.mocked(svc.getTrainFormation)
      .mockResolvedValueOnce({ ok: true, data: originalDetail })
      .mockResolvedValueOnce({ ok: true, data: originalDetail });
    vi.mocked(svc.reorderFormationElements).mockResolvedValueOnce({
      ok: false,
      error: { kind: 'unknown', message: 'Save failed' }
    });

    const state = new TrainFormationState();
    await state.loadDetail('formation-1');
    await state.reorderElements('formation-1', reordered);

    expect(state.detail?.elements.map((element) => element.id)).toEqual(['first', 'second']);
    expect(mockedToaster.error).toHaveBeenCalledWith('Save failed');
  });

  it('updates the matching element when traction override changes', async () => {
    const original = makeElement({ id: 'el-1', traction_override: 0 });
    const updated = makeElement({ id: 'el-1', traction_override: 1, is_traction_slot: true });

    vi.mocked(svc.getTrainFormation).mockResolvedValueOnce({
      ok: true,
      data: makeDetail({ elements: [original] })
    });
    vi.mocked(svc.setTractionOverride).mockResolvedValueOnce({ ok: true, data: updated });

    const state = new TrainFormationState();
    await state.loadDetail('trn:formation:1');
    await state.setTractionOverride('el-1', 1);

    expect(state.detail?.elements[0].traction_override).toBe(1);
  });

  it('appends a created category to local state', async () => {
    vi.mocked(svc.createFormationCategory).mockResolvedValueOnce({
      ok: true,
      data: makeCategory({ id: 'custom', name: 'Regional', is_custom: true })
    });
    vi.mocked(svc.getPrototypes).mockResolvedValueOnce({
      ok: true,
      data: [makePrototypeGroup()]
    });

    const state = new TrainFormationState();
    const category = await state.createCategory('Regional');
    await state.searchPrototypes('re');

    expect(category?.name).toBe('Regional');
    expect(state.categories).toHaveLength(1);
    expect(state.prototypeGroups).toHaveLength(1);
  });

  it('creates a formation and reloads summaries', async () => {
    vi.mocked(svc.createTrainFormation).mockResolvedValueOnce({
      ok: true,
      data: {
        id: 'trn:formation:new',
        name: 'New Formation',
        category: makeCategory(),
        start_year: 1974,
        end_year: null,
        epoch: 'IV',
        notes: null,
        element_count: 0,
        has_traction: false
      }
    });
    vi.mocked(svc.getTrainFormations).mockResolvedValueOnce({
      ok: true,
      data: [makeSummary({ id: 'trn:formation:new', name: 'New Formation' })]
    });
    const state = new TrainFormationState();

    const id = await state.create({
      name: 'New Formation',
      category_id: 'trn:formation-category:eurocity',
      epoch: 'IV',
      start_year: 1974,
      end_year: null,
      notes: null
    });

    expect(id).toBe('trn:formation:new');
    expect(state.summaries).toHaveLength(1);
  });

  it('returns null and toasts when creation fails', async () => {
    vi.mocked(svc.createTrainFormation).mockResolvedValueOnce({
      ok: false,
      error: { kind: 'validation', message: 'Name is required' }
    });
    const state = new TrainFormationState();

    await expect(
      state.create({
        name: '',
        category_id: 'trn:formation-category:eurocity',
        epoch: 'IV',
        start_year: null,
        end_year: null,
        notes: null
      })
    ).resolves.toBeNull();
    expect(mockedToaster.error).toHaveBeenCalledWith('Name is required');
  });

  it('updates a formation and reloads detail when it is active', async () => {
    vi.mocked(svc.getTrainFormation).mockResolvedValue({
      ok: true,
      data: makeDetail({ id: 'formation-1', name: 'Updated Formation' })
    });
    vi.mocked(svc.updateTrainFormation).mockResolvedValueOnce({
      ok: true,
      data: {
        id: 'formation-1',
        name: 'Updated Formation',
        category: makeCategory(),
        start_year: 1974,
        end_year: null,
        epoch: 'IV',
        notes: null,
        element_count: 1,
        has_traction: true
      }
    });
    vi.mocked(svc.getTrainFormations).mockResolvedValueOnce({ ok: true, data: [] });
    const state = new TrainFormationState();
    await state.loadDetail('formation-1');

    const updated = await state.update('formation-1', {
      name: 'Updated Formation',
      category_id: 'trn:formation-category:eurocity',
      epoch: 'IV',
      start_year: 1974,
      end_year: null,
      notes: null
    });

    expect(updated).toBe(true);
    expect(state.detail?.name).toBe('Updated Formation');
  });

  it('adds and removes elements while preserving active detail behavior', async () => {
    vi.mocked(svc.getTrainFormation).mockResolvedValue({
      ok: true,
      data: makeDetail({ id: 'formation-1', elements: [makeElement({ id: 'element-1' })] })
    });
    vi.mocked(svc.addFormationElement).mockResolvedValueOnce({
      ok: true,
      data: makeElement({ id: 'element-2' })
    });
    vi.mocked(svc.removeFormationElement).mockResolvedValueOnce({ ok: true, data: null });
    const state = new TrainFormationState();
    await state.loadDetail('formation-1');

    await expect(
      state.addElement('formation-1', {
        prototype_id: 'trn:prototype:1',
        owned_rolling_stock_id: null
      })
    ).resolves.toBe(true);
    await expect(state.removeElement('element-1')).resolves.toBe(true);

    expect(state.detail?.elements).toEqual([]);
    expect(mockedToaster.success).toHaveBeenCalledWith('Element removed');
  });

  it('reports category load and search failures without corrupting state', async () => {
    vi.mocked(svc.getFormationCategories).mockResolvedValueOnce({
      ok: false,
      error: { kind: 'unknown', message: 'Categories failed' }
    });
    vi.mocked(svc.getPrototypes).mockResolvedValueOnce({
      ok: false,
      error: { kind: 'database', message: 'Search failed' }
    });
    const state = new TrainFormationState();

    await state.loadCategories();
    await state.searchPrototypes('unused');

    expect(state.categories).toEqual([]);
    expect(state.prototypeGroups).toEqual([]);
  });

  it('assigns an element and quick-assigns with a null owned stock', async () => {
    const original = makeElement({ id: 'element-1', owned_rolling_stock_id: null });
    const assigned = makeElement({
      id: 'element-1',
      owned_rolling_stock_id: 'trn:owned-rolling-stock:1'
    });
    vi.mocked(svc.getTrainFormation).mockResolvedValueOnce({
      ok: true,
      data: makeDetail({ id: 'formation-1', elements: [original] })
    });
    vi.mocked(svc.assignRollingStockToElement).mockResolvedValue({
      ok: true,
      data: assigned
    });
    const state = new TrainFormationState();
    await state.loadDetail('formation-1');

    await expect(
      state.assignRollingStock('element-1', {
        owned_rolling_stock_id: 'trn:owned-rolling-stock:1'
      })
    ).resolves.toBe(true);
    await state.quickAssign('element-1');

    expect(state.detail?.elements[0].owned_rolling_stock_id).toBe('trn:owned-rolling-stock:1');
    expect(vi.mocked(svc.assignRollingStockToElement)).toHaveBeenLastCalledWith('element-1', {
      owned_rolling_stock_id: null
    });
  });
});
