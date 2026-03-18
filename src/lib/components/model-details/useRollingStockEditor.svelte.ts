import type { RailwayModel } from '$lib/types/railway-model';
import type { RollingStockView } from '$lib/bindings';
import { commands } from '$lib/bindings';
import { getLocale } from '$lib/paraglide/runtime.js';
import { SvelteMap, SvelteSet } from 'svelte/reactivity';

export interface RsFormState {
  seriesCode: string;
  roadNumber: string;
  livery: string;
  depot: string;
  flywheelFitted: boolean | null;
  bodyShell: string;
  chassis: string;
  interiorLights: string;
  lights: string;
  dccInterface: string;
  control: string;
  couplingSocket: string;
  closeCouplers: boolean | null;
  digitalShunting: boolean | null;
}

function getEmptyRsForm(): RsFormState {
  return {
    seriesCode: '',
    roadNumber: '',
    livery: '',
    depot: '',
    flywheelFitted: null,
    bodyShell: '',
    chassis: '',
    interiorLights: '',
    lights: '',
    dccInterface: '',
    control: '',
    couplingSocket: '',
    closeCouplers: null,
    digitalShunting: null
  };
}

function extractRsDataFromView(view: RollingStockView): RsFormState {
  let rs;
  if ('locomotive' in view) rs = view.locomotive;
  else if ('electricMultipleUnit' in view) rs = view.electricMultipleUnit;
  else if ('freightCar' in view) rs = view.freightCar;
  else if ('passengerCar' in view) rs = view.passengerCar;
  else if ('railcar' in view) rs = view.railcar;
  else return getEmptyRsForm();

  const ts = rs.technical_specifications;
  return {
    seriesCode: rs.series_code,
    roadNumber: rs.road_number ?? '',
    livery: rs.livery ?? '',
    depot: 'depot' in rs ? (rs.depot ?? '') : '',
    flywheelFitted:
      ts?.flywheel_fitted === 'YES' ? true : ts?.flywheel_fitted === 'NO' ? false : null,
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
          : null
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
        if ('locomotive' in r) return r.locomotive.id === unitId;
        if ('electricMultipleUnit' in r) return r.electricMultipleUnit.id === unitId;
        if ('freightCar' in r) return r.freightCar.id === unitId;
        if ('passengerCar' in r) return r.passengerCar.id === unitId;
        if ('railcar' in r) return r.railcar.id === unitId;
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
      roadNumber: form.roadNumber || null,
      livery: form.livery || null,
      depot: form.depot || null,
      flywheelFitted: form.flywheelFitted,
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
      digitalShunting: form.digitalShunting
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
      roadNumber: form.roadNumber || null,
      livery: form.livery || null,
      depot: form.depot || null,
      flywheelFitted: form.flywheelFitted,
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
      digitalShunting: form.digitalShunting
    });

    if (result.status === 'error') throw new Error('Failed to save');

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
    saveBoolSpec
  };
}
