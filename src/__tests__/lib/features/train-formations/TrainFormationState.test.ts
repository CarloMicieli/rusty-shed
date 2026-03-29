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
});
