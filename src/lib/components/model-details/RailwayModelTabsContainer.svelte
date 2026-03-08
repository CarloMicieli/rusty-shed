<script lang="ts">
  import type { RailwayModel } from '$lib/types/railway-model';
  import { Tabs, TabsContent, TabsList, TabsTrigger } from '$lib/components/ui/tabs';
  import { getLocale } from '$lib/paraglide/runtime.js';
  import type { Language, RollingStockView, RollingStockId } from '$lib/bindings';
  import { commands } from '$lib/bindings';
  import BadgePicker from '$lib/components/BadgePicker.svelte';
  import InPlaceEdit from '$lib/components/InPlaceEdit.svelte';
  import LanguageFallbackBadge from '$lib/components/LanguageFallbackBadge.svelte';
  import RichTextEditor from '$lib/components/RichTextEditor.svelte';
  import RollingStockCreateDrawer from '$lib/features/rolling-stock-edit/components/RollingStockCreateDrawer.svelte';
  import * as m from '$lib/paraglide/messages';

  interface RsFormState {
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

  interface _Props {
    model: RailwayModel;
    editable?: boolean;
    onModelUpdated?: () => Promise<void> | void;
    onError?: (error: string) => void;
  }

  const { model, editable = false, onModelUpdated, onError }: _Props = $props();

  const currentLocale = getLocale() as Language;

  // Tab state
  let activeTab = $state<'details' | 'rolling-stock'>('details');

  // Details tab state (derived from model)
  let localDetails = $derived.by(() => model.details ?? '');

  // Rolling stock state
  let rollingStockFormState = $state<Map<string, RsFormState>>(new Map());
  let rollingStockSpecLoaded = $state<Set<string>>(new Set());
  let createDrawerOpen = $state(false);

  $effect(() => {
    if (editable && model.rolling_stock) {
      for (const unit of model.rolling_stock) {
        if (!rollingStockSpecLoaded.has(unit.id)) {
          void loadRollingStockSpec(unit.id);
        }
      }
    }
  });

  // ─────────────────────────────────────────────────────────────────────────
  // Derived values
  // ─────────────────────────────────────────────────────────────────────────

  const isSingleUnit = $derived(model.rolling_stock?.length === 1);

  // Option lists for rolling stock fields
  const controlOptions = [
    { id: '', label: '—' },
    { id: 'DCC_READY', label: 'DCC Ready' },
    { id: 'DCC_FITTED', label: 'DCC Fitted' },
    { id: 'DCC_SOUND', label: 'DCC Sound' },
    { id: 'NO_DCC', label: 'Analogue (No DCC)' }
  ];

  const dccInterfaceOptions = [
    { id: '', label: '—' },
    { id: 'NEM_651', label: 'NEM 651' },
    { id: 'NEM_652', label: 'NEM 652' },
    { id: 'NEM_654', label: 'NEM 654' },
    { id: 'PLUX_8', label: 'PLUX 8' },
    { id: 'PLUX_12', label: 'PLUX 12' },
    { id: 'PLUX_16', label: 'PLUX 16' },
    { id: 'PLUX_22', label: 'PLUX 22' },
    { id: 'NEXT_18', label: 'Next18' },
    { id: 'NEXT_18_S', label: 'Next18-S' },
    { id: 'MTC_21', label: 'MTC 21' }
  ];

  const couplingSocketOptions = [
    { id: '', label: '—' },
    { id: 'NONE', label: 'None' },
    { id: 'NEM_355', label: 'NEM 355' },
    { id: 'NEM_356', label: 'NEM 356' },
    { id: 'NEM_357', label: 'NEM 357' },
    { id: 'NEM_359', label: 'NEM 359' },
    { id: 'NEM_360', label: 'NEM 360' },
    { id: 'NEM_362', label: 'NEM 362' },
    { id: 'NEM_365', label: 'NEM 365' }
  ];

  // ─────────────────────────────────────────────────────────────────────────
  // Rolling stock helper functions
  // ─────────────────────────────────────────────────────────────────────────

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

  async function loadRollingStockSpec(unitId: string) {
    if (rollingStockSpecLoaded.has(unitId)) return;

    try {
      const result = await commands.getRailwayModelById(model.id, getLocale());
      if (result.status === 'error' || !result.data) {
        rollingStockFormState.set(unitId, getEmptyRsForm());
        rollingStockSpecLoaded.add(unitId);
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

      if (!rsView) {
        rollingStockFormState.set(unitId, getEmptyRsForm());
      } else {
        rollingStockFormState.set(unitId, extractRsDataFromView(rsView));
      }
      rollingStockSpecLoaded.add(unitId);
    } catch {
      rollingStockFormState.set(unitId, getEmptyRsForm());
      rollingStockSpecLoaded.add(unitId);
    }
  }

  async function saveRollingStockIdentification(
    unitId: string,
    field: 'series' | 'roadNumber' | 'livery' | 'depot',
    value: string,
    unit: (typeof model.rolling_stock)[0]
  ) {
    const currentForm = rollingStockFormState.get(unitId) || getEmptyRsForm();
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

    if (result.status === 'error') {
      throw new Error('Failed to save');
    }

    if (!currentForm) {
      rollingStockFormState.set(unitId, getEmptyRsForm());
    }
    const form = rollingStockFormState.get(unitId)!;
    form.seriesCode = seriesCode;
    form.roadNumber = roadNumber ?? '';
    form.livery = livery ?? '';
    form.depot = depot ?? '';

    await onModelUpdated?.();
  }

  async function saveRollingStockSpec(unitId: string, field: string, value: string) {
    const form = rollingStockFormState.get(unitId);
    if (!form) return;

    // Update the field in form
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

    if (result.status === 'error') {
      throw new Error('Failed to save');
    }

    await onModelUpdated?.();
  }

  async function saveDetails(value: string) {
    const result = await commands.updateRailwayModelText({
      railwayModelId: model.id,
      field: 'Details',
      value,
      lang: getLocale()
    });
    if (result.status === 'error') {
      onError?.(m.details_save_failed());
      throw new Error('Failed to save details');
    }
    localDetails = value;
    await onModelUpdated?.();
  }
</script>

<Tabs bind:value={activeTab} class="w-full">
  <TabsList
    class="grid h-auto w-full grid-cols-2 rounded-lg border border-zinc-800 bg-zinc-900 p-1"
  >
    <TabsTrigger
      value="details"
      class="rounded-md text-xs text-zinc-400 transition-colors
        data-[state=active]:bg-[#E2994F]/10 data-[state=active]:text-[#E2994F] data-[state=active]:shadow-none
        dark:text-zinc-400 dark:data-[state=active]:border-input dark:data-[state=active]:bg-[#E2994F]/10 dark:data-[state=active]:text-[#E2994F]"
    >
      {m.railway_model_details()}
    </TabsTrigger>
    <TabsTrigger
      value="rolling-stock"
      class="rounded-md text-xs text-zinc-400 transition-colors
        data-[state=active]:bg-[#E2994F]/10 data-[state=active]:text-[#E2994F] data-[state=active]:shadow-none
        dark:text-zinc-400 dark:data-[state=active]:border-input dark:data-[state=active]:bg-[#E2994F]/10 dark:data-[state=active]:text-[#E2994F]"
    >
      {m.rolling_stock_list()}
    </TabsTrigger>
  </TabsList>

  <!-- ── Tab 1: Model Details ───────────────────────────────────────────── -->
  <TabsContent value="details" class="mt-2">
    <div class="rounded-lg border border-zinc-800 bg-zinc-900/40 p-4">
      {#if model.detailsLang && model.detailsLang !== currentLocale}
        <div class="mb-2 flex items-center gap-1 text-xs text-zinc-500">
          <span>{m.railway_model_field_details()}</span>
          <LanguageFallbackBadge lang={model.detailsLang} />
        </div>
      {/if}
      <RichTextEditor
        value={localDetails}
        {editable}
        placeholder={m.details_placeholder()}
        onSave={saveDetails}
      />
    </div>
  </TabsContent>

  <!-- ── Tab 2: Rolling Stock ───────────────────────────────────────────── -->
  <TabsContent value="rolling-stock" class="mt-2">
    {#if model.rolling_stock && model.rolling_stock.length > 0}
      {#if isSingleUnit}
        <!-- Single unit: hero road number + 3-column spec grid -->
        {@const unit = model.rolling_stock[0]}
        <div class="rounded-lg border border-zinc-800 bg-zinc-900/40 p-4">
          <h3 class="sr-only">{m.rolling_stock_list()}</h3>

          <!-- Road number as primary identity element -->
          <div class="relative mb-4 border-b border-zinc-800 pb-3">
            <div class="flex items-baseline gap-2 pr-16">
              <span class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                {m.road_number()}
              </span>
              {#if editable}
                <div class="font-mono text-base font-semibold text-zinc-100">
                  <InPlaceEdit
                    value={unit.road_number ?? ''}
                    placeholder={m.road_number()}
                    onSave={(v) =>
                      saveRollingStockIdentification(unit.id, 'roadNumber', v, unit)}
                  />
                </div>
              {:else}
                <span class="font-mono text-base font-semibold text-zinc-100">
                  {unit.road_number ?? '—'}
                </span>
              {/if}
            </div>
            {#if unit.railway_company}
              <span
                class="absolute top-0 right-0 inline-flex items-center rounded-full border border-[#1F1F1F] bg-[#D48A42] px-2 py-0.5 font-mono text-[9px] font-bold tracking-wider text-[#050505] uppercase"
              >
                {unit.railway_company}
              </span>
            {/if}
          </div>

          <!-- 3-column spec grid -->
          <dl class="grid grid-cols-3 gap-x-4 gap-y-3">
            <div class="flex flex-col gap-0.5">
              <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                {m.series_code()}
              </dt>
              <dd class="text-xs text-zinc-200">
                {#if editable}
                  <InPlaceEdit
                    value={unit.series_code}
                    placeholder={m.rolling_stock_field_series_code()}
                    onSave={(v) => saveRollingStockIdentification(unit.id, 'series', v, unit)}
                  />
                {:else}
                  {unit.series_code}{unit.series_name ? ` — ${unit.series_name}` : ''}
                {/if}
              </dd>
            </div>
            <div class="flex flex-col gap-0.5">
              <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                {m.depot()}
              </dt>
              <dd class="text-xs text-zinc-200">
                {#if editable}
                  <InPlaceEdit
                    value={unit.depot ?? ''}
                    placeholder={m.depot()}
                    onSave={(v) => saveRollingStockIdentification(unit.id, 'depot', v, unit)}
                  />
                {:else}
                  {unit.depot ?? '—'}
                {/if}
              </dd>
            </div>
            <div class="flex flex-col gap-0.5">
              <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                {m.livery()}
              </dt>
              <dd class="text-xs text-zinc-200">
                {#if editable}
                  <InPlaceEdit
                    value={unit.livery ?? ''}
                    placeholder={m.livery()}
                    onSave={(v) => saveRollingStockIdentification(unit.id, 'livery', v, unit)}
                  />
                {:else}
                  {unit.livery ?? '—'}
                {/if}
              </dd>
            </div>
            <div class="flex flex-col gap-0.5">
              <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                {m.control_type()}
              </dt>
              <dd class="text-xs text-zinc-200">
                {#if editable && rollingStockSpecLoaded.has(unit.id)}
                  <BadgePicker
                    value={rollingStockFormState.get(unit.id)?.control ??
                      unit.control_type ??
                      '—'}
                    options={controlOptions}
                    onSelect={(id) => saveRollingStockSpec(unit.id, 'control', id)}
                  />
                {:else if editable}
                  <span class="text-xs text-zinc-500 italic">Loading…</span>
                {:else}
                  {unit.control_type ?? '—'}
                {/if}
              </dd>
            </div>
            <div class="flex flex-col gap-0.5">
              <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                {m.dcc_interface()}
              </dt>
              <dd class="text-xs text-zinc-200">
                {#if editable && rollingStockSpecLoaded.has(unit.id)}
                  <BadgePicker
                    value={rollingStockFormState.get(unit.id)?.dccInterface ??
                      unit.dcc_interface ??
                      '—'}
                    options={dccInterfaceOptions}
                    onSelect={(id) => saveRollingStockSpec(unit.id, 'dccInterface', id)}
                  />
                {:else if editable}
                  <span class="text-xs text-zinc-500 italic">Loading…</span>
                {:else}
                  {unit.dcc_interface ?? '—'}
                {/if}
              </dd>
            </div>
            <div class="flex flex-col gap-0.5">
              <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                Length
              </dt>
              <dd class="font-mono text-xs text-zinc-200">
                {unit.length_mm != null ? `${unit.length_mm} mm` : '—'}
              </dd>
            </div>
            <div class="flex flex-col gap-0.5">
              <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                {m.coupling_type()}
              </dt>
              <dd class="text-xs text-zinc-200">
                {#if editable && rollingStockSpecLoaded.has(unit.id)}
                  <BadgePicker
                    value={rollingStockFormState.get(unit.id)?.couplingSocket ??
                      unit.coupling_type ??
                      '—'}
                    options={couplingSocketOptions}
                    onSelect={(id) => saveRollingStockSpec(unit.id, 'couplingSocket', id)}
                  />
                {:else if editable}
                  <span class="text-xs text-zinc-500 italic">Loading…</span>
                {:else}
                  {unit.coupling_type ?? '—'}
                {/if}
              </dd>
            </div>
          </dl>
        </div>
      {:else}
        <!-- Multi-unit: mini-cards with consistent 3-column spec grid -->
        <div class="space-y-2">
          <h3 class="sr-only">{m.rolling_stock_list()}</h3>
          {#each model.rolling_stock as unit (unit.id)}
            <div class="rounded-lg border border-zinc-800 bg-zinc-900/40 p-3">
              <!-- Mini-card header: series name + railway company -->
              <div class="relative mb-2.5 border-b border-zinc-800/60 pb-2">
                <div class="flex min-w-0 items-baseline gap-2 pr-16">
                  {#if editable}
                    <div class="truncate text-xs font-medium text-zinc-200">
                      <InPlaceEdit
                        value={unit.series_code}
                        placeholder={m.rolling_stock_field_series_code()}
                        onSave={(v) =>
                          saveRollingStockIdentification(unit.id, 'series', v, unit)}
                      />
                    </div>
                    <div class="shrink-0 font-mono text-sm font-semibold text-zinc-100">
                      <InPlaceEdit
                        value={unit.road_number ?? ''}
                        placeholder={m.road_number()}
                        onSave={(v) =>
                          saveRollingStockIdentification(unit.id, 'roadNumber', v, unit)}
                      />
                    </div>
                  {:else}
                    <span class="truncate text-xs font-medium text-zinc-200">
                      {unit.series_code}{unit.series_name ? ` — ${unit.series_name}` : ''}
                    </span>
                    <span class="shrink-0 font-mono text-sm font-semibold text-zinc-100">
                      {unit.road_number ?? '—'}
                    </span>
                  {/if}
                </div>
                {#if unit.railway_company}
                  <span
                    class="absolute top-0 right-0 inline-flex items-center rounded-full border border-[#1F1F1F] bg-[#D48A42] px-2 py-0.5 font-mono text-[9px] font-bold tracking-wider text-[#050505] uppercase"
                  >
                    {unit.railway_company}
                  </span>
                {/if}
              </div>
              <!-- 3-column spec grid (consistent, always all fields) -->
              <dl class="grid grid-cols-3 gap-x-3 gap-y-2">
                <div class="flex flex-col gap-0.5">
                  <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                    {m.depot()}
                  </dt>
                  <dd class="text-xs text-zinc-200">
                    {#if editable}
                      <InPlaceEdit
                        value={unit.depot ?? ''}
                        placeholder={m.depot()}
                        onSave={(v) =>
                          saveRollingStockIdentification(unit.id, 'depot', v, unit)}
                      />
                    {:else}
                      {unit.depot ?? '—'}
                    {/if}
                  </dd>
                </div>
                <div class="flex flex-col gap-0.5">
                  <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                    {m.livery()}
                  </dt>
                  <dd class="truncate text-xs text-zinc-200">
                    {#if editable}
                      <InPlaceEdit
                        value={unit.livery ?? ''}
                        placeholder={m.livery()}
                        onSave={(v) =>
                          saveRollingStockIdentification(unit.id, 'livery', v, unit)}
                      />
                    {:else}
                      {unit.livery ?? '—'}
                    {/if}
                  </dd>
                </div>
                <div class="flex flex-col gap-0.5">
                  <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                    Length
                  </dt>
                  <dd class="font-mono text-xs text-zinc-200">
                    {unit.length_mm != null ? `${unit.length_mm} mm` : '—'}
                  </dd>
                </div>
                <div class="flex flex-col gap-0.5">
                  <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                    {m.control_type()}
                  </dt>
                  <dd class="text-xs text-zinc-200">
                    {#if editable && rollingStockSpecLoaded.has(unit.id)}
                      <BadgePicker
                        value={rollingStockFormState.get(unit.id)?.control ??
                          unit.control_type ??
                          '—'}
                        options={controlOptions}
                        onSelect={(id) => saveRollingStockSpec(unit.id, 'control', id)}
                      />
                    {:else if editable}
                      <span class="text-xs text-zinc-500 italic">—</span>
                    {:else}
                      {unit.control_type ?? '—'}
                    {/if}
                  </dd>
                </div>
                <div class="flex flex-col gap-0.5">
                  <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                    {m.dcc_interface()}
                  </dt>
                  <dd class="text-xs text-zinc-200">
                    {#if editable && rollingStockSpecLoaded.has(unit.id)}
                      <BadgePicker
                        value={rollingStockFormState.get(unit.id)?.dccInterface ??
                          unit.dcc_interface ??
                          '—'}
                        options={dccInterfaceOptions}
                        onSelect={(id) => saveRollingStockSpec(unit.id, 'dccInterface', id)}
                      />
                    {:else if editable}
                      <span class="text-xs text-zinc-500 italic">—</span>
                    {:else}
                      {unit.dcc_interface ?? '—'}
                    {/if}
                  </dd>
                </div>
                <div class="flex flex-col gap-0.5">
                  <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                    {m.coupling_type()}
                  </dt>
                  <dd class="text-xs text-zinc-200">
                    {#if editable && rollingStockSpecLoaded.has(unit.id)}
                      <BadgePicker
                        value={rollingStockFormState.get(unit.id)?.couplingSocket ??
                          unit.coupling_type ??
                          '—'}
                        options={couplingSocketOptions}
                        onSelect={(id) => saveRollingStockSpec(unit.id, 'couplingSocket', id)}
                      />
                    {:else if editable}
                      <span class="text-xs text-zinc-500 italic">—</span>
                    {:else}
                      {unit.coupling_type ?? '—'}
                    {/if}
                  </dd>
                </div>
              </dl>
            </div>
          {/each}
        </div>
      {/if}

      {#if editable}
        <div class="mt-4 flex justify-start">
          <button
            type="button"
            class="inline-flex items-center gap-1.5 rounded-lg border border-[#1F1F1F] bg-transparent px-3 py-1.5 text-xs font-medium text-[#E0E0E0] transition-colors hover:border-[#D48A42]/50 hover:bg-[rgba(212,138,66,0.15)] hover:text-[#D48A42]"
            onclick={() => {
              createDrawerOpen = true;
            }}
          >
            {m.rolling_stock_add_more()}
          </button>
        </div>
      {/if}
    {:else if editable}
      <div class="rounded-lg border border-dashed border-border p-8 text-center">
        <button
          type="button"
          class="inline-flex items-center gap-2 rounded-lg border border-[#1F1F1F] bg-transparent px-4 py-2 text-sm font-medium text-[#E0E0E0] transition-colors hover:border-[#D48A42]/50 hover:bg-[rgba(212,138,66,0.15)] hover:text-[#D48A42]"
          onclick={() => {
            createDrawerOpen = true;
          }}
        >
          {m.rolling_stock_add_cta()}
        </button>
      </div>
    {:else}
      <div class="rounded-lg border border-zinc-800 bg-zinc-900/40 p-4">
        <p class="text-sm text-zinc-600 italic">{m.no_additional_details()}</p>
      </div>
    {/if}
  </TabsContent>
</Tabs>

<RollingStockCreateDrawer
  open={createDrawerOpen}
  railwayModelId={model.id}
  onCreated={(_id: RollingStockId) => {
    void onModelUpdated?.();
  }}
  onClose={() => {
    createDrawerOpen = false;
  }}
/>
