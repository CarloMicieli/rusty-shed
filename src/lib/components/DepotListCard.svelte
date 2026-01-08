<script lang="ts">
  import type { DashboardDepotEntry } from '$lib/bindings';
  import { Building2, TrainFront } from 'lucide-svelte';

  let { depot } = $props<{ depot: DashboardDepotEntry }>();
</script>

<div
  class="variant-filled-surface space-y-3 card border-l-4 border-surface-600 p-4 transition-colors hover:border-primary-500/50"
>
  <div class="flex items-start justify-between">
    <div>
      <h4 class="h4 font-bold">
        {depot.manufacturer ?? '—'}
        {#if depot.productCode}
          <span class="ml-1 font-mono text-primary-400">#{depot.productCode}</span>
        {/if}
      </h4>
      <p class="line-clamp-2 pt-1 text-sm text-surface-300">
        {depot.description ?? '—'}
      </p>
    </div>
    <span class="variant-filled-secondary badge font-bold">
      {depot.scale ?? '—'}
    </span>
  </div>

  <div class="flex flex-wrap gap-2 border-t border-surface-700/50 pt-3">
    {#if depot.category}
      <span class="variant-soft-surface badge flex items-center gap-1">
        <TrainFront size={12} />
        {depot.category}
      </span>
    {/if}
    {#if depot.railwayCompany}
      <span class="variant-soft-surface badge flex items-center gap-1">
        <Building2 size={12} />
        {depot.railwayCompany}
      </span>
    {/if}
  </div>
</div>
