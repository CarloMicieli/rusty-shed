<script lang="ts">
  import { Train } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages';
  import { toaster } from '$lib/toaster';
  import {
    commands,
    type Control,
    type CouplerType,
    type DccInterface,
    type FeatureFlag,
    type PrototypeView,
    type RailwayModelId,
    type RollingStockCategory,
    type RollingStockId
  } from '$lib/bindings';
  import { onMount, untrack } from 'svelte';
  import type { SelectOption } from '$lib/components/SearchableSelect.svelte';
  import RollingStockPrototypeSection from './RollingStockPrototypeSection.svelte';
  import RollingStockTechnicalFields from './RollingStockTechnicalFields.svelte';
  import { DrawerShell, DrawerHeader, DrawerFooter, FormSelect } from '$lib/components/drawer';
  import {
    CONTROL_OPTIONS,
    DCC_INTERFACE_OPTIONS,
    getSubcategoryOptions
  } from '$lib/components/model-details/components/constants';
  import { superForm } from 'sveltekit-superforms';
  import { zod4 as zod } from 'sveltekit-superforms/adapters';
  import { rollingStockCreateSchema } from '$lib/schemas/rolling-stock-form';

  interface Props {
    /** Controls drawer visibility. */
    open: boolean;
    /** The parent railway model. */
    railwayModelId: RailwayModelId;
    /** Called after a successful creation with the new rolling stock id. */
    onCreated?: (id: RollingStockId) => void;
    /** Called when the drawer requests to close. */
    onClose: () => void;
    /**
     * When false, the Coupler Type selector is hidden.
     * Set to false in wishlist context where no owned rolling stock exists yet.
     */
    showCouplerType?: boolean;
  }

  let { open, railwayModelId, onCreated, onClose, showCouplerType = true }: Props = $props();

  function getInitialData() {
    return {
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
      closeCouplers: 'NOT_APPLICABLE' as FeatureFlag,
      subType: '',
      flywheelFitted: 'NOT_APPLICABLE' as FeatureFlag,
      sprungBuffers: 'NOT_APPLICABLE' as FeatureFlag,
      bodyShell: '',
      chassis: '',
      interiorLights: 'NOT_APPLICABLE' as FeatureFlag,
      lights: 'NOT_APPLICABLE' as FeatureFlag,
      digitalShunting: 'NOT_APPLICABLE' as FeatureFlag,
      lengthMm: null as number | null,
      selectedCouplerTypeId: null as string | null,
      isDummy: 'NOT_APPLICABLE' as FeatureFlag
    };
  }

  let isSaving = $state(false);
  let inlineError = $state<string | null>(null);
  let expandTechnical = $state(false);
  let formEl: HTMLFormElement | undefined = $state();

  // ── Company + coupler options ─────────────────────────────────────────────────
  let companyOptions = $state<SelectOption[]>([]);
  let allCouplers = $state<CouplerType[]>([]);

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const { form, tainted, enhance, reset, isTainted } = superForm(getInitialData() as any, {
    SPA: true,
    dataType: 'json',
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    validators: zod(rollingStockCreateSchema as any),
    onUpdate: async ({ form: fd }) => {
      if (!fd.valid) return;
      await handleSave();
    }
  });

  const hasChanges = $derived(isTainted($tainted));

  onMount(async () => {
    const [companiesResult, couplersResult] = await Promise.all([
      commands.getRailwayCompanies(),
      commands.getCouplerTypes(null)
    ]);
    if (companiesResult.status === 'ok') {
      companyOptions = companiesResult.data.map((c) => ({
        value: c.id,
        label: c.name,
        countryCode: c.countryCode,
        registeredCompanyName: c.registeredCompanyName
      }));
    }
    if (couplersResult.status === 'ok') {
      allCouplers = couplersResult.data;
    }
  });

  // ── Reset form when drawer opens ────────────────────────────────────────────
  $effect.pre(() => {
    if (open) {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      reset({ data: getInitialData() as any });
      _prevCategory = '';
      inlineError = null;
      expandTechnical = false;
    }
  });

  // ── Derived state ────────────────────────────────────────────────────────────
  const subTypeOptions = $derived(
    getSubcategoryOptions(($form.category as RollingStockCategory) || null).map((o) => ({
      value: o.id,
      label: o.label
    }))
  );

  const hasSubTypes = $derived(subTypeOptions.length > 0);

  const isFormValid = $derived(
    !!($form.seriesCode as string).trim() && !!$form.railwayCompanyId && !!$form.category
  );

  const filteredCouplers = $derived(
    $form.couplingSocket
      ? allCouplers.filter((c) => c.compatible_socket === ($form.couplingSocket as string))
      : []
  );

  // Clear coupler type when socket changes and it's no longer compatible
  $effect(() => {
    const coupler = $form.selectedCouplerTypeId;
    const valid = filteredCouplers.some((c) => c.id === coupler);
    if (coupler && !valid) {
      untrack(() => {
        $form.selectedCouplerTypeId = null;
      });
    }
  });

  // Reset sub-type when category actually changes (plain var avoids reactive loop)
  let _prevCategory = '';

  $effect(() => {
    const cat = $form.category as string;
    if (cat !== _prevCategory) {
      const isInitial = _prevCategory === '';
      _prevCategory = cat;
      if (!isInitial) {
        untrack(() => {
          $form.subType = '';
        });
      }
    }
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
    $form.prototypeId = p.id;
    $form.railwayCompanyId = p.railway_company_id;
    $form.seriesCode = p.series_code;
    $form.friendlyName = p.friendly_name ?? '';
    const subType =
      p.locomotive_type ??
      p.passenger_car_type ??
      p.freight_car_type ??
      p.railcar_type ??
      p.electric_multiple_unit_type ??
      '';
    if (subType) $form.subType = subType;
    $form.isDummy = p.default_is_dummy ? 'YES' : 'NO';
    expandTechnical = true;
  }

  function handlePrototypeClear() {
    $form.prototypeId = '';
  }

  // ── Save ─────────────────────────────────────────────────────────────────────
  async function handleSave() {
    isSaving = true;
    inlineError = null;
    try {
      const result = await commands.addRollingStockToModel({
        railwayModelId,
        railwayCompanyId: $form.railwayCompanyId,
        category: $form.category,
        seriesCode: ($form.seriesCode as string).trim(),
        friendlyName: ($form.friendlyName as string) || null,
        roadNumber: ($form.roadNumber as string) || null,
        livery: ($form.livery as string) || null,
        depot: ($form.depot as string) || null,
        control: ($form.control as string) || null,
        dccInterface: ($form.dccInterface as string) || null,
        couplingSocket: ($form.couplingSocket as string) || null,
        closeCouplers: $form.couplingSocket
          ? $form.closeCouplers === 'YES'
            ? true
            : $form.closeCouplers === 'NO'
              ? false
              : null
          : null,
        subType: ($form.subType as string) || null,
        prototypeId: ($form.prototypeId as string) || null,
        isDummy: $form.isDummy === 'YES' ? true : $form.isDummy === 'NO' ? false : null
      } as Parameters<typeof commands.addRollingStockToModel>[0]);

      if (result.status === 'error') {
        console.error('[RollingStockCreateDrawer] addRollingStockToModel error:', result.error);
        inlineError = m.rolling_stock_create_error();
        return;
      }

      const { rollingStockId: newId, ownedRollingStockId } = result.data;

      // Save extended technical specs if any are filled
      const hasExtendedSpecs =
        $form.flywheelFitted !== 'NOT_APPLICABLE' ||
        $form.sprungBuffers !== 'NOT_APPLICABLE' ||
        $form.bodyShell ||
        $form.chassis ||
        $form.interiorLights !== 'NOT_APPLICABLE' ||
        $form.lights !== 'NOT_APPLICABLE' ||
        $form.digitalShunting !== 'NOT_APPLICABLE' ||
        $form.series;

      if (hasExtendedSpecs) {
        const specsResult = await commands.updateRollingStockSpecifications({
          railwayModelId,
          rollingStockId: newId,
          seriesCode: ($form.seriesCode as string).trim(),
          series: ($form.series as string) || null,
          roadNumber: ($form.roadNumber as string) || null,
          friendlyName: ($form.friendlyName as string) || null,
          livery: ($form.livery as string) || null,
          depot: ($form.depot as string) || null,
          flywheelFitted:
            $form.flywheelFitted === 'YES' ? true : $form.flywheelFitted === 'NO' ? false : null,
          sprungBuffers:
            $form.sprungBuffers === 'YES' ? true : $form.sprungBuffers === 'NO' ? false : null,
          bodyShell: ($form.bodyShell as string) || null,
          chassis: ($form.chassis as string) || null,
          interiorLights:
            $form.interiorLights === 'NOT_APPLICABLE' ? null : ($form.interiorLights as string),
          lights: $form.lights === 'NOT_APPLICABLE' ? null : ($form.lights as string),
          dccInterface: (($form.dccInterface as string) || null) as DccInterface | null,
          control: (($form.control as string) || null) as Control | null,
          couplingSocket: ($form.couplingSocket as string) || null,
          closeCouplers: $form.couplingSocket
            ? $form.closeCouplers === 'YES'
              ? true
              : $form.closeCouplers === 'NO'
                ? false
                : null
            : null,
          digitalShunting:
            $form.digitalShunting === 'YES' ? true : $form.digitalShunting === 'NO' ? false : null,
          isDummy: $form.isDummy === 'YES' ? true : $form.isDummy === 'NO' ? false : null
        });
        if (specsResult.status === 'error') {
          inlineError = m.rolling_stock_create_error();
          return;
        }
      }

      // Save length if provided
      if ($form.lengthMm !== null) {
        await commands.updateRollingStockDcc({
          railwayModelId,
          rollingStockId: newId,
          control: (($form.control as string) || null) as Control | null,
          dccInterface: (($form.dccInterface as string) || null) as DccInterface | null,
          lengthMillimeters: $form.lengthMm as number,
          lengthInches: null
        });
      }

      // Save coupler type if selected
      if ($form.selectedCouplerTypeId && ownedRollingStockId !== null) {
        const couplerResult = await commands.setRollingStockCoupler({
          ownedRollingStockId,
          couplerTypeId: $form.selectedCouplerTypeId as string
        });
        if (couplerResult.status === 'error') {
          inlineError = m.rolling_stock_create_error();
          return;
        }
      }

      toaster.success(m.rolling_stock_create_success());
      onCreated?.(newId);
      onClose();
    } finally {
      isSaving = false;
    }
  }

  function handleSubmit() {
    formEl?.requestSubmit();
  }
</script>

<DrawerShell
  {open}
  {onClose}
  size="xl"
  {hasChanges}
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

  <form bind:this={formEl} use:enhance class="contents">
    <div class="space-y-3">
      <!-- Category + Type row (no card) -->
      <div class="grid grid-cols-2 gap-3">
        <FormSelect
          id="create-category"
          label={m.rolling_stock_field_category()}
          options={[...categoryOptions]}
          bind:value={$form.category}
          placeholder={m.rolling_stock_select_category()}
          required
        />
        {#if hasSubTypes}
          <FormSelect
            id="create-sub-type"
            label={m.rolling_stock_field_type()}
            options={subTypeOptions}
            bind:value={$form.subType}
            placeholder={m.rolling_stock_select_sub_type()}
          />
        {:else}
          <div></div>
        {/if}
      </div>

      <!-- Prototype / Identification -->
      <RollingStockPrototypeSection
        bind:railwayCompanyId={$form.railwayCompanyId}
        {companyOptions}
        bind:seriesCode={$form.seriesCode}
        bind:series={$form.series}
        bind:friendlyName={$form.friendlyName}
        bind:roadNumber={$form.roadNumber}
        bind:livery={$form.livery}
        bind:depot={$form.depot}
        category={$form.category}
        selectedPrototypeId={$form.prototypeId}
        onPrototypeSelect={handlePrototypeSelect}
        onPrototypeClear={handlePrototypeClear}
      />

      <!-- Technical Specifications -->
      <RollingStockTechnicalFields
        bind:flywheelFitted={$form.flywheelFitted}
        bind:sprungBuffers={$form.sprungBuffers}
        bind:bodyShell={$form.bodyShell}
        bind:chassis={$form.chassis}
        bind:interiorLights={$form.interiorLights}
        bind:lights={$form.lights}
        bind:dccInterface={$form.dccInterface}
        bind:control={$form.control}
        bind:couplingSocket={$form.couplingSocket}
        bind:closeCouplers={$form.closeCouplers}
        bind:digitalShunting={$form.digitalShunting}
        bind:isDummy={$form.isDummy}
        bind:lengthMm={$form.lengthMm}
        {bodyShellOptions}
        {chassisOptions}
        controlOptions={[...controlOptions]}
        dccInterfaceOptions={[...dccInterfaceOptions]}
        couplingSockeOptions={[...couplingSocketOptions]}
        {filteredCouplers}
        bind:selectedCouplerTypeId={$form.selectedCouplerTypeId}
        {expandTechnical}
        category={$form.category}
        {showCouplerType}
      />
    </div>
  </form>

  {#snippet footer({ requestClose })}
    <DrawerFooter
      cancelLabel={m.specs_drawer_cancel()}
      submitLabel={m.specs_drawer_save()}
      onCancel={requestClose}
      onSubmit={handleSubmit}
      submitting={isSaving}
      disabled={!isFormValid}
    />
  {/snippet}
</DrawerShell>
