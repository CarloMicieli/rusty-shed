<script lang="ts">
  import type { DashboardDepotEntry } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages.js';
  import DepotTable from './DepotTable.svelte';
  import DepotListCard from './DepotListCard.svelte';
  import { PackageOpen, Plus } from 'lucide-svelte';
  import { resolve } from '$app/paths';
  import { goto } from '$app/navigation';
  import { Button } from '$lib/components';

  // Use $props() for Svelte 5 component inputs
  let { data = [], isLoading = false } = $props<{
    data?: DashboardDepotEntry[];
    isLoading?: boolean;
  }>();
</script>

{#if isLoading}
  <div class="space-y-4 lg:hidden">
    {#each Array(3) as _item, idx (idx)}
      <div
        class="skeleton rounded-container h-28 w-full"
        aria-label={`loading-depot-card-${idx}`}
      ></div>
    {/each}
  </div>

  <div
    class="rounded-container border-surface-700/50 bg-surface-900/50 hidden overflow-hidden border p-6 lg:block"
  >
    <div class="skeleton mb-6 h-8 w-1/4 rounded"></div>
    <div class="space-y-4">
      {#each Array(5) as _item, idx (idx)}
        <div class="skeleton h-10 w-full rounded-sm" aria-label={`loading-depot-row-${idx}`}></div>
      {/each}
    </div>
  </div>
{:else if !data || data.length === 0}
  <div
    class="rounded-container border-surface-700/60 bg-surface-800/30 flex flex-col items-center justify-center border-2 border-dashed p-12 text-center"
  >
    <div class="variant-soft-surface badge-icon mb-4 h-16 w-16">
      <PackageOpen size={32} class="opacity-50" />
    </div>
    <h4 class="h4 font-bold opacity-80">
      {m.dashboard_empty_depot_title()}
    </h4>
    <p class="text-surface-400 mt-2 max-w-xs text-sm">
      {m.dashboard_empty_depot_message()}
    </p>
    <Button variant="default" class="mt-6" onclick={() => goto(resolve('/catalogue/new-model'))}>
      <Plus size={18} class="mr-2" />
      {m.dashboard_add_first_model()}
    </Button>
  </div>
{:else}
  <div class="space-y-4 lg:hidden">
    {#each data as depot (depot.id)}
      <DepotListCard {depot} />
    {/each}
  </div>

  <div
    class="rounded-container border-surface-700/50 bg-surface-800 hidden overflow-hidden border lg:block"
  >
    <DepotTable {data} />
  </div>
{/if}
