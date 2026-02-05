<script lang="ts">
  import { Cpu, TramFront } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { Badge } from '$lib/components';
  import type { TrainSet } from '../types';

  let { item: train } = $props<{ item: TrainSet }>();
</script>

<div class="variant-hover card border-surface-700/40 space-y-3 border p-4">
  <div class="flex items-start justify-between gap-3">
    <div class="flex items-center gap-2">
      <Badge variant="secondary" class="flex items-center gap-1">
        <TramFront size={14} />
      </Badge>
      <div class="space-y-1">
        <p class="text-surface-500 text-xs tracking-wide uppercase">{m.depot_group()}</p>
        <h3 class="text-base leading-tight font-semibold">{train.group}</h3>
        <p class="text-surface-400 text-sm">{train.railwayCompany}</p>
      </div>
    </div>
    <Badge variant="outline" class="flex items-center gap-1 font-mono text-xs">
      <Cpu size={14} />
      {m.depot_dcc_address()}: {train.dccAddress}
    </Badge>
  </div>

  <div class="flex flex-wrap gap-2 text-sm">
    <Badge variant="default">{m.depot_road_number()}: {train.roadNumber}</Badge>
    <Badge variant="outline">{m.depot_company()}: {train.railwayCompany}</Badge>
    <Badge variant="outline">{m.depot_livery()}: {train.livery}</Badge>
  </div>
</div>
