<script lang="ts">
  import { Settings } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages';
  import { getLocale } from '$lib/paraglide/runtime.js';
  import { toaster } from '$lib/toaster';
  import {
    commands,
    type Control,
    type DccInterface,
    type PrototypeView,
    type RailwayCompanyId,
    type RailwayModelId,
    type RollingStockId,
    type RollingStockView
  } from '$lib/bindings';
  import RollingStockPrototypeSection from './RollingStockPrototypeSection.svelte';
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
    category: string;
    railwayCompanyId: string;
    seriesCode: string;
    series: string;
    roadNumber: string;
    friendlyName: string;
    livery: string;
    depot: string;
    flywheelFitted: boolean | null;
    sprungBuffers: boolean | null;
    bodyShell: string;
    chassis: string;
    interiorLights: string;
    lights: string;
    dccInterface: string;
    control: string;
    couplingSocket: string;
    closeCouplers: boolean | null;
    digitalShunting: boolean | null;
    lengthMm: number | null;
  }

  const emptyForm: FormState = {
    category: '',
    railwayCompanyId: '',
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
    lengthMm: null
  };

  let form = $state<FormState>({ ...emptyForm });
  let originalForm = $state<FormState>({ ...emptyForm });
  let isLoading = $state(false);
  let isSaving = $state(false);
  let inlineError = $state<string | null>(null);
  let companyOptions = $state<{ value: string; label: string }[]>([]);

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
  function extractRsData(view: RollingStockView): FormState {
    const rs =
      ('locomotive' in view && view.locomotive) ||
      ('electricMultipleUnit' in view && view.electricMultipleUnit) ||
      ('freightCar' in view && view.freightCar) ||
      ('passengerCar' in view && view.passengerCar) ||
      ('railcar' in view && view.railcar) ||
      null;

    if (!rs) return { ...emptyForm };

    const category =
      'locomotive' in view && view.locomotive
        ? 'LOCOMOTIVE'
        : 'electricMultipleUnit' in view && view.electricMultipleUnit
          ? 'ELECTRIC_MULTIPLE_UNIT'
          : 'freightCar' in view && view.freightCar
            ? 'FREIGHT_CAR'
            : 'passengerCar' in view && view.passengerCar
              ? 'PASSENGER_CAR'
              : 'RAILCAR';

    const ts = rs.technical_specifications;
    const lob = rs.length_over_buffer;

    return {
      category,
      railwayCompanyId: rs.railway.railwayCompanyId ?? '',
      seriesCode: rs.series_code,
      series: 'series' in rs ? (rs.series ?? '') : '',
      roadNumber: rs.road_number ?? '',
      friendlyName: rs.friendly_name ?? '',
      livery: rs.livery ?? '',
      depot: 'depot' in rs ? (rs.depot ?? '') : '',
      flywheelFitted:
        ts?.flywheel_fitted === 'YES' ? true : ts?.flywheel_fitted === 'NO' ? false : null,
      sprungBuffers:
        ts?.sprung_buffers === 'YES' ? true : ts?.sprung_buffers === 'NO' ? false : null,
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
      lengthMm: lob?.millimeters ? Number(lob.millimeters) : null
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
      const [modelResult, companiesResult] = await Promise.all([
        commands.getRailwayModelById(railwayModelId, getLocale()),
        commands.getRailwayCompanies()
      ]);

      if (modelResult.status === 'error' || !modelResult.data) {
        toaster.error(m.specs_drawer_save_error());
        onClose();
        return;
      }

      if (companiesResult.status === 'ok' && companiesResult.data) {
        companyOptions = companiesResult.data.map((c) => ({ value: c.id, label: c.name }));
      }

      const rs = modelResult.data.rollingStock.find((r) => {
        if ('locomotive' in r && r.locomotive) return r.locomotive.id === rollingStockId;
        if ('electricMultipleUnit' in r && r.electricMultipleUnit)
          return r.electricMultipleUnit.id === rollingStockId;
        if ('freightCar' in r && r.freightCar) return r.freightCar.id === rollingStockId;
        if ('passengerCar' in r && r.passengerCar) return r.passengerCar.id === rollingStockId;
        if ('railcar' in r && r.railcar) return r.railcar.id === rollingStockId;
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

  // ── Prototype autofill (edit drawer — does not persist prototype association) ─
  function handlePrototypeSelect(p: PrototypeView) {
    form.railwayCompanyId = p.railway_company_id;
    form.seriesCode = p.series_code;
    form.friendlyName = p.friendly_name ?? '';
  }

  function handlePrototypeClear() {
    // No-op: clearing prototype in edit mode only resets the picker UI
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
        digitalShunting: form.digitalShunting
      });

      if (result.status === 'error') {
        inlineError = m.specs_drawer_save_error();
        return;
      }

      // Save railway company separately if it changed.
      if (form.railwayCompanyId && form.railwayCompanyId !== originalForm.railwayCompanyId) {
        const companyResult = await commands.updateRollingStockRailwayCompany({
          railwayModelId,
          rollingStockId,
          railwayCompanyId: form.railwayCompanyId as RailwayCompanyId
        });
        if (companyResult.status === 'error') {
          inlineError = m.specs_drawer_save_error();
          return;
        }
      }

      // Save length if it changed.
      if (form.lengthMm !== originalForm.lengthMm) {
        const dccResult = await commands.updateRollingStockDcc({
          railwayModelId,
          rollingStockId,
          control: (form.control || null) as Control | null,
          dccInterface: (form.dccInterface || null) as DccInterface | null,
          lengthMillimeters: form.lengthMm,
          lengthInches: null
        });
        if (dccResult.status === 'error') {
          inlineError = m.specs_drawer_save_error();
          return;
        }
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
  size="xl"
  hasChanges={isDirty}
  labelledby="rs-specs-title"
  error={inlineError}
>
  {#snippet header({ requestClose })}
    <DrawerHeader
      id="rs-specs-title"
      title={m.specs_drawer_title()}
      subtitle={m.specs_drawer_subtitle()}
      icon={Settings}
      onClose={requestClose}
    />
  {/snippet}

  {#if isLoading}
    <div class="flex h-32 items-center justify-center">
      <div
        class="h-6 w-6 animate-spin rounded-full border-2 border-amber-400 border-t-transparent"
      ></div>
    </div>
  {:else}
    <div class="space-y-3">
      <RollingStockPrototypeSection
        bind:railwayCompanyId={form.railwayCompanyId}
        {companyOptions}
        bind:seriesCode={form.seriesCode}
        bind:series={form.series}
        bind:friendlyName={form.friendlyName}
        bind:roadNumber={form.roadNumber}
        bind:livery={form.livery}
        bind:depot={form.depot}
        category={form.category}
        onPrototypeSelect={handlePrototypeSelect}
        onPrototypeClear={handlePrototypeClear}
      />

      <RollingStockTechnicalFields
        bind:flywheelFitted={form.flywheelFitted}
        bind:sprungBuffers={form.sprungBuffers}
        bind:bodyShell={form.bodyShell}
        bind:chassis={form.chassis}
        bind:interiorLights={form.interiorLights}
        bind:lights={form.lights}
        bind:dccInterface={form.dccInterface}
        bind:control={form.control}
        bind:couplingSocket={form.couplingSocket}
        bind:closeCouplers={form.closeCouplers}
        bind:digitalShunting={form.digitalShunting}
        bind:lengthMm={form.lengthMm}
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
