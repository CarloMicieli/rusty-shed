<script lang="ts">
  import { onMount } from 'svelte';
  import { Box as BoxIcon, Search, TrainFront, TramFront, Users, X } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import * as Accordion from '$lib/components/ui/accordion';
  import { Badge, Button, PageHeader } from '$lib/components';
  import DepotTable from '$lib/features/depot/components/DepotTable.svelte';
  import { getDepotContext } from '$lib/features/depot/DepotState.svelte';
  import { debounce } from '$lib/utils/debounce';

  const depot = getDepotContext();

  let searchInput = $state('');

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
  const filteredRailcarsEmuDmu = $derived(depot.filteredRailcarsEmuDmu);
  const filteredPassengerCars = $derived(depot.filteredPassengerCars);
  const filteredFreightCars = $derived(depot.filteredFreightCars);
  const totalFiltered = $derived(depot.totalFiltered);
  const isLoading = $derived(depot.isLoading);
  const error = $derived(depot.error);

  onMount(() => {
    void depot.load();
  });
</script>

<svelte:head>
  <title>{m.depot_title()}</title>
</svelte:head>

<div class="space-y-6">
  <PageHeader title={m.depot_title()} subtitle={m.app_depot()} description={m.depot_subtitle()} />

  <div class="rounded-xl border border-border bg-card p-3">
    <div class="flex items-center gap-2">
      <Search size={18} class="text-muted-foreground" />
      <input
        class="flex-1 bg-transparent text-sm outline-none placeholder:text-muted-foreground"
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
    <Accordion.Root
      type="multiple"
      value={['locomotives', 'railcarsEmuDmu', 'passengerCars', 'freightCars']}
      class="space-y-4"
    >
      <!-- Category 1: Locomotives -->
      {#if filteredLocomotives.length > 0}
        <Accordion.Item value="locomotives" class="rounded-lg border border-border bg-card">
          <Accordion.Trigger
            class="bg-surface-900/95 sticky top-[var(--header-offset,4rem)] z-10 flex w-full items-center justify-between px-4 py-3 backdrop-blur-sm"
          >
            <div class="flex items-center gap-3">
              <TrainFront size={20} class="text-primary" />
              <h3 class="text-lg font-semibold">{m.depot_locomotives_title()}</h3>
              <Badge variant="secondary">{filteredLocomotives.length}</Badge>
            </div>
          </Accordion.Trigger>
          <Accordion.Content class="px-0 pt-0">
            <DepotTable items={filteredLocomotives} type="locomotive" />
          </Accordion.Content>
        </Accordion.Item>
      {/if}

      <!-- Category 2: Railcars & EMU/DMU -->
      {#if filteredRailcarsEmuDmu.length > 0}
        <Accordion.Item value="railcarsEmuDmu" class="rounded-lg border border-border bg-card">
          <Accordion.Trigger
            class="bg-surface-900/95 sticky top-[var(--header-offset,4rem)] z-10 flex w-full items-center justify-between px-4 py-3 backdrop-blur-sm"
          >
            <div class="flex items-center gap-3">
              <TramFront size={20} class="text-primary" />
              <h3 class="text-lg font-semibold">{m.depot_railcars_and_emu_title()}</h3>
              <Badge variant="secondary">{filteredRailcarsEmuDmu.length}</Badge>
            </div>
          </Accordion.Trigger>
          <Accordion.Content class="px-0 pt-0">
            <DepotTable items={filteredRailcarsEmuDmu} type="train" />
          </Accordion.Content>
        </Accordion.Item>
      {/if}

      <!-- Category 3: Passenger Cars -->
      {#if filteredPassengerCars.length > 0}
        <Accordion.Item value="passengerCars" class="rounded-lg border border-border bg-card">
          <Accordion.Trigger
            class="bg-surface-900/95 sticky top-[var(--header-offset,4rem)] z-10 flex w-full items-center justify-between px-4 py-3 backdrop-blur-sm"
          >
            <div class="flex items-center gap-3">
              <Users size={20} class="text-primary" />
              <h3 class="text-lg font-semibold">{m.depot_passenger_cars_title()}</h3>
              <Badge variant="secondary">{filteredPassengerCars.length}</Badge>
            </div>
          </Accordion.Trigger>
          <Accordion.Content class="px-0 pt-0">
            <DepotTable items={filteredPassengerCars} type="car" />
          </Accordion.Content>
        </Accordion.Item>
      {/if}

      <!-- Category 4: Freight Cars -->
      {#if filteredFreightCars.length > 0}
        <Accordion.Item value="freightCars" class="rounded-lg border border-border bg-card">
          <Accordion.Trigger
            class="bg-surface-900/95 sticky top-[var(--header-offset,4rem)] z-10 flex w-full items-center justify-between px-4 py-3 backdrop-blur-sm"
          >
            <div class="flex items-center gap-3">
              <BoxIcon size={20} class="text-primary" />
              <h3 class="text-lg font-semibold">{m.depot_freight_cars_title()}</h3>
              <Badge variant="secondary">{filteredFreightCars.length}</Badge>
            </div>
          </Accordion.Trigger>
          <Accordion.Content class="px-0 pt-0">
            <DepotTable items={filteredFreightCars} type="car" />
          </Accordion.Content>
        </Accordion.Item>
      {/if}
    </Accordion.Root>
  {/if}
</div>
