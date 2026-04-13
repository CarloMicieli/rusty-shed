import type { RailwayModel } from '$lib/types/railway-model';
import type { RollingStockView } from '$lib/bindings';
import { commands } from '$lib/bindings';
import { getLocale } from '$lib/paraglide/runtime.js';
import { SvelteMap, SvelteSet } from 'svelte/reactivity';
import type { RollingStockUnitSpecsFormState } from './components/rolling-stock-unit-specs-form-state';

export interface RsFormState extends RollingStockUnitSpecsFormState {
  series: string;
  friendlyName: string;
  flywheelFitted: boolean | null;
  sprungBuffers: boolean | null;
  bodyShell: string;
  chassis: string;
  interiorLights: string;
  lights: string;
  isDummy: boolean | null;
}

function getEmptyRsForm(): RsFormState {
  return {
    seriesCode: '',
    series: '',
    roadNumber: '',
    friendlyName: '',
    livery: '',
    depot: '',
    flywheelFitted: null,
    sprungBuffers: null,
    bodyShell: '',
    chassis: '',
    interiorLights: '',
    lights: '',
    dccInterface: '',
    control: '',
    couplingSocket: '',
    closeCouplers: null,
    digitalShunting: null,
    category: null,
    subcategory: null,
    serviceLevel: null,
    subcategoryFlashed: false,
    isDummy: null
  };
}

function extractRsDataFromView(view: RollingStockView): RsFormState {
  const rs =
    ('locomotive' in view && view.locomotive) ||
    ('electricMultipleUnit' in view && view.electricMultipleUnit) ||
    ('freightCar' in view && view.freightCar) ||
    ('passengerCar' in view && view.passengerCar) ||
    ('railcar' in view && view.railcar) ||
    null;

  if (!rs) return getEmptyRsForm();

  const ts = rs.technical_specifications;

  let category: string | null = null;
  let subcategory: string | null = null;
  let serviceLevel: string | null = null;

  if ('locomotive' in view && view.locomotive) {
    category = 'LOCOMOTIVE';
    subcategory = view.locomotive.locomotive_type ?? null;
  } else if ('electricMultipleUnit' in view && view.electricMultipleUnit) {
    category = 'ELECTRIC_MULTIPLE_UNIT';
    subcategory = view.electricMultipleUnit.electric_multiple_unit_type ?? null;
  } else if ('freightCar' in view && view.freightCar) {
    category = 'FREIGHT_CAR';
    subcategory = view.freightCar.freight_car_type ?? null;
  } else if ('passengerCar' in view && view.passengerCar) {
    category = 'PASSENGER_CAR';
    subcategory = view.passengerCar.passenger_car_type ?? null;
    serviceLevel = view.passengerCar.service_level ?? null;
  } else if ('railcar' in view && view.railcar) {
    category = 'RAILCAR';
    subcategory = view.railcar.railcar_type ?? null;
  }

  return {
    seriesCode: rs.series_code,
    series: 'series' in rs ? (rs.series ?? '') : '',
    roadNumber: rs.road_number ?? '',
    friendlyName: rs.friendly_name ?? '',
    livery: rs.livery ?? '',
    depot: 'depot' in rs ? (rs.depot ?? '') : '',
    flywheelFitted:
      ts?.flywheel_fitted === 'YES' ? true : ts?.flywheel_fitted === 'NO' ? false : null,
    sprungBuffers: ts?.sprung_buffers === 'YES' ? true : ts?.sprung_buffers === 'NO' ? false : null,
    bodyShell: ts?.body_shell ?? '',
    chassis: ts?.chassis ?? '',
    interiorLights: ts?.interior_lights ?? '',
    lights: ts?.lights ?? '',
    dccInterface: 'dcc_interface' in rs ? (rs.dcc_interface ?? '') : '',
    control: 'control' in rs ? (rs.control ?? '') : '',
    couplingSocket: ts?.coupling?.socket ?? '',
    closeCouplers:
      ts?.coupling?.close_couplers === 'YES'
        ? true
        : ts?.coupling?.close_couplers === 'NO'
          ? false
          : null,
    digitalShunting:
      ts?.coupling?.digital_shunting === 'YES'
        ? true
        : ts?.coupling?.digital_shunting === 'NO'
          ? false
          : null,
    category,
    subcategory,
    serviceLevel,
    subcategoryFlashed: false,
    isDummy: 'is_dummy' in rs ? (rs.is_dummy as boolean) : null
  };
}

