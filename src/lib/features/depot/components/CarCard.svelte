<script lang="ts">
  import { Box } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { Badge } from '$lib/components';
  import type { Car } from '../types';

  let { item: car } = $props<{ item: Car }>();
</script>

<div class="variant-hover card border-surface-700/40 space-y-3 border p-4">
  <div class="flex items-start justify-between gap-3">
    <div class="flex items-center gap-2">
      <Badge variant="secondary" class="flex items-center gap-1">
        <Box size={14} />
      </Badge>
      <div class="space-y-1">
        <p class="text-surface-500 text-xs tracking-wide uppercase">{m.depot_type()}</p>
        <h3 class="text-base leading-tight font-semibold">{car.type}</h3>
        <p class="text-surface-400 text-sm">{car.railwayCompany}</p>
      </div>
    </div>
  </div>

  <div class="flex flex-wrap gap-2 text-sm">
    <Badge variant="default">{m.depot_road_number()}: {car.roadNumber}</Badge>
    <Badge variant="outline">{m.depot_livery()}: {car.livery}</Badge>
    {#if car.category === 'passenger' && car.serviceLevel}
      <Badge variant="secondary">{m.depot_service_level()}: {car.serviceLevel}</Badge>
    {/if}
    <Badge variant="outline">{m.depot_company()}: {car.railwayCompany}</Badge>
  </div>
</div>
