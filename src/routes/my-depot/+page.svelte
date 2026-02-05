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
  import { Button } from '$lib/components';
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

<div class="space-y-6">
  <div class="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
    <div>
      <p class="text-sm tracking-[0.2em] text-muted-foreground uppercase">{m.app_depot()}</p>
      <h1 class="h2 font-bold">{m.depot_title()}</h1>
      <p class="text-sm text-muted-foreground">{m.depot_subtitle()}</p>
    </div>
    <div class="flex flex-col gap-3 md:flex-row md:items-center">
      <div class="flex items-center gap-1 rounded-lg border border-border bg-card p-1">
        <button
          class="btn-icon btn-icon-sm rounded-md {viewMode === 'table'
            ? 'bg-sidebar-accent text-sidebar-foreground'
            : 'text-muted-foreground hover:text-foreground'}"
          title="Table view"
          onclick={() => handleViewModeChange('table')}
        >
          <List size={18} />
        </button>
        <button
          class="btn-icon btn-icon-sm rounded-md {viewMode === 'grid'
            ? 'bg-sidebar-accent text-sidebar-foreground'
            : 'text-muted-foreground hover:text-foreground'}"
          title="Grid view"
          onclick={() => handleViewModeChange('grid')}
        >
          <LayoutGrid size={18} />
        </button>
      </div>
    </div>
  </div>

  <div class="rounded-xl border border-border bg-card p-3">
    <div class="input-group items-center gap-2">
      <Search size={18} class="text-muted-foreground" />
      <input
        class="w-full bg-transparent text-sm outline-none placeholder:text-muted-foreground"
        placeholder={m.depot_search_placeholder()}
        value={searchInput}
        oninput={(event) => handleInput(event.currentTarget.value)}
      />
      {#if searchInput}
        <Button
          variant="ghost"
          size="sm"
          class="px-2"
          aria-label={m.depot_clear_search()}
          onclick={clearSearch}
        >
          <X size={16} />
        </Button>
      {/if}
    </div>
  </div>

  {#if isLoading}
    <div class="flex items-center gap-3 rounded-xl border border-border bg-card p-4">
      <div
        class="h-4 w-4 animate-spin rounded-full border-2 border-primary border-t-transparent"
        aria-hidden="true"
      ></div>
      <p class="text-sm text-muted-foreground">Loading depot…</p>
    </div>
  {:else if error}
    <div
      class="flex flex-col gap-3 rounded-xl border border-amber-500/50 bg-amber-950/50 p-4 text-amber-100"
    >
      <p class="text-sm font-semibold">{error}</p>
      <div class="flex gap-2">
        <Button variant="default" size="sm" onclick={() => depot.load()}>Retry</Button>
        <Button variant="ghost" size="sm" onclick={clearSearch}>{m.depot_clear_search()}</Button>
      </div>
    </div>
  {:else if totalFiltered === 0}
    <div
      class="flex flex-col items-center justify-center gap-3 rounded-xl border border-dashed border-border bg-card p-8 text-center"
    >
      <p class="text-lg font-semibold">{m.depot_no_results()}</p>
      <Button variant="outline" onclick={clearSearch}>
        {m.depot_clear_search()}
      </Button>
    </div>
  {:else}
    <div class="space-y-8">
      {#if viewMode === 'grid'}
        <DepotSection
          title={m.depot_locomotives_title()}
          items={filteredLocomotives}
          icon={TrainFront}
          card={LocomotiveCard}
          toneClass="default"
          {stickyOffset}
          emptyMessage={m.depot_empty_locomotives()}
        />

        <DepotSection
          title={m.depot_trains_title()}
          items={filteredTrains}
          icon={TramFront}
          card={TrainCard}
          toneClass="secondary"
          {stickyOffset}
          emptyMessage={m.depot_empty_trains()}
        />

        <DepotSection
          title={m.depot_cars_title()}
          items={filteredCars}
          icon={BoxIcon}
          card={CarCard}
          toneClass="outline"
          {stickyOffset}
          emptyMessage={m.depot_empty_cars()}
        />
      {:else}
        <DepotTable
          title={m.depot_locomotives_title()}
          items={filteredLocomotives}
          icon={TrainFront}
          type="locomotive"
          toneClass="default"
          {stickyOffset}
          emptyMessage={m.depot_empty_locomotives()}
        />

        <DepotTable
          title={m.depot_trains_title()}
          items={filteredTrains}
          icon={TramFront}
          type="train"
          toneClass="secondary"
          {stickyOffset}
          emptyMessage={m.depot_empty_trains()}
        />

        <DepotTable
          title={m.depot_cars_title()}
          items={filteredCars}
          icon={BoxIcon}
          type="car"
          toneClass="outline"
          {stickyOffset}
          emptyMessage={m.depot_empty_cars()}
        />
      {/if}
    </div>
  {/if}
</div>
