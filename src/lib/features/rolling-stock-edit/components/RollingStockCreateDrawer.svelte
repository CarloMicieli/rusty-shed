<script lang="ts">
  import { Train } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages';
  import { toaster } from '$lib/toaster';
  import {
    commands,
    type PrototypeView,
    type RailwayModelId,
    type RollingStockCategory,
    type RollingStockId
  } from '$lib/bindings';
  import { onMount } from 'svelte';
  import PrototypeLibraryPicker from './PrototypeLibraryPicker.svelte';
  import {
    DrawerShell,
    DrawerHeader,
    DrawerFooter,
    FormInput,
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
      friendlyName: '',
      roadNumber: '',
      livery: '',
      depot: '',
      control: '',
      dccInterface: '',
      couplingSocket: '',
      closeCouplers: false,
      subType: ''
    }),
    validate: (v) => ({
      seriesCode: !v.seriesCode.trim() ? m.error_required() : undefined,
      railwayCompanyId: !v.railwayCompanyId ? m.error_required() : undefined,
      category: !v.category ? m.error_required() : undefined
    })
  });

  let isSaving = $state(false);
  let inlineError = $state<string | null>(null);

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

  // ── Coupling socket options ──────────────────────────────────────────────────
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
  }

  function handlePrototypeClear() {
    f.values.prototypeId = '';
  }

  // ── Save ─────────────────────────────────────────────────────────────────────
  async function handleSave() {
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

      toaster.success(m.rolling_stock_create_success());
      onCreated?.(result.data);
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

    <!-- Block: Identification -->
    <div class="rounded-sm border border-border bg-background/30 p-4">
      <p class="mb-3 font-bebas text-xs tracking-widest text-muted-foreground uppercase">
        {m.rolling_stock_create_section_prototype()}
      </p>
      <div class="space-y-4">
        <PrototypeLibraryPicker
          category={f.values.category}
          selectedId={f.values.prototypeId}
          onSelect={handlePrototypeSelect}
          onClear={handlePrototypeClear}
        />

        <div class="border-t border-border/40"></div>

        <FormSelect
          id="create-company"
          label={m.rolling_stock_field_railway_company()}
          options={companyOptions}
          bind:value={f.values.railwayCompanyId}
          placeholder={m.rolling_stock_select_category()}
          isSearchable
          required
        />
        <div class="grid grid-cols-2 gap-3">
          <FormInput
            id="create-series-code"
            label={m.rolling_stock_field_series_code()}
            bind:value={f.values.seriesCode}
            placeholder="e.g. BR 218"
            required
          />
          <FormInput
            id="create-friendly-name"
            label={m.rolling_stock_field_friendly_name()}
            bind:value={f.values.friendlyName}
            placeholder="e.g. Krocodile"
          />
        </div>
        <div class="grid grid-cols-2 gap-3">
          <FormInput
            id="create-road-number"
            label={m.rolling_stock_field_road_number()}
            bind:value={f.values.roadNumber}
            placeholder="e.g. 218 401-6"
          />
          <FormInput
            id="create-livery"
            label={m.rolling_stock_field_livery()}
            bind:value={f.values.livery}
            placeholder="e.g. Orient Red"
          />
        </div>
        <FormInput
          id="create-depot"
          label={m.rolling_stock_field_depot()}
          bind:value={f.values.depot}
          placeholder="e.g. München Hbf"
        />
      </div>
    </div>

    <!-- Block: Technical Specifications -->
    <div class="rounded-sm border border-border bg-background/30 p-4">
      <p class="mb-3 font-bebas text-xs tracking-widest text-muted-foreground uppercase">
        {m.rolling_stock_create_section_technical()}
      </p>
      <div class="space-y-4">
        <div class="flex items-end gap-3">
          <div class="flex-1">
            <FormSelect
              id="create-coupling-socket"
              label={m.specs_drawer_field_coupling_socket()}
              options={[...couplingSocketOptions]}
              bind:value={f.values.couplingSocket}
              placeholder={m.rolling_stock_select_coupling()}
            />
          </div>
          {#if f.values.couplingSocket}
            <label class="mb-1 flex cursor-pointer items-center gap-2 pb-2.5">
              <input
                type="checkbox"
                class="variant-steampunk-valve"
                bind:checked={f.values.closeCouplers}
              />
              <span class="text-[10px] font-bold text-muted-foreground uppercase">
                {m.rolling_stock_field_short_coupler()}
              </span>
            </label>
          {/if}
        </div>
        <div class="grid grid-cols-2 gap-3">
          <FormSelect
            id="create-control"
            label={m.rolling_stock_field_control_type()}
            options={[...controlOptions]}
            bind:value={f.values.control}
            placeholder={m.rolling_stock_select_control()}
          />
          <FormSelect
            id="create-dcc-interface"
            label={m.rolling_stock_field_dcc_interface()}
            options={[...dccInterfaceOptions]}
            bind:value={f.values.dccInterface}
            placeholder={m.rolling_stock_select_dcc_interface()}
          />
        </div>
      </div>
    </div>
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
