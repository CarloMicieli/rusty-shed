<script lang="ts">
  import type { RollingStock } from '$lib/types/railway-model';
  import type { RailwayModelId } from '$lib/bindings';
  import BadgePicker from '$lib/components/BadgePicker.svelte';
  import InPlaceEdit from '$lib/components/InPlaceEdit.svelte';
  import RollingStockSpecsDrawer from '$lib/features/rolling-stock-edit/components/RollingStockSpecsDrawer.svelte';
  import { Settings } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages';

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
    onSaveLength
  }: Props = $props();

  let specsDrawerOpen = $state(false);
</script>

<div class="rounded-lg border border-zinc-800 bg-zinc-900/40 p-4">
  <h3 class="sr-only">{m.rolling_stock_list()}</h3>

  <!-- Header row: ID cluster (left) + category (right) -->
  <div
    class="-mx-4 -mt-4 mb-4 flex items-center gap-3 rounded-t-lg border-b border-white/10 bg-white/[0.03] px-4 py-2.5"
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
        <div class="font-mono text-sm font-bold text-zinc-100">
          <InPlaceEdit
            value={unit.road_number ?? ''}
            placeholder={m.road_number()}
            onSave={(v) => onSaveIdentification('roadNumber', v)}
          />
        </div>
      {:else}
        <span class="font-mono text-sm font-bold text-zinc-100 normal-case">
          {unit.road_number ?? '—'}
        </span>
      {/if}
    </div>
    <!-- Category ghost pill (right-aligned) -->
    {#if unit.rolling_stock_type}
      <span
        class="ml-auto rounded border border-white/10 px-2 py-0.5 text-[9px] font-medium tracking-widest text-zinc-500 uppercase"
      >
        {unit.rolling_stock_type}
      </span>
    {/if}
  </div>

  <!-- 4-column spec grid -->
  <dl class="grid grid-cols-4 gap-x-4 gap-y-3">
    <!-- Row 1: Series Code | Depot | Livery | Edit Specs -->
    <div class="flex flex-col gap-0.5">
      <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
        {m.series_code()}
      </dt>
      <dd class="text-xs text-zinc-200">
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
      <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
        {m.depot()}
      </dt>
      <dd class="text-xs text-zinc-200">
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
      <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
        {m.livery()}
      </dt>
      <dd class="text-xs text-zinc-200">
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
    <div class="flex items-center justify-end">
      {#if editable}
        <button
          type="button"
          class="inline-flex items-center gap-1.5 rounded-md border border-[#1F1F1F] bg-transparent px-3 py-1.5 text-[10px] font-bold tracking-wider text-[#808080] uppercase transition-colors hover:bg-[rgba(212,138,66,0.15)] hover:text-[#D48A42]"
          onclick={() => {
            specsDrawerOpen = true;
          }}
        >
          <Settings size={12} />
          {m.rolling_stock_edit_specs_button()}
        </button>
      {/if}
    </div>

    <!-- Row 2: Control Type | DCC Interface | Length | (spacer) -->
    <div class="flex flex-col gap-0.5">
      <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
        {m.control_type()}
      </dt>
      <dd class="text-xs text-zinc-200">
        {#if editable && specLoaded}
          <BadgePicker
            value={formState?.control ?? unit.control_type ?? '—'}
            options={controlOptions}
            onSelect={(id) => onSaveSpec('control', id)}
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
        {#if editable && specLoaded}
          <BadgePicker
            value={formState?.dccInterface ?? unit.dcc_interface ?? '—'}
            options={dccInterfaceOptions}
            onSelect={(id) => onSaveSpec('dccInterface', id)}
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
        {m.rolling_stock_field_length()}
      </dt>
      <dd class="font-mono text-xs text-zinc-200">
        {#if editable && specLoaded}
          <InPlaceEdit
            value={unit.length_mm != null ? String(unit.length_mm) : ''}
            placeholder="mm"
            onSave={onSaveLength}
          />
        {:else if editable}
          <span class="text-xs text-zinc-500 italic">Loading…</span>
        {:else}
          {unit.length_mm != null ? `${unit.length_mm} mm` : '—'}
        {/if}
      </dd>
    </div>
    <div></div>

    <!-- Row 3: Coupling Type | Close Couplers | Digital Shunting | (spacer) -->
    <div class="flex flex-col gap-0.5">
      <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
        {m.coupling_type()}
      </dt>
      <dd class="text-xs text-zinc-200">
        {#if editable && specLoaded}
          <BadgePicker
            value={formState?.couplingSocket ?? unit.coupling_type ?? '—'}
            options={couplingSocketOptions}
            onSelect={(id) => onSaveSpec('couplingSocket', id)}
          />
        {:else if editable}
          <span class="text-xs text-zinc-500 italic">Loading…</span>
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
              onSaveBoolSpec('closeCouplers', (e.target as HTMLInputElement).checked)}
          />
        {:else if editable}
          <span class="text-xs text-zinc-500 italic">Loading…</span>
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
              onSaveBoolSpec('digitalShunting', (e.target as HTMLInputElement).checked)}
          />
        {:else if editable}
          <span class="text-xs text-zinc-500 italic">Loading…</span>
        {:else}
          {unit.digital_shunting === true ? '✓' : unit.digital_shunting === false ? '✗' : '—'}
        {/if}
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
    onClose={() => {
      specsDrawerOpen = false;
    }}
  />
{/if}
