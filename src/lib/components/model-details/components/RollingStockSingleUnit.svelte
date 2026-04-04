<script lang="ts">
  import type { RollingStock } from '$lib/types/railway-model';
  import type { RailwayModelId, RollingStockCategory } from '$lib/bindings';
  import BadgePicker from '$lib/components/BadgePicker.svelte';
  import InPlaceEdit from '$lib/components/InPlaceEdit.svelte';
  import FeatureFlagSwitch from '$lib/components/FeatureFlagSwitch.svelte';
  import RollingStockSpecsDrawer from '$lib/features/rolling-stock-edit/components/RollingStockSpecsDrawer.svelte';
  import { Settings } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages';
  import { CATEGORY_OPTIONS, getSubcategoryOptions, SERVICE_LEVEL_OPTIONS } from './constants';
  import { getFlag } from '$lib/utils/flags';

  interface RsFormState {
    seriesCode: string;
    roadNumber: string;
    livery: string;
    depot: string;
    control: string;
    dccInterface: string;
    couplingSocket: string;
    closeCouplers: boolean | null;
    digitalShunting: boolean | null;
    category: string | null;
    subcategory: string | null;
    serviceLevel: string | null;
    subcategoryFlashed: boolean;
  }

  interface Props {
    unit: RollingStock;
    editable: boolean;
    formState: RsFormState | undefined;
    specLoaded: boolean;
    railwayModelId: RailwayModelId;
    controlOptions: { id: string; label: string }[];
    dccInterfaceOptions: { id: string; label: string }[];
    couplingSocketOptions: { id: string; label: string }[];
    onSaveIdentification: (
      field: 'series' | 'roadNumber' | 'livery' | 'depot',
      value: string
    ) => Promise<void>;
    onSaveSpec: (field: string, value: string) => Promise<void>;
    onSaveBoolSpec: (
      field: 'closeCouplers' | 'digitalShunting',
      value: boolean | null
    ) => Promise<void>;
    onSaveLength: (value: string) => Promise<void>;
    onSaveCategory: (category: string) => Promise<void>;
    onSaveSubcategory: (subcategory: string) => Promise<void>;
    onSaveServiceLevel: (serviceLevel: string) => Promise<void>;
    onSpecsSaved?: () => Promise<void> | void;
  }

  const {
    unit,
    editable,
    formState,
    specLoaded,
    railwayModelId,
    controlOptions,
    dccInterfaceOptions,
    couplingSocketOptions,
    onSaveIdentification,
    onSaveSpec,
    onSaveBoolSpec,
    onSaveLength,
    onSaveCategory,
    onSaveSubcategory,
    onSaveServiceLevel,
    onSpecsSaved
  }: Props = $props();

  let specsDrawerOpen = $state(false);
</script>

