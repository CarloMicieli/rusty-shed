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
    unit: RollingStock;
    editable: boolean;
    formState: RsFormState | undefined;
    specLoaded: boolean;
    controlOptions: { id: string; label: string }[];
    dccInterfaceOptions: { id: string; label: string }[];
    couplingSocketOptions: { id: string; label: string }[];
    onSaveIdentification: (
      field: 'series' | 'roadNumber' | 'livery' | 'depot',
      value: string
    ) => Promise<void>;
    onSaveSpec: (field: string, value: string) => Promise<void>;
  }

  const {
    unit,
    editable,
    formState,
    specLoaded,
    controlOptions,
    dccInterfaceOptions,
    couplingSocketOptions,
    onSaveIdentification,
    onSaveSpec
  }: Props = $props();
</script>

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
            onSave={(v) => onSaveIdentification('roadNumber', v)}
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
      <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">Length</dt>
      <dd class="font-mono text-xs text-zinc-200">
        {unit.length_mm != null ? `${unit.length_mm} mm` : '—'}
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
            onSelect={(id) => onSaveSpec('couplingSocket', id)}
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
