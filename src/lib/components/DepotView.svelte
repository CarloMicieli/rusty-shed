<script lang="ts">
  import type { DashboardDepotEntry } from '$lib/stores/dashboardStore.svelte';
  import { _ } from 'svelte-i18n';
  import DepotTable from './DepotTable.svelte';
  import DepotListCard from './DepotListCard.svelte';
  import { PackageOpen, Plus } from 'lucide-svelte';
  import { resolve } from '$app/paths';
  import { goto } from '$app/navigation';

  // Use $props() for Svelte 5 component inputs
  let { data = [], isLoading = false } = $props<{
    data?: DashboardDepotEntry[];
    isLoading?: boolean;
  }>();

  // Use the reactive $ prefix for the i18n store to satisfy the linter
  const t = $derived($_);
</script>

{#if isLoading}
  <div class="space-y-4 lg:hidden">
    {#each Array(3) as _item, idx (idx)}
      <div
        class="skeleton h-28 w-full rounded-container"
        aria-label={`loading-depot-card-${idx}`}
      ></div>
    {/each}
  </div>

  <div
    class="hidden overflow-hidden rounded-container border border-surface-700/50 bg-surface-900/50 p-6 lg:block"
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
    class="flex flex-col items-center justify-center rounded-container border-2 border-dashed border-surface-700/60 bg-surface-800/30 p-12 text-center"
  >
    <div class="variant-soft-surface mb-4 badge-icon h-16 w-16">
      <PackageOpen size={32} class="opacity-50" />
    </div>
    <h4 class="h4 font-bold opacity-80">
      {t('dashboard.empty_depot_title') || 'Your Depot is Empty'}
    </h4>
    <p class="mt-2 max-w-xs text-sm text-surface-400">
      {t('dashboard.empty_depot_message') ||
        'Start building your collection by adding your first railway model.'}
    </p>
    <button
      class="variant-filled-primary mt-6 btn"
      onclick={() => goto(resolve('/catalogue/new-model'))}
    >
      <Plus size={18} class="mr-2" />
      {t('dashboard.add_first_model') || 'Add Model'}
    </button>
  </div>
{:else}
  <div class="space-y-4 lg:hidden">
    {#each data as depot (depot.id)}
      <DepotListCard {depot} />
    {/each}
  </div>

  <div
    class="hidden overflow-hidden rounded-container border border-surface-700/50 bg-surface-800 lg:block"
  >
    <DepotTable {data} />
  </div>
{/if}
