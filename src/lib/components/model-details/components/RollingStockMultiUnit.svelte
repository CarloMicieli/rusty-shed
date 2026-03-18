<script lang="ts">
  import type { RollingStock } from '$lib/types/railway-model';
  import type { RollingStockCategory } from '$lib/bindings';
  import BadgePicker from '$lib/components/BadgePicker.svelte';
  import InPlaceEdit from '$lib/components/InPlaceEdit.svelte';
  import * as m from '$lib/paraglide/messages';
  import { CATEGORY_OPTIONS, getSubcategoryOptions, SERVICE_LEVEL_OPTIONS } from './constants';

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
  }

  const {
    units,
    editable,
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
    onSaveServiceLevel
  }: Props = $props();
</script>

<div class="space-y-2">
  <h3 class="sr-only">{m.rolling_stock_list()}</h3>
  {#each units as unit (unit.id)}
    {@const formState = rollingStockFormState.get(unit.id)}
    {@const specLoaded = rollingStockSpecLoaded.has(unit.id)}
    <div class="rounded-lg border border-zinc-800 bg-zinc-900/40 p-3">
      <!-- Header row: ID cluster (left) + Category • Subcategory (right) -->
      <div
        class="-mx-3 -mt-3 mb-2.5 flex items-center gap-2 rounded-t-lg border-b border-white/10 bg-white/[0.03] px-3 py-2"
      >
        <!-- ID cluster: company badge + road number -->
        <div class="flex items-center gap-2">
          {#if unit.railway_company}
            <span
              class="rounded bg-[#f0a34b] px-1.5 py-0.5 font-mono text-[10px] font-black tracking-wider text-black uppercase"
            >
              {unit.railway_company}
            </span>
          {/if}
          {#if editable}
            <div class="shrink-0 font-mono text-sm font-bold text-zinc-100">
              <InPlaceEdit
                value={unit.road_number ?? ''}
                placeholder={m.road_number()}
                onSave={(v) => onSaveIdentification(unit.id, 'roadNumber', v)}
              />
            </div>
          {:else}
            <span class="shrink-0 font-mono text-sm font-bold text-zinc-100 normal-case">
              {unit.road_number ?? '—'}
            </span>
          {/if}
        </div>

        <!-- Right: Category • Subcategory -->
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
              <span class="text-xs text-zinc-500">•</span>
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
            {@const subcatLabel = subcatOpts.find((o) => o.id === unit.subcategory)?.label ?? null}
            <span class="text-xs text-zinc-400">
              {catLabel}{subcatLabel ? ` • ${subcatLabel}` : ''}
            </span>
          {/if}
        </div>
      </div>

      <!-- 4-column spec grid -->
      <dl class="grid grid-cols-4 gap-x-4 gap-y-3">
        <!-- Row 1: Series Code | Livery | Length | Service Level (PASSENGER_CAR only) -->
        <div class="flex flex-col gap-0.5">
          <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
            {m.series_code()}
          </dt>
          <dd class="text-xs text-zinc-200">
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
          <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
            {m.livery()}
          </dt>
          <dd class="truncate text-xs text-zinc-200">
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
          <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
            {m.rolling_stock_field_length()}
          </dt>
          <dd class="font-mono text-xs text-zinc-200">
            {#if editable && specLoaded}
              <InPlaceEdit
                value={unit.length_mm != null ? String(unit.length_mm) : ''}
                placeholder="mm"
                onSave={(v) => onSaveLength(unit.id, v)}
              />
            {:else if editable}
              <span class="text-xs text-zinc-500 italic">—</span>
            {:else}
              {unit.length_mm != null ? `${unit.length_mm} mm` : '—'}
            {/if}
          </dd>
        </div>
        <!-- Service Level: PASSENGER_CAR only, edit mode only -->
        {#if editable && specLoaded && (formState?.category ?? unit.category) === 'PASSENGER_CAR'}
          <div class="flex flex-col gap-0.5">
            <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
              {m.rolling_stock_field_service_level()}
            </dt>
            <dd class="text-xs text-zinc-200">
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
          <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
            {m.depot()}
          </dt>
          <dd class="text-xs text-zinc-200">
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
          <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
            {m.control_type()}
          </dt>
          <dd class="text-xs text-zinc-200">
            {#if editable && specLoaded && ['LOCOMOTIVE', 'ELECTRIC_MULTIPLE_UNIT', 'RAILCAR'].includes(formState?.category ?? unit.category ?? '')}
              <BadgePicker
                value={formState?.control ?? unit.control_type ?? ''}
                options={controlOptions}
                onSelect={(id) => onSaveSpec(unit.id, 'control', id)}
              />
            {:else if editable && !specLoaded}
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
            {#if editable && specLoaded && ['LOCOMOTIVE', 'ELECTRIC_MULTIPLE_UNIT', 'RAILCAR'].includes(formState?.category ?? unit.category ?? '')}
              <BadgePicker
                value={formState?.dccInterface ?? unit.dcc_interface ?? ''}
                options={dccInterfaceOptions}
                onSelect={(id) => onSaveSpec(unit.id, 'dccInterface', id)}
              />
            {:else if editable && !specLoaded}
              <span class="text-xs text-zinc-500 italic">—</span>
            {:else}
              {unit.dcc_interface ?? '—'}
            {/if}
          </dd>
        </div>
        <div></div>

        <!-- Row 3: Coupling Socket | Close Couplers | Digital Shunting | (empty) -->
        <div class="flex flex-col gap-0.5">
          <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
            {m.specs_drawer_field_coupling_socket()}
          </dt>
          <dd class="text-xs text-zinc-200">
            {#if editable && specLoaded}
              <BadgePicker
                value={formState?.couplingSocket ?? unit.coupling_type ?? '—'}
                options={couplingSocketOptions}
                onSelect={(id) => onSaveSpec(unit.id, 'couplingSocket', id)}
              />
            {:else if editable}
              <span class="text-xs text-zinc-500 italic">—</span>
            {:else}
              {unit.coupling_type ?? '—'}
            {/if}
          </dd>
        </div>
        <div class="flex flex-col gap-0.5">
          <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
            {m.rolling_stock_field_close_couplers()}
          </dt>
          <dd class="text-xs text-zinc-200">
            {#if editable && specLoaded}
              <input
                type="checkbox"
                class="accent-[#D48A42]"
                checked={formState?.closeCouplers ?? unit.close_couplers ?? false}
                onchange={(e) =>
                  onSaveBoolSpec(unit.id, 'closeCouplers', (e.target as HTMLInputElement).checked)}
              />
            {:else if editable}
              <span class="text-xs text-zinc-500 italic">—</span>
            {:else}
              {unit.close_couplers === true ? '✓' : unit.close_couplers === false ? '✗' : '—'}
            {/if}
          </dd>
        </div>
        <div class="flex flex-col gap-0.5">
          <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
            {m.rolling_stock_field_digital_shunting()}
          </dt>
          <dd class="text-xs text-zinc-200">
            {#if editable && specLoaded}
              <input
                type="checkbox"
                class="accent-[#D48A42]"
                checked={formState?.digitalShunting ?? unit.digital_shunting ?? false}
                onchange={(e) =>
                  onSaveBoolSpec(
                    unit.id,
                    'digitalShunting',
                    (e.target as HTMLInputElement).checked
                  )}
              />
            {:else if editable}
              <span class="text-xs text-zinc-500 italic">—</span>
            {:else}
              {unit.digital_shunting === true ? '✓' : unit.digital_shunting === false ? '✗' : '—'}
            {/if}
          </dd>
        </div>
        <div></div>
      </dl>
    </div>
  {/each}
</div>
