<script lang="ts">
  import { Cpu, TrainFront } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import type { Locomotive } from '../types';

  let { item: locomotive } = $props<{ item: Locomotive }>();
</script>

<div class="variant-hover card border-surface-700/40 space-y-3 border p-4">
  <div class="flex items-start justify-between gap-3">
    <div class="flex items-center gap-2">
      <span class="variant-filled-primary badge flex items-center gap-1">
        <TrainFront size={14} />
      </span>
      <div class="space-y-1">
        <p class="text-surface-500 text-xs tracking-wide uppercase">{m.depot_group()}</p>
        <h3 class="text-base leading-tight font-semibold">{locomotive.group}</h3>
        <p class="text-surface-400 text-sm">{locomotive.railwayCompany}</p>
      </div>
    </div>
    <span class="variant-soft-warning badge flex items-center gap-1 font-mono text-xs">
      <Cpu size={14} />
      {m.depot_dcc_address()}: {locomotive.dccAddress}
    </span>
  </div>

  <div class="flex flex-wrap gap-2 text-sm">
    <span class="variant-soft-primary badge">{m.depot_road_number()}: {locomotive.roadNumber}</span>
    <span class="variant-soft-surface badge">{m.depot_company()}: {locomotive.railwayCompany}</span>
    <span class="variant-soft-surface badge">{m.depot_livery()}: {locomotive.livery}</span>
  </div>
</div>