/**
 * Runes-based controller for rolling stock editing state.
 * All reactive state lives here; the parent component simply reads the
 * exported Map/Set references and calls the async save functions.
 */
export function useRollingStockEditor(
  getModel: () => RailwayModel,
  onModelUpdated: () => Promise<void> | void
) {
  const formState = new SvelteMap<string, RsFormState>();
  const specLoaded = new SvelteSet<string>();

  async function reloadSpec(unitId: string) {
    specLoaded.delete(unitId);
    formState.delete(unitId);
    await loadSpec(unitId);
  }

  async function loadSpec(unitId: string) {
    if (specLoaded.has(unitId)) return;

    const model = getModel();
    try {
      const result = await commands.getRailwayModelById(model.id, getLocale());
      if (result.status === 'error' || !result.data) {
        formState.set(unitId, getEmptyRsForm());
        specLoaded.add(unitId);
        return;
      }

      const rsView = result.data.rollingStock.find((r) => {
        if ('locomotive' in r && r.locomotive) return r.locomotive.id === unitId;
        if ('electricMultipleUnit' in r && r.electricMultipleUnit)
          return r.electricMultipleUnit.id === unitId;
        if ('freightCar' in r && r.freightCar) return r.freightCar.id === unitId;
        if ('passengerCar' in r && r.passengerCar) return r.passengerCar.id === unitId;
        if ('railcar' in r && r.railcar) return r.railcar.id === unitId;
        return false;
      });

      formState.set(unitId, rsView ? extractRsDataFromView(rsView) : getEmptyRsForm());
      specLoaded.add(unitId);
    } catch {
      formState.set(unitId, getEmptyRsForm());
      specLoaded.add(unitId);
    }
  }

  async function saveIdentification(
    unitId: string,
    field: 'series' | 'roadNumber' | 'livery' | 'depot',
    value: string,
    unit: RailwayModel['rolling_stock'][0]
  ) {
    const model = getModel();
    const currentForm = formState.get(unitId) ?? getEmptyRsForm();
    const seriesCode = field === 'series' ? value : currentForm.seriesCode || unit.series_code;
    const roadNumber =
      field === 'roadNumber' ? value || null : currentForm.roadNumber || unit.road_number || null;
    const livery = field === 'livery' ? value || null : currentForm.livery || unit.livery || null;
    const depot = field === 'depot' ? value || null : currentForm.depot || unit.depot || null;

    const result = await commands.updateRollingStockIdentification({
      railwayModelId: model.id,
      rollingStockId: unitId,
      seriesCode,
      roadNumber,
      livery,
      depot
    });

    if (result.status === 'error') throw new Error('Failed to save');

    const form = formState.get(unitId) ?? getEmptyRsForm();
    if (!formState.has(unitId)) formState.set(unitId, form);
    form.seriesCode = seriesCode;
    form.roadNumber = roadNumber ?? '';
    form.livery = livery ?? '';
    form.depot = depot ?? '';

    await onModelUpdated();
  }

  async function saveLength(unitId: string, rawValue: string) {
    const model = getModel();
    const form = formState.get(unitId);
    const trimmed = rawValue.trim();
    const lengthMm = trimmed ? parseFloat(trimmed) : null;

    const result = await commands.updateRollingStockDcc({
      railwayModelId: model.id,
      rollingStockId: unitId,
      control: (form?.control || null) as Parameters<
        typeof commands.updateRollingStockDcc
      >[0]['control'],
      dccInterface: (form?.dccInterface || null) as Parameters<
        typeof commands.updateRollingStockDcc
      >[0]['dccInterface'],
      lengthMillimeters: Number.isFinite(lengthMm) ? lengthMm : null,
      lengthInches: null
    });

    if (result.status === 'error') throw new Error('Failed to save length');

    await onModelUpdated();
  }

  async function saveBoolSpec(
    unitId: string,
    field: 'closeCouplers' | 'digitalShunting',
    value: boolean | null
  ) {
    const model = getModel();
    const form = formState.get(unitId);
    if (!form) return;

    form[field] = value;

    const result = await commands.updateRollingStockSpecifications({
      railwayModelId: model.id,
      rollingStockId: unitId,
      seriesCode: form.seriesCode,
      series: form.series || null,
      roadNumber: form.roadNumber || null,
      friendlyName: form.friendlyName || null,
      livery: form.livery || null,
      depot: form.depot || null,
      flywheelFitted: form.flywheelFitted,
      sprungBuffers: form.sprungBuffers,
      bodyShell: form.bodyShell || null,
      chassis: form.chassis || null,
      interiorLights: form.interiorLights || null,
      lights: form.lights || null,
      dccInterface: (form.dccInterface || null) as Parameters<
        typeof commands.updateRollingStockSpecifications
      >[0]['dccInterface'],
      control: (form.control || null) as Parameters<
        typeof commands.updateRollingStockSpecifications
      >[0]['control'],
      couplingSocket: form.couplingSocket || null,
      closeCouplers: form.closeCouplers,
      digitalShunting: form.digitalShunting,
      isDummy: form.isDummy
    });

    if (result.status === 'error') throw new Error('Failed to save');

    await onModelUpdated();
  }

  async function saveSpec(unitId: string, field: string, value: string) {
    const model = getModel();
    const form = formState.get(unitId);
    if (!form) return;

    (form as unknown as Record<string, string | boolean | null>)[field] = value;

    const result = await commands.updateRollingStockSpecifications({
      railwayModelId: model.id,
      rollingStockId: unitId,
      seriesCode: form.seriesCode,
      series: form.series || null,
      roadNumber: form.roadNumber || null,
      friendlyName: form.friendlyName || null,
      livery: form.livery || null,
      depot: form.depot || null,
      flywheelFitted: form.flywheelFitted,
      sprungBuffers: form.sprungBuffers,
      bodyShell: form.bodyShell || null,
      chassis: form.chassis || null,
      interiorLights: form.interiorLights || null,
      lights: form.lights || null,
      dccInterface: (form.dccInterface || null) as Parameters<
        typeof commands.updateRollingStockSpecifications
      >[0]['dccInterface'],
      control: (form.control || null) as Parameters<
        typeof commands.updateRollingStockSpecifications
      >[0]['control'],
      couplingSocket: form.couplingSocket || null,
      closeCouplers: form.closeCouplers,
      digitalShunting: form.digitalShunting,
      isDummy: form.isDummy
    });

    if (result.status === 'error') throw new Error('Failed to save');

    await onModelUpdated();
  }

  async function saveCategory(unitId: string, newCategory: string) {
    const model = getModel();
    const result = await commands.updateRollingStockCategory({
      railwayModelId: model.id,
      rollingStockId: unitId,
      category: newCategory as Parameters<typeof commands.updateRollingStockCategory>[0]['category']
    });
    if (result.status === 'error') throw new Error('Failed to save category');

    const form = formState.get(unitId) ?? getEmptyRsForm();
    if (!formState.has(unitId)) formState.set(unitId, form);
    form.category = newCategory;
    form.subcategory = null;
    form.serviceLevel = null;
    form.subcategoryFlashed = true;
    setTimeout(() => {
      const f = formState.get(unitId);
      if (f) f.subcategoryFlashed = false;
    }, 800);

    await onModelUpdated();
  }

  async function saveSubcategory(unitId: string, subcategory: string) {
    const model = getModel();
    const result = await commands.updateRollingStockSubcategory({
      railwayModelId: model.id,
      rollingStockId: unitId,
      subcategory
    });
    if (result.status === 'error') throw new Error('Failed to save subcategory');

    const form = formState.get(unitId);
    if (form) form.subcategory = subcategory;

    await onModelUpdated();
  }

  async function saveServiceLevel(unitId: string, serviceLevel: string | null) {
    const model = getModel();
    const result = await commands.updateRollingStockServiceLevel({
      railwayModelId: model.id,
      rollingStockId: unitId,
      serviceLevel: (serviceLevel || null) as Parameters<
        typeof commands.updateRollingStockServiceLevel
      >[0]['serviceLevel']
    });
    if (result.status === 'error') throw new Error('Failed to save service level');

    const form = formState.get(unitId);
    if (form) form.serviceLevel = serviceLevel || null;

    await onModelUpdated();
  }

  async function deleteUnit(unitId: string) {
    const model = getModel();
    const result = await commands.deleteRollingStock({
      railwayModelId: model.id,
      rollingStockId: unitId
    });

    if (result.status === 'error') throw new Error('Failed to delete rolling stock');

    await onModelUpdated();
  }

  return {
    formState,
    specLoaded,
    loadSpec,
    reloadSpec,
    saveIdentification,
    saveSpec,
    saveLength,
    saveBoolSpec,
    saveCategory,
    saveSubcategory,
    saveServiceLevel,
    deleteUnit
  };
}
