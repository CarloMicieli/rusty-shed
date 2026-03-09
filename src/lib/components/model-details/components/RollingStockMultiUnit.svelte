<script lang="ts">
  import type { RollingStock } from '$lib/types/railway-model';
  import BadgePicker from '$lib/components/BadgePicker.svelte';
  import InPlaceEdit from '$lib/components/InPlaceEdit.svelte';
  import * as m from '$lib/paraglide/messages';

  interface RsFormState {
    seriesCode: string;
    roadNumber: string;
    livery: string;
    depot: string;
    control: string;
    dccInterface: string;
    couplingSocket: string;
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
    onSaveSpec
  }: Props = $props();
</script>

<div class="space-y-2">
  <h3 class="sr-only">{m.rolling_stock_list()}</h3>
  {#each units as unit (unit.id)}
    {@const formState = rollingStockFormState.get(unit.id)}
    {@const specLoaded = rollingStockSpecLoaded.has(unit.id)}
    <div class="rounded-lg border border-zinc-800 bg-zinc-900/40 p-3">
      <!-- Mini-card header: series name + railway company -->
      <div class="relative mb-2.5 border-b border-zinc-800/60 pb-2">
        <div class="flex min-w-0 items-baseline gap-2 pr-16">
          {#if editable}
            <div class="truncate text-xs font-medium text-zinc-200">
              <InPlaceEdit
                value={unit.series_code}
                placeholder={m.rolling_stock_field_series_code()}
                onSave={(v) => onSaveIdentification(unit.id, 'series', v)}
              />
            </div>
            <div class="shrink-0 font-mono text-sm font-semibold text-zinc-100">
              <InPlaceEdit
                value={unit.road_number ?? ''}
                placeholder={m.road_number()}
                onSave={(v) => onSaveIdentification(unit.id, 'roadNumber', v)}
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
      <!-- 3-column spec grid -->
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
                onSave={(v) => onSaveIdentification(unit.id, 'depot', v)}
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
                onSave={(v) => onSaveIdentification(unit.id, 'livery', v)}
              />
            {:else}
              {unit.livery ?? '—'}
            {/if}
          </dd>
        </div>
        <div class="flex flex-col gap-0.5">
          <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">Length</dt>
          <dd class="font-mono text-xs text-zinc-200">
            {unit.length_mm != null ? `${unit.length_mm} mm` : '—'}
          </dd>
        </div>
        <div class="flex flex-col gap-0.5">
          <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
            {m.control_type()}
          </dt>
          <dd class="text-xs text-zinc-200">
            {#if editable && specLoaded}
              <BadgePicker
                value={formState?.control ?? unit.control_type ?? '—'}
                options={controlOptions}
                onSelect={(id) => onSaveSpec(unit.id, 'control', id)}
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
            {#if editable && specLoaded}
              <BadgePicker
                value={formState?.dccInterface ?? unit.dcc_interface ?? '—'}
                options={dccInterfaceOptions}
                onSelect={(id) => onSaveSpec(unit.id, 'dccInterface', id)}
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
      </dl>
    </div>
  {/each}
</div>
