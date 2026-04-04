<script lang="ts">
  import { Train } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages';
  import { toaster } from '$lib/toaster';
  import {
    commands,
    type Control,
    type DccInterface,
    type PrototypeView,
    type RailwayModelId,
    type RollingStockCategory,
    type RollingStockId
  } from '$lib/bindings';
  import { onMount } from 'svelte';
  import RollingStockPrototypeSection from './RollingStockPrototypeSection.svelte';
  import RollingStockTechnicalFields from './RollingStockTechnicalFields.svelte';
  import {
    DrawerShell,
    DrawerHeader,
    DrawerFooter,
    FormSelect,
    createDrawerForm
  } from '$lib/components/drawer';
  import {
    CONTROL_OPTIONS,
    DCC_INTERFACE_OPTIONS,
    getSubcategoryOptions
  } from '$lib/components/model-details/components/constants';

  interface Props {
    /** Controls drawer visibility. */
    open: boolean;
    /** The parent railway model. */
    railwayModelId: RailwayModelId;
    /** Called after a successful creation with the new rolling stock id. */
    onCreated?: (id: RollingStockId) => void;
    /** Called when the drawer requests to close. */
    onClose: () => void;
  }

  let { open, railwayModelId, onCreated, onClose }: Props = $props();

  const f = createDrawerForm({
    initial: () => ({
      prototypeId: '',
      railwayCompanyId: '',
      category: '',
      seriesCode: '',
      series: '',
      friendlyName: '',
      roadNumber: '',
      livery: '',
      depot: '',
      control: '',
      dccInterface: '',
      couplingSocket: '',
      closeCouplers: false,
      subType: '',
      flywheelFitted: null as boolean | null,
      sprungBuffers: null as boolean | null,
      bodyShell: '',
      chassis: '',
      interiorLights: '',
      lights: '',
      digitalShunting: null as boolean | null,
      lengthMm: null as number | null
    }),
    validate: (v) => ({
      seriesCode: !v.seriesCode.trim() ? m.error_required() : undefined,
      railwayCompanyId: !v.railwayCompanyId ? m.error_required() : undefined,
      category: !v.category ? m.error_required() : undefined
    })
  });

  let isSaving = $state(false);
  let inlineError = $state<string | null>(null);
  let expandTechnical = $state(false);

  // ── Company options ──────────────────────────────────────────────────────────
  let companyOptions = $state<{ value: string; label: string }[]>([]);

  onMount(async () => {
    const result = await commands.getRailwayCompanies();
    if (result.status === 'ok') {
      companyOptions = result.data.map((c) => ({ value: c.id, label: c.name }));
    }
  });

  // ── Reset form when drawer opens ────────────────────────────────────────────
  $effect(() => {
    if (open) {
      f.reset();
      inlineError = null;
      expandTechnical = false;
    }
  });

  // ── Derived state ────────────────────────────────────────────────────────────
  const subTypeOptions = $derived(
    getSubcategoryOptions((f.values.category as RollingStockCategory) || null).map((o) => ({
      value: o.id,
      label: o.label
    }))
  );

  const hasSubTypes = $derived(subTypeOptions.length > 0);

  // Reset sub-type when category changes
  $effect(() => {
    // eslint-disable-next-line @typescript-eslint/no-unused-expressions
    f.values.category;
    f.values.subType = '';
  });

  // ── Option lists ─────────────────────────────────────────────────────────────
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

  const couplingSocketOptions = [
    { value: '', label: '—' },
    { value: 'NONE', label: 'None' },
    { value: 'NEM_355', label: 'NEM 355' },
    { value: 'NEM_356', label: 'NEM 356' },
    { value: 'NEM_357', label: 'NEM 357' },
    { value: 'NEM_359', label: 'NEM 359' },
    { value: 'NEM_360', label: 'NEM 360' },
    { value: 'NEM_362', label: 'NEM 362' },
    { value: 'NEM_365', label: 'NEM 365' }
  ] as const;

  const dccInterfaceOptions = [
    { value: '', label: '—' },
    ...DCC_INTERFACE_OPTIONS.filter((o) => o.id !== '').map((o) => ({
      value: o.id,
      label: o.label
    }))
  ] as const;

  const controlOptions = [
    { value: '', label: '—' },
    ...CONTROL_OPTIONS.filter((o) => o.id !== '').map((o) => ({ value: o.id, label: o.label }))
  ] as const;

  const categoryOptions = [
    { value: 'LOCOMOTIVE', label: 'Locomotive' },
    { value: 'ELECTRIC_MULTIPLE_UNIT', label: 'Electric Multiple Unit' },
    { value: 'PASSENGER_CAR', label: 'Passenger Car' },
    { value: 'FREIGHT_CAR', label: 'Freight Car' },
    { value: 'RAILCAR', label: 'Railcar' }
  ] as const;

  // ── Prototype autofill ────────────────────────────────────────────────────────
  function handlePrototypeSelect(p: PrototypeView) {
    f.values.prototypeId = p.id;
    f.values.railwayCompanyId = p.railway_company_id;
    f.values.seriesCode = p.series_code;
    f.values.friendlyName = p.friendly_name ?? '';
    const subType =
      p.locomotive_type ??
      p.passenger_car_type ??
      p.freight_car_type ??
      p.railcar_type ??
      p.electric_multiple_unit_type ??
      '';
    if (subType) f.values.subType = subType;
    expandTechnical = true;
  }

  function handlePrototypeClear() {
    f.values.prototypeId = '';
  }

  // ── Save ─────────────────────────────────────────────────────────────────────
  async function handleSave() {
    f.touch();
    if (!f.isValid) return;
    isSaving = true;
    inlineError = null;
    try {
      const result = await commands.addRollingStockToModel({
        railwayModelId,
        railwayCompanyId: f.values.railwayCompanyId,
        category: f.values.category,
        seriesCode: f.values.seriesCode.trim(),
        friendlyName: f.values.friendlyName || null,
        roadNumber: f.values.roadNumber || null,
        livery: f.values.livery || null,
        depot: f.values.depot || null,
        control: f.values.control || null,
        dccInterface: f.values.dccInterface || null,
        couplingSocket: f.values.couplingSocket || null,
        closeCouplers: f.values.couplingSocket ? f.values.closeCouplers : null,
        subType: f.values.subType || null,
        prototypeId: f.values.prototypeId || null
      });

      if (result.status === 'error') {
        inlineError = m.rolling_stock_create_error();
        return;
      }

      const newId = result.data;

      // Save extended technical specs if any are filled
      const hasExtendedSpecs =
        f.values.flywheelFitted !== null ||
        f.values.sprungBuffers !== null ||
        f.values.bodyShell ||
        f.values.chassis ||
        f.values.interiorLights ||
        f.values.lights ||
        f.values.digitalShunting !== null ||
        f.values.series;

      if (hasExtendedSpecs) {
        const specsResult = await commands.updateRollingStockSpecifications({
          railwayModelId,
          rollingStockId: newId,
          seriesCode: f.values.seriesCode.trim(),
          series: f.values.series || null,
          roadNumber: f.values.roadNumber || null,
          friendlyName: f.values.friendlyName || null,
          livery: f.values.livery || null,
          depot: f.values.depot || null,
          flywheelFitted: f.values.flywheelFitted,
          sprungBuffers: f.values.sprungBuffers,
          bodyShell: f.values.bodyShell || null,
          chassis: f.values.chassis || null,
          interiorLights: f.values.interiorLights || null,
          lights: f.values.lights || null,
          dccInterface: (f.values.dccInterface || null) as DccInterface | null,
          control: (f.values.control || null) as Control | null,
          couplingSocket: f.values.couplingSocket || null,
          closeCouplers: f.values.couplingSocket ? f.values.closeCouplers : null,
          digitalShunting: f.values.digitalShunting
        });
        if (specsResult.status === 'error') {
          inlineError = m.rolling_stock_create_error();
          return;
        }
      }

      // Save length if provided
      if (f.values.lengthMm !== null) {
        await commands.updateRollingStockDcc({
          railwayModelId,
          rollingStockId: newId,
          control: (f.values.control || null) as Control | null,
          dccInterface: (f.values.dccInterface || null) as DccInterface | null,
          lengthMillimeters: f.values.lengthMm,
          lengthInches: null
        });
      }

      toaster.success(m.rolling_stock_create_success());
      onCreated?.(newId);
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
  hasChanges={f.isDirty}
  labelledby="rs-create-title"
  error={inlineError}
>
  {#snippet header({ requestClose })}
    <DrawerHeader
      id="rs-create-title"
      title={m.rolling_stock_create_drawer_title()}
      subtitle={m.rolling_stock_create_drawer_subtitle()}
      icon={Train}
      onClose={requestClose}
    />
  {/snippet}

  <div class="space-y-3">
    <!-- Category + Type row (no card) -->
    <div class="grid grid-cols-2 gap-3">
      <FormSelect
        id="create-category"
        label={m.rolling_stock_field_category()}
        options={[...categoryOptions]}
        bind:value={f.values.category}
        placeholder={m.rolling_stock_select_category()}
        required
      />
      {#if hasSubTypes}
        <FormSelect
          id="create-sub-type"
          label={m.rolling_stock_field_type()}
          options={subTypeOptions}
          bind:value={f.values.subType}
          placeholder={m.rolling_stock_select_sub_type()}
        />
      {:else}
        <div></div>
      {/if}
    </div>

    <!-- Prototype / Identification -->
    <RollingStockPrototypeSection
      bind:railwayCompanyId={f.values.railwayCompanyId}
      {companyOptions}
      bind:seriesCode={f.values.seriesCode}
      bind:series={f.values.series}
      bind:friendlyName={f.values.friendlyName}
      bind:roadNumber={f.values.roadNumber}
      bind:livery={f.values.livery}
      bind:depot={f.values.depot}
      category={f.values.category}
      selectedPrototypeId={f.values.prototypeId}
      onPrototypeSelect={handlePrototypeSelect}
      onPrototypeClear={handlePrototypeClear}
    />

    <!-- Technical Specifications -->
    <RollingStockTechnicalFields
      bind:flywheelFitted={f.values.flywheelFitted}
      bind:sprungBuffers={f.values.sprungBuffers}
      bind:bodyShell={f.values.bodyShell}
      bind:chassis={f.values.chassis}
      bind:interiorLights={f.values.interiorLights}
      bind:lights={f.values.lights}
      bind:dccInterface={f.values.dccInterface}
      bind:control={f.values.control}
      bind:couplingSocket={f.values.couplingSocket}
      bind:closeCouplers={f.values.closeCouplers}
      bind:digitalShunting={f.values.digitalShunting}
      bind:lengthMm={f.values.lengthMm}
      {bodyShellOptions}
      {chassisOptions}
      {featureFlagOptions}
      controlOptions={[...controlOptions]}
      dccInterfaceOptions={[...dccInterfaceOptions]}
      couplingSockeOptions={[...couplingSocketOptions]}
      {expandTechnical}
    />
  </div>

  {#snippet footer({ requestClose })}
    <DrawerFooter
      cancelLabel={m.specs_drawer_cancel()}
      submitLabel={m.specs_drawer_save()}
      onCancel={requestClose}
      onSubmit={handleSave}
      submitting={isSaving}
      disabled={!f.isValid}
    />
  {/snippet}
</DrawerShell>
