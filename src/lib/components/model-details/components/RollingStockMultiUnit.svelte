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
    closeCouplers: boolean | null;
    digitalShunting: boolean | null;
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
    onSaveLength
  }: Props = $props();
</script>

<div class="space-y-2">
  <h3 class="sr-only">{m.rolling_stock_list()}</h3>
  {#each units as unit (unit.id)}
    {@const formState = rollingStockFormState.get(unit.id)}
    {@const specLoaded = rollingStockSpecLoaded.has(unit.id)}
    <div class="rounded-lg border border-zinc-800 bg-zinc-900/40 p-3">
      <!-- Header row: ID cluster (left) + category (right) -->
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
        <!-- Category ghost pill (right-aligned) -->
        {#if unit.rolling_stock_type}
          <span
            class="ml-auto rounded border border-white/10 px-2 py-0.5 text-[9px] font-medium tracking-widest text-zinc-500 uppercase"
          >
            {unit.rolling_stock_type}
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
      </dl>
    </div>
  {/each}
</div>
