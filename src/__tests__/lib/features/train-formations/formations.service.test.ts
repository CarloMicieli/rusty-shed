import { beforeEach, describe, expect, it, vi } from 'vitest';

const { commandMocks } = vi.hoisted(() => ({
  commandMocks: {
    getTrainFormations: vi.fn(),
    getTrainFormation: vi.fn(),
    createTrainFormation: vi.fn(),
    updateTrainFormation: vi.fn(),
    deleteTrainFormation: vi.fn(),
    addFormationElement: vi.fn(),
    removeFormationElement: vi.fn(),
    reorderFormationElements: vi.fn(),
    assignRollingStockToElement: vi.fn(),
    setTractionOverride: vi.fn(),
    getPrototypes: vi.fn(),
    createCustomPrototype: vi.fn(),
    getFormationCategories: vi.fn(),
    createFormationCategory: vi.fn()
  }
}));

vi.mock('$lib/bindings', () => ({
  commands: commandMocks
}));

import {
  addFormationElement,
  assignRollingStockToElement,
  createCustomPrototype,
  createFormationCategory,
  createTrainFormation,
  deleteTrainFormation,
  getFormationCategories,
  getPrototypes,
  getTrainFormation,
  getTrainFormations,
  removeFormationElement,
  reorderFormationElements,
  setTractionOverride,
  updateTrainFormation
} from '$lib/features/train-formations/services/formations.service';

describe('formations.service', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('returns ok result for successful train formation calls', async () => {
    commandMocks.getTrainFormations.mockResolvedValueOnce([{ id: 'f-1' }]);
    commandMocks.getTrainFormation.mockResolvedValueOnce({ id: 'f-1' });
    commandMocks.createTrainFormation.mockResolvedValueOnce({ id: 'f-new' });
    commandMocks.updateTrainFormation.mockResolvedValueOnce({ id: 'f-1', name: 'Updated' });
    commandMocks.deleteTrainFormation.mockResolvedValueOnce(null);

    expect(await getTrainFormations()).toEqual({ ok: true, data: [{ id: 'f-1' }] });
    expect(await getTrainFormation('f-1')).toEqual({ ok: true, data: { id: 'f-1' } });
    expect(await createTrainFormation({} as never)).toEqual({ ok: true, data: { id: 'f-new' } });
    expect(await updateTrainFormation('f-1', {} as never)).toEqual({
      ok: true,
      data: { id: 'f-1', name: 'Updated' }
    });
    expect(await deleteTrainFormation('f-1')).toEqual({ ok: true, data: null });
  });

  it('returns ok result for successful formation element and prototype/category calls', async () => {
    commandMocks.addFormationElement.mockResolvedValueOnce({ id: 'el-1' });
    commandMocks.removeFormationElement.mockResolvedValueOnce(null);
    commandMocks.reorderFormationElements.mockResolvedValueOnce({ id: 'f-1', elements: [] });
    commandMocks.assignRollingStockToElement.mockResolvedValueOnce({ id: 'el-2' });
    commandMocks.setTractionOverride.mockResolvedValueOnce({ id: 'el-3' });
    commandMocks.getPrototypes.mockResolvedValueOnce([{ key: 'group' }]);
    commandMocks.createCustomPrototype.mockResolvedValueOnce({ id: 'p-1' });
    commandMocks.getFormationCategories.mockResolvedValueOnce([{ id: 'cat-1' }]);
    commandMocks.createFormationCategory.mockResolvedValueOnce({ id: 'cat-2' });

    expect(await addFormationElement('f-1', {} as never)).toEqual({
      ok: true,
      data: { id: 'el-1' }
    });
    expect(await removeFormationElement('el-1')).toEqual({ ok: true, data: null });
    expect(await reorderFormationElements('f-1', {} as never)).toEqual({
      ok: true,
      data: { id: 'f-1', elements: [] }
    });
    expect(await assignRollingStockToElement('el-2', {} as never)).toEqual({
      ok: true,
      data: { id: 'el-2' }
    });
    expect(await setTractionOverride('el-3', {} as never)).toEqual({
      ok: true,
      data: { id: 'el-3' }
    });
    expect(await getPrototypes('steam')).toEqual({ ok: true, data: [{ key: 'group' }] });
    expect(await createCustomPrototype({} as never)).toEqual({ ok: true, data: { id: 'p-1' } });
    expect(await getFormationCategories()).toEqual({ ok: true, data: [{ id: 'cat-1' }] });
    expect(await createFormationCategory({} as never)).toEqual({ ok: true, data: { id: 'cat-2' } });
  });

  it('maps all command error variants into normalized safe errors', async () => {
    commandMocks.getTrainFormations.mockRejectedValueOnce({
      DatabaseError: { message: 'db down' }
    });
    commandMocks.getTrainFormation.mockRejectedValueOnce({
      NotFound: { message: 'missing formation' }
    });
    commandMocks.createTrainFormation.mockRejectedValueOnce({
      ValidationError: { name: 'required' }
    });
    commandMocks.updateTrainFormation.mockRejectedValueOnce({
      PermissionDenied: { message: 'forbidden' }
    });
    commandMocks.deleteTrainFormation.mockRejectedValueOnce({
      Conflict: { message: 'already used' }
    });
    commandMocks.addFormationElement.mockRejectedValueOnce({
      BusinessRule: { message: 'cannot add twice' }
    });
    commandMocks.removeFormationElement.mockRejectedValueOnce({
      Unknown: { message: 'unexpected backend error' }
    });

    expect(await getTrainFormations()).toEqual({
      ok: false,
      error: { kind: 'database', message: 'db down' }
    });
    expect(await getTrainFormation('missing')).toEqual({
      ok: false,
      error: { kind: 'not_found', message: 'missing formation' }
    });
    expect(await createTrainFormation({} as never)).toEqual({
      ok: false,
      error: { kind: 'validation', message: 'Validation failed' }
    });
    expect(await updateTrainFormation('f-1', {} as never)).toEqual({
      ok: false,
      error: { kind: 'permission_denied', message: 'forbidden' }
    });
    expect(await deleteTrainFormation('f-1')).toEqual({
      ok: false,
      error: { kind: 'unknown', message: 'already used' }
    });
    expect(await addFormationElement('f-1', {} as never)).toEqual({
      ok: false,
      error: { kind: 'unknown', message: 'cannot add twice' }
    });
    expect(await removeFormationElement('el-1')).toEqual({
      ok: false,
      error: { kind: 'unknown', message: 'unexpected backend error' }
    });
  });

  it('handles non-object thrown values and unknown object shapes', async () => {
    commandMocks.getPrototypes.mockRejectedValueOnce('string failure');
    commandMocks.createCustomPrototype.mockRejectedValueOnce(new Error('boom'));
    commandMocks.getFormationCategories.mockRejectedValueOnce({
      Unexpected: { message: 'ignored shape' }
    });

    expect(await getPrototypes(null)).toEqual({
      ok: false,
      error: { kind: 'unknown', message: 'string failure' }
    });
    expect(await createCustomPrototype({} as never)).toEqual({
      ok: false,
      error: { kind: 'unknown', message: 'Unknown error' }
    });
    expect(await getFormationCategories()).toEqual({
      ok: false,
      error: { kind: 'unknown', message: 'Unknown error' }
    });
  });
});
