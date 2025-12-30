<script lang="ts">
  import { Box } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import type { Car } from '../types';

  let { item: car } = $props<{ item: Car }>();
</script>

<div class="variant-hover space-y-3 card border border-surface-700/40 p-4">
  <div class="flex items-start justify-between gap-3">
    <div class="flex items-center gap-2">
      <span class="variant-filled-surface badge flex items-center gap-1">
        <Box size={14} />
      </span>
      <div class="space-y-1">
        <p class="text-xs tracking-wide text-surface-500 uppercase">{m.depot_type()}</p>
        <h3 class="text-base leading-tight font-semibold">{car.type}</h3>
        <p class="text-sm text-surface-400">{car.railwayCompany}</p>
      </div>
    </div>
  </div>

  <div class="flex flex-wrap gap-2 text-sm">
    <span class="variant-soft-primary badge">{m.depot_road_number()}: {car.roadNumber}</span>
    <span class="variant-soft-surface badge">{m.depot_livery()}: {car.livery}</span>
    {#if car.category === 'passenger' && car.serviceLevel}
      <span class="variant-soft-secondary badge">{m.depot_service_level()}: {car.serviceLevel}</span
      >
    {/if}
    <span class="variant-soft-surface badge">{m.depot_company()}: {car.railwayCompany}</span>
  </div>
</div>
