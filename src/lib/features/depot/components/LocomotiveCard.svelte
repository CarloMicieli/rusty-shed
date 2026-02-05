<script lang="ts">
  import { Cpu, TrainFront } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { Badge } from '$lib/components';
  import type { Locomotive } from '../types';

  let { item: locomotive } = $props<{ item: Locomotive }>();
</script>

<div class="variant-hover card border-surface-700/40 space-y-3 border p-4">
  <div class="flex items-start justify-between gap-3">
    <div class="flex items-center gap-2">
      <Badge variant="default" class="flex items-center gap-1">
        <TrainFront size={14} />
      </Badge>
      <div class="space-y-1">
        <p class="text-surface-500 text-xs tracking-wide uppercase">{m.depot_group()}</p>
        <h3 class="text-base leading-tight font-semibold">{locomotive.group}</h3>
        <p class="text-surface-400 text-sm">{locomotive.railwayCompany}</p>
      </div>
    </div>
    <Badge variant="outline" class="flex items-center gap-1 font-mono text-xs">
      <Cpu size={14} />
      {m.depot_dcc_address()}: {locomotive.dccAddress}
    </Badge>
  </div>

  <div class="flex flex-wrap gap-2 text-sm">
    <Badge variant="default">{m.depot_road_number()}: {locomotive.roadNumber}</Badge>
    <Badge variant="outline">{m.depot_company()}: {locomotive.railwayCompany}</Badge>
    <Badge variant="outline">{m.depot_livery()}: {locomotive.livery}</Badge>
  </div>
</div>