<div class="rounded-lg border border-border bg-card/40 p-4">
  <h3 class="sr-only">{m.rolling_stock_list()}</h3>

  <!-- Header row: ID cluster (left) | Category • Subcategory (center) | Edit Specs (right) -->
  <div
    class="-mx-4 -mt-4 mb-4 flex items-center rounded-t-lg border-b border-border/20 bg-foreground/[0.03] px-4 py-2.5"
  >
    <!-- Left: flag + company name + road number -->
    <div class="flex flex-1 items-center gap-2">
      <span class="text-base leading-none" title={unit.country_code ?? ''}>
        {getFlag(unit.country_code)}
      </span>
      {#if unit.railway_company}
        <span class="font-bebas tracking-widest text-muted-foreground uppercase">
          {unit.railway_company}
        </span>
      {/if}
      {#if editable}
        <div class="font-mono text-sm font-bold whitespace-nowrap text-foreground">
          <InPlaceEdit
            value={unit.road_number ?? ''}
            placeholder={m.road_number()}
            onSave={(v) => onSaveIdentification('roadNumber', v)}
          />
        </div>
      {:else}
        <span class="font-mono text-sm font-bold whitespace-nowrap text-foreground normal-case">
          {unit.road_number ?? '—'}
        </span>
      {/if}
    </div>

    <!-- Center: Category • Subcategory -->
    {#if editable && specLoaded}
      {@const currentCategory = (formState?.category ??
        unit.category) as RollingStockCategory | null}
      {@const subcategoryOpts = getSubcategoryOptions(currentCategory)}
      <div class="flex items-center gap-1">
        <BadgePicker
          value={formState?.category ?? unit.category ?? ''}
          options={CATEGORY_OPTIONS}
          onSelect={onSaveCategory}
        />
        {#if subcategoryOpts.length > 0}
          <span class="text-xs text-muted-foreground">•</span>
          <div class="inline-flex items-center" class:animate-pulse={formState?.subcategoryFlashed}>
            <BadgePicker
              value={formState?.subcategory ?? ''}
              options={subcategoryOpts}
              onSelect={onSaveSubcategory}
            />
          </div>
        {/if}
      </div>
    {:else}
      {@const catLabel =
        CATEGORY_OPTIONS.find((o) => o.id === unit.category)?.label ??
        unit.rolling_stock_type ??
        ''}
      {@const subcatOpts = getSubcategoryOptions(unit.category as RollingStockCategory | null)}
      {@const subcatLabel = subcatOpts.find((o) => o.id === unit.subcategory)?.label ?? null}
      <span class="text-xs text-muted-foreground">
        {catLabel}{subcatLabel ? ` • ${subcatLabel}` : ''}
      </span>
    {/if}

    <!-- Right: Edit Specs -->
    <div class="flex flex-1 items-center justify-end">
      {#if editable}
        <button
          type="button"
          class="variant-steampunk-lever flex items-center gap-1.5 rounded-sm border border-border bg-background px-3 py-1.5 text-[10px] font-bold tracking-widest text-muted-foreground uppercase transition-all hover:border-primary/50 hover:bg-primary/5 hover:text-primary active:scale-95"
          onclick={() => {
            specsDrawerOpen = true;
          }}
        >
          <Settings size={12} />
          {m.rolling_stock_edit_specs_button()}
        </button>
      {/if}
    </div>
  </div>

  <!-- 4-column spec grid -->
  <dl class="grid grid-cols-4 gap-x-4 gap-y-3">
    <!-- Row 1: Series Code | Livery | Length | Service Level (PASSENGER_CAR only) -->
    <div class="flex flex-col gap-0.5">
      <dt class="text-[9px] font-medium tracking-wider text-muted-foreground uppercase">
        {m.series_code()}
      </dt>
      <dd class="text-xs text-foreground">
        {#if editable}
          <InPlaceEdit
            value={unit.series_code}
            placeholder={m.rolling_stock_field_series_code()}
            onSave={(v) => onSaveIdentification('series', v)}
          />
        {:else}
          {unit.series_code}{unit.series_name ? ` — ${unit.series_name}` : ''}
        {/if}
      </dd>
    </div>
    <div class="flex flex-col gap-0.5">
      <dt class="text-[9px] font-medium tracking-wider text-muted-foreground uppercase">
        {m.livery()}
      </dt>
      <dd class="text-xs text-foreground">
        {#if editable}
          <InPlaceEdit
            value={unit.livery ?? ''}
            placeholder={m.livery()}
            onSave={(v) => onSaveIdentification('livery', v)}
          />
        {:else}
          {unit.livery ?? '—'}
        {/if}
      </dd>
    </div>
    <div class="flex flex-col gap-0.5">
      <dt class="text-[9px] font-medium tracking-wider text-muted-foreground uppercase">
        {m.rolling_stock_field_length()}
      </dt>
      <dd class="font-mono text-xs text-foreground">
        {#if editable && specLoaded}
          <InPlaceEdit
            value={unit.length_mm != null ? String(unit.length_mm) : ''}
            placeholder="mm"
            onSave={onSaveLength}
          />
        {:else if editable}
          <span class="text-xs text-muted-foreground italic">Loading…</span>
        {:else}
          {unit.length_mm != null ? `${unit.length_mm} mm` : '—'}
        {/if}
      </dd>
    </div>
    <!-- Service Level: PASSENGER_CAR only, edit mode only -->
    {#if editable && specLoaded && (formState?.category ?? unit.category) === 'PASSENGER_CAR'}
      <div class="flex flex-col gap-0.5">
        <dt class="text-[9px] font-medium tracking-wider text-muted-foreground uppercase">
          {m.rolling_stock_field_service_level()}
        </dt>
        <dd class="text-xs text-foreground">
          <BadgePicker
            value={formState?.serviceLevel ?? ''}
            options={SERVICE_LEVEL_OPTIONS}
            onSelect={onSaveServiceLevel}
          />
        </dd>
      </div>
    {:else}
      <div></div>
    {/if}

    <!-- Row 2: Depot | Control Type | DCC Interface | (empty) -->
    <div class="flex flex-col gap-0.5">
      <dt class="text-[9px] font-medium tracking-wider text-muted-foreground uppercase">
        {m.depot()}
      </dt>
      <dd class="text-xs text-foreground">
        {#if editable}
          <InPlaceEdit
            value={unit.depot ?? ''}
            placeholder={m.depot()}
            onSave={(v) => onSaveIdentification('depot', v)}
          />
        {:else}
          {unit.depot ?? '—'}
        {/if}
      </dd>
    </div>
    <div class="flex flex-col gap-0.5">
      <dt class="text-[9px] font-medium tracking-wider text-muted-foreground uppercase">
        {m.control_type()}
      </dt>
      <dd class="text-xs text-foreground">
        {#if editable && specLoaded && ['LOCOMOTIVE', 'ELECTRIC_MULTIPLE_UNIT', 'RAILCAR'].includes(formState?.category ?? unit.category ?? '')}
          <BadgePicker
            value={formState?.control ?? unit.control_type ?? ''}
            options={controlOptions}
            onSelect={(id) => onSaveSpec('control', id)}
          />
        {:else if editable && !specLoaded}
          <span class="text-xs text-muted-foreground italic">Loading…</span>
        {:else}
          {unit.control_type ?? '—'}
        {/if}
      </dd>
    </div>
    <div class="flex flex-col gap-0.5">
      <dt class="text-[9px] font-medium tracking-wider text-muted-foreground uppercase">
        {m.dcc_interface()}
      </dt>
      <dd class="text-xs text-foreground">
        {#if editable && specLoaded && ['LOCOMOTIVE', 'ELECTRIC_MULTIPLE_UNIT', 'RAILCAR'].includes(formState?.category ?? unit.category ?? '')}
          <BadgePicker
            value={formState?.dccInterface ?? unit.dcc_interface ?? ''}
            options={dccInterfaceOptions}
            onSelect={(id) => onSaveSpec('dccInterface', id)}
          />
        {:else if editable && !specLoaded}
          <span class="text-xs text-muted-foreground italic">Loading…</span>
        {:else}
          {unit.dcc_interface ?? '—'}
        {/if}
      </dd>
    </div>
    <div></div>

    <!-- Row 3: Coupling Socket | Close Couplers | Digital Shunting | (empty) -->
    <div class="flex flex-col gap-0.5">
      <dt class="text-[9px] font-medium tracking-wider text-muted-foreground uppercase">
        {m.specs_drawer_field_coupling_socket()}
      </dt>
      <dd class="text-xs text-foreground">
        {#if editable && specLoaded}
          <BadgePicker
            value={formState?.couplingSocket ?? unit.coupling_type ?? '—'}
            options={couplingSocketOptions}
            onSelect={(id) => onSaveSpec('couplingSocket', id)}
          />
        {:else if editable}
          <span class="text-xs text-muted-foreground italic">Loading…</span>
        {:else}
          {unit.coupling_type ?? '—'}
        {/if}
      </dd>
    </div>
    <div class="flex flex-col gap-0.5">
      <dt class="text-[9px] font-medium tracking-wider text-muted-foreground uppercase">
        {m.rolling_stock_field_close_couplers()}
      </dt>
      <dd class="text-xs text-foreground">
        <FeatureFlagSwitch
          label={m.rolling_stock_field_close_couplers()}
          value={(formState?.closeCouplers ?? unit.close_couplers) === true
            ? 'YES'
            : (formState?.closeCouplers ?? unit.close_couplers) === false
              ? 'NO'
              : 'NOT_APPLICABLE'}
          compact={true}
          disabled={!editable || !specLoaded}
          onUpdate={(v) =>
            onSaveBoolSpec('closeCouplers', v === 'YES' ? true : v === 'NO' ? false : null)}
        />
      </dd>
    </div>
    <div class="flex flex-col gap-0.5">
      <dt class="text-[9px] font-medium tracking-wider text-muted-foreground uppercase">
        {m.rolling_stock_field_digital_shunting()}
      </dt>
      <dd class="text-xs text-foreground">
        <FeatureFlagSwitch
          label={m.rolling_stock_field_digital_shunting()}
          value={(formState?.digitalShunting ?? unit.digital_shunting) === true
            ? 'YES'
            : (formState?.digitalShunting ?? unit.digital_shunting) === false
              ? 'NO'
              : 'NOT_APPLICABLE'}
          compact={true}
          disabled={!editable || !specLoaded}
          onUpdate={(v) =>
            onSaveBoolSpec('digitalShunting', v === 'YES' ? true : v === 'NO' ? false : null)}
        />
      </dd>
    </div>
    <div></div>
  </dl>
</div>

{#if editable}
  <RollingStockSpecsDrawer
    open={specsDrawerOpen}
    {railwayModelId}
    rollingStockId={unit.id}
    ownedRollingStockId={unit.ownedRollingStockId}
    currentCouplerId={unit.currentCouplerId}
    onClose={() => {
      specsDrawerOpen = false;
    }}
    onSaved={() => onSpecsSaved?.()}
  />
{/if}
