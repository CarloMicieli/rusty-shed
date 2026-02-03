<script lang="ts">
  import type { DashboardDepotEntry } from '$lib/bindings';
  import { Building2, TrainFront } from 'lucide-svelte';
  import StatusBadge from '$lib/components/StatusBadge.svelte';

  let { depot } = $props<{ depot: DashboardDepotEntry }>();
</script>

<div
  class="variant-filled-surface space-y-3 card border-l-4 border-surface-600 p-4 transition-colors hover:border-primary-500/50"
>
  <div class="flex items-start justify-between">
    <div class="flex-1">
      <div class="flex items-center gap-2">
        <h4 class="h4 font-bold">
          {depot.manufacturer.name ?? '—'}
          {#if depot.product_code}
            <span class="ml-1 font-mono text-primary-400">#{depot.product_code}</span>
          {/if}
        </h4>
        <StatusBadge status="in-service" />
      </div>
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
    {#if depot.railway_company}
      <span class="variant-soft-surface badge flex items-center gap-1">
        <Building2 size={12} />
        {depot.railway_company.name}
      </span>
    {/if}
  </div>
</div>
