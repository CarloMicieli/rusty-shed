<script lang="ts">
  import { Settings } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages';
  import { getLocale } from '$lib/paraglide/runtime.js';
  import { toaster } from '$lib/toaster';
  import {
    commands,
    type Control,
    type CouplerType,
    type DccInterface,
    type FeatureFlag,
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
    /** The owned rolling stock ID. */
    ownedRollingStockId: string;
    /** The currently installed coupler type ID, if any. */
    currentCouplerId?: string | null;
    /** Called after a successful save (to trigger parent refresh). */
    onSaved?: () => void;
    /** Called when the drawer requests to close. */
    onClose: () => void;
  }

  let {
    open,
    railwayModelId,
    rollingStockId,
    ownedRollingStockId,
    currentCouplerId = null,
    onSaved,
    onClose
  }: Props = $props();

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
    flywheelFitted: FeatureFlag;
    sprungBuffers: FeatureFlag;
    bodyShell: string;
    chassis: string;
    interiorLights: FeatureFlag;
    lights: FeatureFlag;
    dccInterface: string;
    control: string;
    couplingSocket: string;
    closeCouplers: FeatureFlag;
    digitalShunting: FeatureFlag;
    selectedCouplerTypeId: string | null;
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
    flywheelFitted: 'NOT_APPLICABLE',
    sprungBuffers: 'NOT_APPLICABLE',
    bodyShell: '',
    chassis: '',
    interiorLights: 'NOT_APPLICABLE',
    lights: 'NOT_APPLICABLE',
    dccInterface: '',
    control: '',
    couplingSocket: '',
    closeCouplers: 'NOT_APPLICABLE',
    digitalShunting: 'NOT_APPLICABLE',
    selectedCouplerTypeId: null,
    lengthMm: null
  };

  let form = $state<FormState>({ ...emptyForm });
  let originalForm = $state<FormState>({ ...emptyForm });
  let isLoading = $state(false);
  let isSaving = $state(false);
  let inlineError = $state<string | null>(null);
  let companyOptions = $state<{ value: string; label: string }[]>([]);
  let expandTechnical = $state(false);
  let allCouplers = $state<CouplerType[]>([]);

  // ── Derived ─────────────────────────────────────────────────────────────────
  const isDirty = $derived(JSON.stringify(form) !== JSON.stringify(originalForm));

  const filteredCouplers = $derived(
    form.couplingSocket
      ? allCouplers.filter((c) => c.compatible_socket === form.couplingSocket)
      : []
  );

  // Clear selected coupler when the socket changes and it's no longer compatible
  $effect(() => {
    if (
      form.selectedCouplerTypeId &&
      !filteredCouplers.some((c) => c.id === form.selectedCouplerTypeId)
    ) {
      form.selectedCouplerTypeId = null;
    }
  });

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
      flywheelFitted: ts?.flywheel_fitted ?? 'NOT_APPLICABLE',
      sprungBuffers: ts?.sprung_buffers ?? 'NOT_APPLICABLE',
      bodyShell: ts?.body_shell ?? '',
      chassis: ts?.chassis ?? '',
      interiorLights: ts?.interior_lights ?? 'NOT_APPLICABLE',
      lights: ts?.lights ?? 'NOT_APPLICABLE',
      dccInterface: 'dcc_interface' in rs ? (rs.dcc_interface ?? '') : '',
      control: 'control' in rs ? (rs.control ?? '') : '',
      couplingSocket: ts?.coupling?.socket ?? '',
      closeCouplers: ts?.coupling?.close_couplers ?? 'NOT_APPLICABLE',
      digitalShunting: ts?.coupling?.digital_shunting ?? 'NOT_APPLICABLE',
      selectedCouplerTypeId: null,
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
    expandTechnical = false;
    try {
      const [modelResult, companiesResult, couplersResult] = await Promise.all([
        commands.getRailwayModelById(railwayModelId, getLocale()),
        commands.getRailwayCompanies(),
        commands.getCouplerTypes(null)
      ]);

      if (modelResult.status === 'error' || !modelResult.data) {
        toaster.error(m.specs_drawer_save_error());
        onClose();
        return;
      }

      if (companiesResult.status === 'ok' && companiesResult.data) {
        companyOptions = companiesResult.data.map((c) => ({ value: c.id, label: c.name }));
      }

      if (couplersResult.status === 'ok' && couplersResult.data) {
        allCouplers = couplersResult.data;
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
      data.selectedCouplerTypeId = currentCouplerId ?? null;
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
    expandTechnical = true;
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
        flywheelFitted:
          form.flywheelFitted === 'YES' ? true : form.flywheelFitted === 'NO' ? false : null,
        sprungBuffers:
          form.sprungBuffers === 'YES' ? true : form.sprungBuffers === 'NO' ? false : null,
        bodyShell: form.bodyShell || null,
        chassis: form.chassis || null,
        interiorLights:
          form.interiorLights === 'NOT_APPLICABLE' ? null : (form.interiorLights as string),
        lights: form.lights === 'NOT_APPLICABLE' ? null : (form.lights as string),
        dccInterface: (form.dccInterface || null) as Parameters<
          typeof commands.updateRollingStockSpecifications
        >[0]['dccInterface'],
        control: (form.control || null) as Parameters<
          typeof commands.updateRollingStockSpecifications
        >[0]['control'],
        couplingSocket: form.couplingSocket || null,
        closeCouplers:
          form.closeCouplers === 'YES' ? true : form.closeCouplers === 'NO' ? false : null,
        digitalShunting:
          form.digitalShunting === 'YES' ? true : form.digitalShunting === 'NO' ? false : null
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

      // Save coupler type if it changed.
      if (form.selectedCouplerTypeId !== originalForm.selectedCouplerTypeId) {
        const couplerResult = await commands.setRollingStockCoupler({
          ownedRollingStockId,
          couplerTypeId: form.selectedCouplerTypeId
        });
        if (couplerResult.status === 'error') {
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
        bind:selectedCouplerTypeId={form.selectedCouplerTypeId}
        bind:lengthMm={form.lengthMm}
        {bodyShellOptions}
        {chassisOptions}
        {controlOptions}
        {dccInterfaceOptions}
        {couplingSockeOptions}
        {filteredCouplers}
        {expandTechnical}
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
