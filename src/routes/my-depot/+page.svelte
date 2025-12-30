<script lang="ts">
  import { Box as BoxIcon, Search, TrainFront, TramFront, X } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import DepotSection from '$lib/features/depot/components/DepotSection.svelte';
  import LocomotiveCard from '$lib/features/depot/components/LocomotiveCard.svelte';
  import TrainCard from '$lib/features/depot/components/TrainCard.svelte';
  import CarCard from '$lib/features/depot/components/CarCard.svelte';
  import { cars, locomotives, trains } from '$lib/features/depot/depot-data';
  import { debounce } from '$lib/utils/debounce';

  let searchInput = $state('');
  let query = $state('');
  const stickyOffset = $state('var(--header-offset, 4rem)');

  const debouncedSearch = debounce((value: string) => {
    query = value.trim().toLowerCase();
  }, 150);

  function handleInput(value: string) {
    searchInput = value;
    debouncedSearch(value);
  }

  function clearSearch() {
    searchInput = '';
    query = '';
  }

  const normalizedQuery = $derived(query);

  const filterMatch = (value?: string) => {
    if (!normalizedQuery) return true;
    const text = (value ?? '').toLowerCase();
    return text.includes(normalizedQuery);
  };

  const filteredLocomotives = $derived(
    normalizedQuery
      ? locomotives.filter((item) => filterMatch(item.roadNumber) || filterMatch(item.dccAddress))
      : locomotives
  );

  const filteredTrains = $derived(
    normalizedQuery
      ? trains.filter((item) => filterMatch(item.roadNumber) || filterMatch(item.dccAddress))
      : trains
  );

  const filteredCars = $derived(
    normalizedQuery
      ? cars.filter((item) => filterMatch(item.roadNumber) || filterMatch(item.dccAddress))
      : cars
  );

  const totalFiltered = $derived(
    filteredLocomotives.length + filteredTrains.length + filteredCars.length
  );
</script>

<svelte:head>
  <title>{m.depot_title()}</title>
</svelte:head>

<div class="mx-auto max-w-4xl space-y-6 p-4 pt-4" style="--header-offset: 4rem;">
  <div class="space-y-1">
    <p class="text-sm tracking-[0.2em] text-surface-400 uppercase">{m.app_depot()}</p>
    <h1 class="h2 font-bold">{m.depot_title()}</h1>
    <p class="text-sm text-surface-400">{m.depot_subtitle()}</p>
  </div>

  <div class="rounded-xl border border-surface-700/60 bg-surface-900 p-3">
    <div class="input-group items-center gap-2">
      <Search size={18} class="text-surface-500" />
      <input
        class="w-full bg-transparent text-sm outline-none placeholder:text-surface-500"
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

  {#if totalFiltered === 0}
    <div
      class="flex flex-col items-center justify-center gap-3 rounded-xl border border-dashed border-surface-700/50 bg-surface-900 p-8 text-center"
    >
      <p class="text-lg font-semibold">{m.depot_no_results()}</p>
      <button class="variant-soft-primary btn" onclick={clearSearch}>
        {m.depot_clear_search()}
      </button>
    </div>
  {:else}
    <div class="space-y-8">
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
    </div>
  {/if}
</div>
