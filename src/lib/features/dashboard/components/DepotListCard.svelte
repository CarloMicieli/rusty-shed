<script lang="ts">
  import type { DashboardDepotEntry } from '$lib/bindings';
  import { Building2, TrainFront } from 'lucide-svelte';
  import Badge from '$lib/components/shadcn/badge/Badge.svelte';

  let { depot } = $props<{ depot: DashboardDepotEntry }>();
</script>

<div
  class="variant-filled-surface card border-surface-600 hover:border-primary-500/50 space-y-3 border-l-4 p-4 transition-colors"
>
  <div class="flex items-start justify-between">
    <div>
      <h4 class="h4 font-bold">
        {depot.manufacturer.name ?? '—'}
        {#if depot.product_code}
          <span class="text-primary-400 ml-1 font-mono">#{depot.product_code}</span>
        {/if}
      </h4>
      <p class="text-surface-300 line-clamp-2 pt-1 text-sm">
        {depot.description ?? '—'}
      </p>
    </div>
    <Badge variant="secondary" class="font-bold">
      {depot.scale ?? '—'}
    </Badge>
  </div>

  <div class="border-surface-700/50 flex flex-wrap gap-2 border-t pt-3">
    {#if depot.category}
      <Badge variant="outline" class="flex items-center gap-1">
        <TrainFront size={12} />
        {depot.category}
      </Badge>
    {/if}
    {#if depot.railway_company}
      <Badge variant="outline" class="flex items-center gap-1">
        <Building2 size={12} />
        {depot.railway_company.name}
      </Badge>
    {/if}
  </div>
</div>
