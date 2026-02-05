<script lang="ts">
  import { onMount } from 'svelte';
  import {
    Box as BoxIcon,
    Search,
    TrainFront,
    TramFront,
    X,
    LayoutGrid,
    List
  } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import DepotSection from '$lib/features/depot/components/DepotSection.svelte';
  import DepotTable from '$lib/features/depot/components/DepotTable.svelte';
  import LocomotiveCard from '$lib/features/depot/components/LocomotiveCard.svelte';
  import TrainCard from '$lib/features/depot/components/TrainCard.svelte';
  import CarCard from '$lib/features/depot/components/CarCard.svelte';
  import { getDepotContext } from '$lib/features/depot/DepotState.svelte';
  import { debounce } from '$lib/utils/debounce';

  const depot = getDepotContext();

  let searchInput = $state('');
  const stickyOffset = $state('var(--header-offset, 4rem)');

  const debouncedSearch = debounce((value: string) => {
    depot.setQuery(value);
  }, 150);

  function handleInput(value: string) {
    searchInput = value;
    debouncedSearch(value);
  }

  function clearSearch() {
    searchInput = '';
    depot.clearQuery();
  }

  const filteredLocomotives = $derived(depot.filteredLocomotives);
  const filteredTrains = $derived(depot.filteredTrains);
  const filteredCars = $derived(depot.filteredCars);
  const totalFiltered = $derived(depot.totalFiltered);
  const isLoading = $derived(depot.isLoading);
  const error = $derived(depot.error);
  const viewMode = $derived(depot.viewMode);

  function handleViewModeChange(mode: 'table' | 'grid') {
    depot.setViewMode(mode);
  }

  onMount(() => {
    void depot.load();
  });
</script>

<svelte:head>
  <title>{m.depot_title()}</title>
</svelte:head>

<div class="mx-auto max-w-4xl space-y-6 p-4 pt-4" style="--header-offset: 4rem;">
  <div class="space-y-1">
    <p class="text-surface-400 text-sm tracking-[0.2em] uppercase">{m.app_depot()}</p>
    <div class="flex items-center justify-between">
      <h1 class="h2 font-bold">{m.depot_title()}</h1>
      <div
        class="border-surface-700/60 bg-surface-900 flex items-center gap-1 rounded-lg border p-1"
      >
        <button
          class="btn-icon btn-icon-sm rounded-md {viewMode === 'table'
            ? 'variant-filled bg-surface-700'
            : 'text-surface-400 hover:text-surface-200'}"
          title="Table view"
          onclick={() => handleViewModeChange('table')}
        >
          <List size={18} />
        </button>
        <button
          class="btn-icon btn-icon-sm rounded-md {viewMode === 'grid'
            ? 'variant-filled bg-surface-700'
            : 'text-surface-400 hover:text-surface-200'}"
          title="Grid view"
          onclick={() => handleViewModeChange('grid')}
        >
          <LayoutGrid size={18} />
        </button>
      </div>
    </div>
    <p class="text-surface-400 text-sm">{m.depot_subtitle()}</p>
  </div>

  <div class="border-surface-700/60 bg-surface-900 rounded-xl border p-3">
    <div class="input-group items-center gap-2">
      <Search size={18} class="text-surface-500" />
      <input
        class="placeholder:text-surface-500 w-full bg-transparent text-sm outline-none"
        placeholder={m.depot_search_placeholder()}
        value={searchInput}
        oninput={(event) => handleInput(event.currentTarget.value)}
      />
      {#if searchInput}
        <button
          class="variant-ghost-surface btn btn-sm px-2"
          aria-label={m.depot_clear_search()}
          onclick={clearSearch}
        >
          <X size={16} />
        </button>
      {/if}
    </div>
  </div>

  {#if isLoading}
    <div class="border-surface-700/60 bg-surface-900 flex items-center gap-3 rounded-xl border p-4">
      <div
        class="border-accent-400 h-4 w-4 animate-spin rounded-full border-2 border-t-transparent"
        aria-hidden="true"
      ></div>
      <p class="text-sm text-surface-300">Loading depot…</p>
    </div>
  {:else if error}
    <div
      class="flex flex-col gap-3 rounded-xl border border-amber-500/50 bg-amber-950/50 p-4 text-amber-100"
    >
      <p class="text-sm font-semibold">{error}</p>
      <div class="flex gap-2">
        <button class="variant-filled-primary btn btn-sm" onclick={() => depot.load()}>Retry</button
        >
        <button class="variant-ghost-surface btn btn-sm" onclick={clearSearch}
          >{m.depot_clear_search()}</button
        >
      </div>
    </div>
  {:else if totalFiltered === 0}
    <div
      class="border-surface-700/50 bg-surface-900 flex flex-col items-center justify-center gap-3 rounded-xl border border-dashed p-8 text-center"
    >
      <p class="text-lg font-semibold">{m.depot_no_results()}</p>
      <button class="variant-soft-primary btn" onclick={clearSearch}>
        {m.depot_clear_search()}
      </button>
    </div>
  {:else}
    <div class="space-y-8">
      {#if viewMode === 'grid'}
        <DepotSection
          title={m.depot_locomotives_title()}
          items={filteredLocomotives}
          icon={TrainFront}
          card={LocomotiveCard}
          toneClass="variant-filled-primary"
          {stickyOffset}
          emptyMessage={m.depot_empty_locomotives()}
        />

        <DepotSection
          title={m.depot_trains_title()}
          items={filteredTrains}
          icon={TramFront}
          card={TrainCard}
          toneClass="variant-filled-secondary"
          {stickyOffset}
          emptyMessage={m.depot_empty_trains()}
        />

        <DepotSection
          title={m.depot_cars_title()}
          items={filteredCars}
          icon={BoxIcon}
          card={CarCard}
          toneClass="variant-filled-surface"
          {stickyOffset}
          emptyMessage={m.depot_empty_cars()}
        />
      {:else}
        <DepotTable
          title={m.depot_locomotives_title()}
          items={filteredLocomotives}
          icon={TrainFront}
          type="locomotive"
          toneClass="variant-filled-primary"
          {stickyOffset}
          emptyMessage={m.depot_empty_locomotives()}
        />

        <DepotTable
          title={m.depot_trains_title()}
          items={filteredTrains}
          icon={TramFront}
          type="train"
          toneClass="variant-filled-secondary"
          {stickyOffset}
          emptyMessage={m.depot_empty_trains()}
        />

        <DepotTable
          title={m.depot_cars_title()}
          items={filteredCars}
          icon={BoxIcon}
          type="car"
          toneClass="variant-filled-surface"
          {stickyOffset}
          emptyMessage={m.depot_empty_cars()}
        />
      {/if}
    </div>
  {/if}
</div>
