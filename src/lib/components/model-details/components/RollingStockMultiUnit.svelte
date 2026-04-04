<script lang="ts">
  import type { RollingStock } from '$lib/types/railway-model';
  import type { RollingStockCategory, RailwayModelId } from '$lib/bindings';
  import { Accordion as AccordionPrimitive } from 'bits-ui';
  import * as Accordion from '$lib/components/ui/accordion';
  import BadgePicker from '$lib/components/BadgePicker.svelte';
  import InPlaceEdit from '$lib/components/InPlaceEdit.svelte';
  import FeatureFlagSwitch from '$lib/components/FeatureFlagSwitch.svelte';
  import RollingStockSpecsDrawer from '$lib/features/rolling-stock-edit/components/RollingStockSpecsDrawer.svelte';
  import ChevronDown from '@lucide/svelte/icons/chevron-down';
  import { Settings } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages';
  import { CATEGORY_OPTIONS, getSubcategoryOptions, SERVICE_LEVEL_OPTIONS } from './constants';
  import { untrack } from 'svelte';

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
    units: RollingStock[];
    editable: boolean;
    railwayModelId: RailwayModelId;
    rollingStockFormState: Map<string, RsFormState>;
    rollingStockSpecLoaded: Set<string>;
    controlOptions: { id: string; label: string }[];
    dccInterfaceOptions: { id: string; label: string }[];
    couplingSocketOptions: { id: string; label: string }[];
    onSaveIdentification: (
      unitId: string,
      field: 'series' | 'roadNumber' | 'livery' | 'depot',
      value: string
    ) => Promise<void>;
    onSaveSpec: (unitId: string, field: string, value: string) => Promise<void>;
    onSaveBoolSpec: (
      unitId: string,
      field: 'closeCouplers' | 'digitalShunting',
      value: boolean | null
    ) => Promise<void>;
    onSaveLength: (unitId: string, value: string) => Promise<void>;
    onSaveCategory: (unitId: string, category: string) => Promise<void>;
    onSaveSubcategory: (unitId: string, subcategory: string) => Promise<void>;
    onSaveServiceLevel: (unitId: string, serviceLevel: string) => Promise<void>;
    onSpecsSaved?: (unitId: string) => Promise<void> | void;
  }

  const {
    units,
    editable,
    railwayModelId,
    rollingStockFormState,
    rollingStockSpecLoaded,
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

  let accordionValue = $state<string | undefined>(untrack(() => units[0]?.id));
  let specsDrawerOpenFor = $state<string | null>(null);
</script>

<Accordion.Root type="single" bind:value={accordionValue} class="space-y-2">
  <h3 class="sr-only">{m.rolling_stock_list()}</h3>
  {#each units as unit (unit.id)}
    {@const formState = rollingStockFormState.get(unit.id)}
    {@const specLoaded = rollingStockSpecLoaded.has(unit.id)}
    <Accordion.Item value={unit.id} class="rounded-lg border border-border bg-card/40">
      <!-- Custom header: trigger (toggle) + Edit Specs button as siblings -->
      <AccordionPrimitive.Header
        class="flex items-center gap-2 rounded-t-lg border-b border-border/20 bg-foreground/[0.03] px-3 py-2
               data-[state=closed]:rounded-b-lg data-[state=closed]:border-b-transparent"
      >
        <!-- Trigger spans the identity + category area + chevron -->
        <AccordionPrimitive.Trigger
          class="flex flex-1 items-center gap-2 text-left outline-none [&[data-state=open]>div.chevron-wrap>svg]:rotate-180"
        >
          <!-- ID cluster: company badge + road number -->
          <div class="flex items-center gap-2">
            {#if unit.railway_company}
              <span
                class="rounded bg-primary px-1.5 py-0.5 font-mono text-[10px] font-black tracking-wider text-primary-foreground uppercase"
              >
                {unit.railway_company}
              </span>
            {/if}
            {#if editable}
              <div class="shrink-0 font-mono text-sm font-bold text-foreground">
                <InPlaceEdit
                  value={unit.road_number ?? ''}
                  placeholder={m.road_number()}
                  onSave={(v) => onSaveIdentification(unit.id, 'roadNumber', v)}
                />
              </div>
            {:else}
              <span class="shrink-0 font-mono text-sm font-bold text-foreground normal-case">
                {unit.road_number ?? '—'}
              </span>
            {/if}
          </div>

          <!-- Category • Subcategory (right of identity, left of chevron) -->
          <div class="ml-auto flex items-center gap-1">
            {#if editable && specLoaded}
              {@const currentCategory = (formState?.category ??
                unit.category) as RollingStockCategory | null}
              {@const subcategoryOpts = getSubcategoryOptions(currentCategory)}
              <BadgePicker
                value={formState?.category ?? unit.category ?? ''}
                options={CATEGORY_OPTIONS}
                onSelect={(cat) => onSaveCategory(unit.id, cat)}
              />
              {#if subcategoryOpts.length > 0}
                <span class="text-xs text-muted-foreground">•</span>
                <div
                  class="inline-flex items-center"
                  class:animate-pulse={formState?.subcategoryFlashed}
                >
                  <BadgePicker
                    value={formState?.subcategory ?? ''}
                    options={subcategoryOpts}
                    onSelect={(sub) => onSaveSubcategory(unit.id, sub)}
                  />
                </div>
              {/if}
            {:else}
              {@const catLabel =
                CATEGORY_OPTIONS.find((o) => o.id === unit.category)?.label ??
                unit.rolling_stock_type ??
                ''}
              {@const subcatOpts = getSubcategoryOptions(
                unit.category as RollingStockCategory | null
              )}
              {@const subcatLabel =
                subcatOpts.find((o) => o.id === unit.subcategory)?.label ?? null}
              <span class="text-xs text-muted-foreground">
                {catLabel}{subcatLabel ? ` • ${subcatLabel}` : ''}
              </span>
            {/if}
          </div>

          <!-- Chevron (auto-rotates via trigger class) -->
          <div class="chevron-wrap shrink-0">
            <ChevronDown class="size-4 text-muted-foreground transition-transform duration-200" />
          </div>
        </AccordionPrimitive.Trigger>

        <!-- Edit Specs button: sibling to trigger (NOT inside it, to avoid button-in-button) -->
        {#if editable}
          <button
            type="button"
            class="inline-flex items-center gap-1.5 rounded-md border border-border bg-transparent px-3 py-1.5 text-[10px] font-bold tracking-wider text-muted-foreground uppercase transition-colors hover:bg-primary/15 hover:text-primary"
            onclick={() => {
              specsDrawerOpenFor = unit.id;
            }}
          >
            <Settings size={12} />
            {m.rolling_stock_edit_specs_button()}
          </button>
        {/if}
      </AccordionPrimitive.Header>

      <!-- Spec grid in content -->
      <Accordion.Content class="px-3 pt-2.5 pb-3">
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
                  onSave={(v) => onSaveIdentification(unit.id, 'series', v)}
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
            <dd class="truncate text-xs text-foreground">
              {#if editable}
                <InPlaceEdit
                  value={unit.livery ?? ''}
                  placeholder={m.livery()}
                  onSave={(v) => onSaveIdentification(unit.id, 'livery', v)}
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
                  onSave={(v) => onSaveLength(unit.id, v)}
                />
              {:else if editable}
                <span class="text-xs text-muted-foreground italic">—</span>
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
                  onSelect={(sl) => onSaveServiceLevel(unit.id, sl)}
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
                  onSave={(v) => onSaveIdentification(unit.id, 'depot', v)}
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
                  onSelect={(id) => onSaveSpec(unit.id, 'control', id)}
                />
              {:else if editable && !specLoaded}
                <span class="text-xs text-muted-foreground italic">—</span>
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
                  onSelect={(id) => onSaveSpec(unit.id, 'dccInterface', id)}
                />
              {:else if editable && !specLoaded}
                <span class="text-xs text-muted-foreground italic">—</span>
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
                  onSelect={(id) => onSaveSpec(unit.id, 'couplingSocket', id)}
                />
              {:else if editable}
                <span class="text-xs text-muted-foreground italic">—</span>
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
                  onSaveBoolSpec(
                    unit.id,
                    'closeCouplers',
                    v === 'YES' ? true : v === 'NO' ? false : null
                  )}
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
                  onSaveBoolSpec(
                    unit.id,
                    'digitalShunting',
                    v === 'YES' ? true : v === 'NO' ? false : null
                  )}
              />
            </dd>
          </div>
          <div></div>
        </dl>
      </Accordion.Content>
    </Accordion.Item>
  {/each}
</Accordion.Root>

<!-- Specs drawers (one per unit, lazy-rendered) -->
{#each units as unit (unit.id)}
  {#if editable}
    <RollingStockSpecsDrawer
      open={specsDrawerOpenFor === unit.id}
      {railwayModelId}
      rollingStockId={unit.id}
      ownedRollingStockId={unit.ownedRollingStockId}
      currentCouplerId={unit.currentCouplerId}
      onClose={() => {
        specsDrawerOpenFor = null;
      }}
      onSaved={() => onSpecsSaved?.(unit.id)}
    />
  {/if}
{/each}
