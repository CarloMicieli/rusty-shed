import { beforeEach, describe, expect, it, vi } from 'vitest';

const commandMocks = vi.hoisted(() => ({
  getRailwayModelById: vi.fn(),
  updateRollingStockIdentification: vi.fn(),
  updateRollingStockDcc: vi.fn(),
  updateRollingStockSpecifications: vi.fn(),
  updateRollingStockCategory: vi.fn(),
  updateRollingStockSubcategory: vi.fn(),
  updateRollingStockServiceLevel: vi.fn(),
  deleteRollingStock: vi.fn()
}));

vi.mock('$lib/bindings', () => ({ commands: commandMocks }));
vi.mock('$lib/paraglide/runtime.js', () => ({ getLocale: () => 'en' }));

import { useRollingStockEditor } from '$lib/components/model-details/useRollingStockEditor.svelte';
import type { RailwayModel, RollingStock } from '$lib/types/railway-model';

function makeModel(): RailwayModel {
  return {
    id: 'model-1',
    manufacturer: 'Roco',
    product_code: '12345',
    scale: 'H0',
    era: 'VI',
    power_method: 'DC',
    category: 'Locomotive',
    delivery_date: null,
    description: null,
    descriptionLang: 'en',
    details: null,
    detailsLang: null,
    image_path: null,
    status: 'InCollection',
    rolling_stock: [makeRollingStock()]
  };
}

function makeRollingStock(): RollingStock {
  return {
    id: 'unit-1',
    ownedRollingStockId: 'owned-1',
    currentCouplerId: null,
    railway_model_id: 1,
    railway_company: 'FS',
    country_code: 'IT',
    series_code: 'E656',
    series_name: 'E656',
    rolling_stock_type: 'LOCOMOTIVE',
    category: 'LOCOMOTIVE',
    subcategory: 'ELECTRIC',
    road_number: 'E656 001',
    depot: 'Milano',
    livery: 'Green',
    length_mm: 220,
    control_type: 'DCC',
    dcc_interface: 'NEM652',
    coupling_type: null,
    close_couplers: true,
    digital_shunting: false
  };
}

