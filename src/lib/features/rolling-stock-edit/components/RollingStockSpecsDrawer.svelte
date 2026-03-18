<script lang="ts">
  import { Settings } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages';
  import { getLocale } from '$lib/paraglide/runtime.js';
  import { toaster } from '$lib/toaster';
  import {
    commands,
    type RailwayModelId,
    type RollingStockId,
    type RollingStockView
  } from '$lib/bindings';
  import RollingStockBasicFields from './RollingStockBasicFields.svelte';
  import RollingStockTechnicalFields from './RollingStockTechnicalFields.svelte';
  import { DrawerShell, DrawerHeader, DrawerFooter } from '$lib/components/drawer';

  interface Props {
    /** Controls drawer visibility. */
    open: boolean;
    /** The parent railway model. */
    railwayModelId: RailwayModelId;
    /** The rolling stock unit to edit. */
    rollingStockId: RollingStockId;
    /** Called after a successful save (to trigger parent refresh). */
    onSaved?: () => void;
    /** Called when the drawer requests to close. */
    onClose: () => void;
  }

  let { open, railwayModelId, rollingStockId, onSaved, onClose }: Props = $props();

  // ── Form state ──────────────────────────────────────────────────────────────
  interface FormState {
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

  const emptyForm: FormState = {
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

  let form = $state<FormState>({ ...emptyForm });
  let originalForm = $state<FormState>({ ...emptyForm });
  let isLoading = $state(false);
  let isSaving = $state(false);
  let inlineError = $state<string | null>(null);

  // ── Derived ─────────────────────────────────────────────────────────────────
  const isDirty = $derived(JSON.stringify(form) !== JSON.stringify(originalForm));

  // ── Option lists ────────────────────────────────────────────────────────────
  const bodyShellOptions = [
    { value: '', label: '—' },
    { value: 'PLASTIC', label: 'Plastic' },
    { value: 'METAL_DIE_CAST', label: 'Metal die-cast' }
  ];

  const chassisOptions = [
    { value: '', label: '—' },
    { value: 'PLASTIC', label: 'Plastic' },
    { value: 'METAL_DIE_CAST', label: 'Metal die-cast' }
  ];

  const featureFlagOptions = [
    { value: '', label: '—' },
    { value: 'YES', label: 'Yes' },
    { value: 'NO', label: 'No' }
  ];

  const controlOptions = [
    { value: 'DCC_READY', label: 'DCC Ready' },
    { value: 'DCC_FITTED', label: 'DCC Fitted' },
    { value: 'DCC_SOUND', label: 'DCC Sound' },
    { value: 'NO_DCC', label: 'Analogue (No DCC)' }
  ];

  const dccInterfaceOptions = [
    { value: '', label: '—' },
    { value: 'NEM_651', label: 'NEM 651' },
    { value: 'NEM_652', label: 'NEM 652' },
    { value: 'NEM_654', label: 'NEM 654' },
    { value: 'PLUX_8', label: 'PLUX 8' },
    { value: 'PLUX_12', label: 'PLUX 12' },
    { value: 'PLUX_16', label: 'PLUX 16' },
    { value: 'PLUX_22', label: 'PLUX 22' },
    { value: 'NEXT_18', label: 'Next18' },
    { value: 'NEXT_18_S', label: 'Next18-S' },
    { value: 'MTC_21', label: 'MTC 21' }
  ];

  const couplingSockeOptions = [
    { value: '', label: '—' },
    { value: 'NONE', label: 'None' },
    { value: 'NEM_355', label: 'NEM 355' },
    { value: 'NEM_356', label: 'NEM 356' },
    { value: 'NEM_357', label: 'NEM 357' },
    { value: 'NEM_359', label: 'NEM 359' },
    { value: 'NEM_360', label: 'NEM 360' },
    { value: 'NEM_362', label: 'NEM 362' },
    { value: 'NEM_365', label: 'NEM 365' }
  ];

  // ── Data loading ─────────────────────────────────────────────────────────────
  function extractRsData(view: RollingStockView): Omit<FormState, never> {
    let rs;
    if ('locomotive' in view) rs = view.locomotive;
    else if ('electricMultipleUnit' in view) rs = view.electricMultipleUnit;
    else if ('freightCar' in view) rs = view.freightCar;
    else if ('passengerCar' in view) rs = view.passengerCar;
    else if ('railcar' in view) rs = view.railcar;
    else return { ...emptyForm };

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

  $effect(() => {
    if (!open) return;
    void loadData();
  });

  async function loadData() {
    isLoading = true;
    inlineError = null;
    try {
      const result = await commands.getRailwayModelById(railwayModelId, getLocale());
      if (result.status === 'error') {
        toaster.error(m.specs_drawer_save_error());
        onClose();
        return;
      }
      if (!result.data) {
        toaster.error(m.specs_drawer_save_error());
        onClose();
        return;
      }
      const rs = result.data.rollingStock.find((r) => {
        if ('locomotive' in r) return r.locomotive.id === rollingStockId;
        if ('electricMultipleUnit' in r) return r.electricMultipleUnit.id === rollingStockId;
        if ('freightCar' in r) return r.freightCar.id === rollingStockId;
        if ('passengerCar' in r) return r.passengerCar.id === rollingStockId;
        if ('railcar' in r) return r.railcar.id === rollingStockId;
        return false;
      });
      if (!rs) {
        toaster.error(m.specs_drawer_save_error());
        onClose();
        return;
      }
      const data = extractRsData(rs);
      form = { ...data };
      originalForm = { ...data };
    } finally {
      isLoading = false;
    }
  }

  // ── Save ────────────────────────────────────────────────────────────────────
  async function handleSave() {
    isSaving = true;
    inlineError = null;
    try {
      const result = await commands.updateRollingStockSpecifications({
        railwayModelId,
        rollingStockId,
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

      if (result.status === 'error') {
        inlineError = m.specs_drawer_save_error();
        return;
      }

      toaster.success(m.specs_drawer_save_success());
      originalForm = { ...form };
      onSaved?.();
      onClose();
    } finally {
      isSaving = false;
    }
  }
</script>

<DrawerShell
  {open}
  {onClose}
  size="lg"
  hasChanges={isDirty}
  labelledby="rs-specs-title"
  error={inlineError}
>
  {#snippet header({ requestClose })}
    <DrawerHeader
      id="rs-specs-title"
      title={m.specs_drawer_title()}
      icon={Settings}
      onClose={requestClose}
    />
  {/snippet}

  {#if isLoading}
    <div class="flex h-32 items-center justify-center">
      <div
        class="h-6 w-6 animate-spin rounded-full border-2 border-[#E2994F] border-t-transparent"
      ></div>
    </div>
  {:else}
    <div class="space-y-6">
      <RollingStockBasicFields
        bind:seriesCode={form.seriesCode}
        bind:roadNumber={form.roadNumber}
        bind:livery={form.livery}
        bind:depot={form.depot}
      />

      <RollingStockTechnicalFields
        bind:flywheelFitted={form.flywheelFitted}
        bind:bodyShell={form.bodyShell}
        bind:chassis={form.chassis}
        bind:interiorLights={form.interiorLights}
        bind:lights={form.lights}
        bind:dccInterface={form.dccInterface}
        bind:control={form.control}
        bind:couplingSocket={form.couplingSocket}
        bind:closeCouplers={form.closeCouplers}
        bind:digitalShunting={form.digitalShunting}
        {bodyShellOptions}
        {chassisOptions}
        {featureFlagOptions}
        {controlOptions}
        {dccInterfaceOptions}
        {couplingSockeOptions}
      />
    </div>
  {/if}

  {#snippet footer({ requestClose })}
    <DrawerFooter
      cancelLabel={m.specs_drawer_cancel()}
      submitLabel={m.specs_drawer_save()}
      onCancel={requestClose}
      onSubmit={handleSave}
      submitting={isSaving}
      {isLoading}
      disabled={!form.seriesCode.trim()}
    />
  {/snippet}
</DrawerShell>