describe('useRollingStockEditor', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('loads and caches a matching locomotive form, including technical specifications', async () => {
    commandMocks.getRailwayModelById.mockResolvedValue({
      rollingStock: [
        {
          locomotive: {
            id: 'unit-1',
            series_code: 'E656',
            series: 'E656',
            road_number: '001',
            friendly_name: 'Caimano',
            livery: 'Green',
            depot: 'Milano',
            is_dummy: false,
            dcc_interface: 'NEM652',
            control: 'DCC',
            locomotive_type: 'ELECTRIC',
            technical_specifications: {
              flywheel_fitted: 'YES',
              sprung_buffers: 'NO',
              body_shell: 'Die cast',
              chassis: 'Metal',
              interior_lights: 'NONE',
              lights: 'Directional',
              coupling: {
                socket: 'NEM652',
                close_couplers: 'YES',
                digital_shunting: 'NO'
              }
            }
          }
        }
      ]
    });
    const onModelUpdated = vi.fn();
    const editor = useRollingStockEditor(makeModel, onModelUpdated);

    await editor.loadSpec('unit-1');
    await editor.loadSpec('unit-1');

    expect(commandMocks.getRailwayModelById).toHaveBeenCalledTimes(1);
    expect(editor.formState.get('unit-1')).toMatchObject({
      category: 'LOCOMOTIVE',
      subcategory: 'ELECTRIC',
      flywheelFitted: true,
      sprungBuffers: false,
      closeCouplers: true,
      digitalShunting: false,
      isDummy: false
    });
  });

  it('uses an empty form when the model or unit is missing', async () => {
    commandMocks.getRailwayModelById.mockResolvedValueOnce(null).mockResolvedValueOnce({
      rollingStock: []
    });
    const editor = useRollingStockEditor(makeModel, vi.fn());

    await editor.loadSpec('missing-1');
    expect(editor.formState.get('missing-1')).toMatchObject({
      seriesCode: '',
      roadNumber: '',
      category: null
    });

    await editor.reloadSpec('missing-1');
    expect(editor.formState.get('missing-1')).toMatchObject({
      seriesCode: '',
      roadNumber: '',
      category: null
    });
  });

  it('falls back to an empty form when loading IPC throws', async () => {
    commandMocks.getRailwayModelById.mockRejectedValue(new Error('Backend unavailable'));
    const editor = useRollingStockEditor(makeModel, vi.fn());

    await editor.loadSpec('unit-1');

    expect(editor.specLoaded.has('unit-1')).toBe(true);
    expect(editor.formState.get('unit-1')).toMatchObject({ seriesCode: '', category: null });
  });

  it('updates identification fields and refreshes the model', async () => {
    commandMocks.updateRollingStockIdentification.mockResolvedValue(undefined);
    const onModelUpdated = vi.fn().mockResolvedValue(undefined);
    const editor = useRollingStockEditor(makeModel, onModelUpdated);

    await editor.saveIdentification('unit-1', 'roadNumber', '002', makeRollingStock());

    expect(commandMocks.updateRollingStockIdentification).toHaveBeenCalledWith({
      railwayModelId: 'model-1',
      rollingStockId: 'unit-1',
      seriesCode: 'E656',
      roadNumber: '002',
      livery: 'Green',
      depot: 'Milano'
    });
    expect(editor.formState.get('unit-1')).toMatchObject({
      roadNumber: '002',
      livery: 'Green',
      depot: 'Milano'
    });
    expect(onModelUpdated).toHaveBeenCalledTimes(1);
  });

  it('normalizes empty or invalid length values to null', async () => {
    commandMocks.updateRollingStockDcc.mockResolvedValue(undefined);
    const onModelUpdated = vi.fn();
    const editor = useRollingStockEditor(makeModel, onModelUpdated);

    await editor.saveLength('unit-1', '  ');
    await editor.saveLength('unit-1', 'not-a-number');

    expect(commandMocks.updateRollingStockDcc).toHaveBeenNthCalledWith(
      1,
      expect.objectContaining({ lengthMillimeters: null, lengthInches: null })
    );
    expect(commandMocks.updateRollingStockDcc).toHaveBeenNthCalledWith(
      2,
      expect.objectContaining({ lengthMillimeters: null, lengthInches: null })
    );
  });

  it('updates boolean specifications and refreshes the model', async () => {
    commandMocks.updateRollingStockSpecifications.mockResolvedValue(undefined);
    const onModelUpdated = vi.fn().mockResolvedValue(undefined);
    const editor = useRollingStockEditor(makeModel, onModelUpdated);
    await editor.loadSpec('unit-1');
    commandMocks.getRailwayModelById.mockResolvedValue(null);

    await editor.saveBoolSpec('unit-1', 'closeCouplers', false);

    expect(editor.formState.get('unit-1')?.closeCouplers).toBe(false);
    expect(commandMocks.updateRollingStockSpecifications).toHaveBeenCalledWith(
      expect.objectContaining({
        railwayModelId: 'model-1',
        rollingStockId: 'unit-1',
        closeCouplers: false
      })
    );
    expect(onModelUpdated).toHaveBeenCalledTimes(1);
  });

  it('resets subcategory and service level when a category changes', async () => {
    vi.useFakeTimers();
    commandMocks.updateRollingStockCategory.mockResolvedValue(undefined);
    const editor = useRollingStockEditor(makeModel, vi.fn());
    await editor.loadSpec('unit-1');

    await editor.saveCategory('unit-1', 'FREIGHT_CAR');

    expect(editor.formState.get('unit-1')).toMatchObject({
      category: 'FREIGHT_CAR',
      subcategory: null,
      serviceLevel: null,
      subcategoryFlashed: true
    });
    vi.advanceTimersByTime(800);
    expect(editor.formState.get('unit-1')?.subcategoryFlashed).toBe(false);
  });

  it('updates subcategory and service level independently', async () => {
    commandMocks.updateRollingStockSubcategory.mockResolvedValue(undefined);
    commandMocks.updateRollingStockServiceLevel.mockResolvedValue(undefined);
    const onModelUpdated = vi.fn();
    const editor = useRollingStockEditor(makeModel, onModelUpdated);
    await editor.loadSpec('unit-1');

    await editor.saveSubcategory('unit-1', 'PASSENGER');
    await editor.saveServiceLevel('unit-1', null);

    expect(editor.formState.get('unit-1')).toMatchObject({
      subcategory: 'PASSENGER',
      serviceLevel: null
    });
    expect(commandMocks.updateRollingStockSubcategory).toHaveBeenCalledWith({
      railwayModelId: 'model-1',
      rollingStockId: 'unit-1',
      subcategory: 'PASSENGER'
    });
    expect(commandMocks.updateRollingStockServiceLevel).toHaveBeenCalledWith({
      railwayModelId: 'model-1',
      rollingStockId: 'unit-1',
      serviceLevel: null
    });
  });

  it('deletes a unit and refreshes the model', async () => {
    commandMocks.deleteRollingStock.mockResolvedValue(undefined);
    const onModelUpdated = vi.fn();
    const editor = useRollingStockEditor(makeModel, onModelUpdated);

    await editor.deleteUnit('unit-1');

    expect(commandMocks.deleteRollingStock).toHaveBeenCalledWith({
      railwayModelId: 'model-1',
      rollingStockId: 'unit-1'
    });
    expect(onModelUpdated).toHaveBeenCalledTimes(1);
  });
});
