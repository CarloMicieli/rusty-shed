<script lang="ts">
  import { onMount } from 'svelte';
  import {
    Box as BoxIcon,
    Search,
    TrainFront,
    TramFront,
    Users,
    X,
    Filter,
    Plus,
    LayoutGrid,
    List
  } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import * as Accordion from '$lib/components/ui/accordion';
  import { Badge, Button } from '$lib/components';
  import DepotTable from '$lib/features/depot/components/DepotTable.svelte';
  import { getDepotContext } from '$lib/features/depot/DepotState.svelte';
  import { debounce } from '$lib/utils/debounce';

  const depot = getDepotContext();

  let searchInput = $state('');
  let activeCategories = $state(['locomotives', 'railcarsEmuDmu', 'passengerCars', 'freightCars']);

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
  <title>{m.depot_title()} | Rusty Shed</title>
</svelte:head>

<div class="min-h-screen space-y-8 bg-[#0c0c0c] p-6 text-zinc-100">
  <!-- Header Section -->
  <header class="flex flex-col gap-6 md:flex-row md:items-end md:justify-between">
    <div class="space-y-1">
      <div class="flex items-center gap-2">
        <div class="h-1 w-8 bg-[#f59e0b]"></div>
        <span class="text-[10px] font-bold tracking-[0.2em] text-[#f59e0b] uppercase"
          >System Depot</span
        >
      </div>
      <h1 class="text-4xl font-bold tracking-tight text-white">{m.depot_title()}</h1>
      <p class="text-sm font-medium text-zinc-500">{m.depot_subtitle()}</p>
    </div>

    <div class="flex flex-wrap items-center gap-3">
      <Button
        variant="outline"
        class="border-[#f59e0b]/20 bg-transparent text-[#f59e0b] transition-all duration-300 hover:bg-[#f59e0b] hover:text-black"
      >
        <Filter size={16} class="mr-2" />
        Filter
      </Button>
      <Button
        class="bg-[#f59e0b] font-bold text-black transition-all duration-300 hover:bg-[#f59e0b]/90"
      >
        <Plus size={18} class="mr-1" />
        Add railway model
      </Button>
    </div>
  </header>

  <!-- Control Panel: Search & View Toggle -->
  <div class="grid grid-cols-1 gap-4 lg:grid-cols-4">
    <div class="lg:col-span-3">
      <div
        class="group relative flex items-center rounded-lg border border-white/10 bg-white/5 p-1 transition-all focus-within:border-[#f59e0b]/50 focus-within:bg-white/10"
      >
        <div
          class="flex h-10 w-10 items-center justify-center text-zinc-500 group-focus-within:text-[#f59e0b]"
        >
          <Search size={18} />
        </div>
        <input
          class="h-10 flex-1 bg-transparent px-2 font-mono text-sm outline-none placeholder:text-zinc-600"
          placeholder={m.depot_search_placeholder()}
          value={searchInput}
          oninput={(event) => handleInput(event.currentTarget.value)}
        />
        {#if searchInput}
          <button
            class="flex h-8 w-8 items-center justify-center rounded-md text-zinc-400 hover:bg-white/5 hover:text-white"
            onclick={clearSearch}
          >
            <X size={16} />
          </button>
        {/if}
        <div
          class="flex h-10 items-center border-l border-white/10 px-3 font-mono text-[10px] tracking-widest text-zinc-500 uppercase"
        >
          Query_Search
        </div>
      </div>
    </div>

    <div class="flex items-center justify-between rounded-lg border border-white/10 bg-white/5 p-1">
      <button
        class="flex h-10 flex-1 items-center justify-center rounded-md text-zinc-500 transition-colors hover:text-white"
        onclick={() => depot.setViewMode('grid')}
        class:bg-white-5={depot.viewMode === 'grid'}
        class:text-white={depot.viewMode === 'grid'}
      >
        <LayoutGrid size={18} />
      </button>
      <button
        class="flex h-10 flex-1 items-center justify-center rounded-md bg-white/10 text-[#f59e0b] text-zinc-500 transition-colors"
        onclick={() => depot.setViewMode('table')}
        class:bg-white-10={depot.viewMode === 'table'}
      >
        <List size={18} />
      </button>
    </div>
  </div>

  <!-- Content Area -->
  {#if isLoading}
    <div class="flex items-center justify-center py-20">
      <div class="flex flex-col items-center gap-4">
        <div class="relative">
          <div
            class="h-12 w-12 animate-spin rounded-full border-2 border-[#f59e0b]/20 border-t-[#f59e0b]"
          ></div>
          <div class="absolute inset-0 flex items-center justify-center">
            <div class="h-2 w-2 animate-pulse rounded-full bg-[#f59e0b]"></div>
          </div>
        </div>
        <p class="text-[10px] tracking-[0.3em] text-[#f59e0b] uppercase">System_Loading</p>
      </div>
    </div>
  {:else if error}
    <div class="rounded-lg border border-red-500/20 bg-red-500/5 p-8 text-center">
      <div
        class="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-full bg-red-500/10 text-red-500"
      >
        <X size={24} />
      </div>
      <h3
        class="mb-2 text-lg font-bold text-white underline decoration-red-500/50 underline-offset-4"
      >
        HARDWARE_ERROR_0xFD
      </h3>
      <p class="mb-6 text-sm text-zinc-400">{error}</p>
      <div class="flex justify-center gap-3">
        <Button
          variant="outline"
          class="border-red-500/50 text-red-400 hover:bg-red-500 hover:text-white"
          onclick={() => depot.load()}
        >
          Retry Connection
        </Button>
        <Button variant="ghost" onclick={clearSearch}>Reset System</Button>
      </div>
    </div>
  {:else if totalFiltered === 0}
    <div
      class="flex flex-col items-center justify-center rounded-lg border border-dashed border-white/10 bg-white/2 py-32"
    >
      <BoxIcon size={48} class="mb-6 text-zinc-700" />
      <p class="text-xl font-bold text-zinc-400">{m.depot_no_results()}</p>
      <p class="mt-2 mb-8 text-sm text-zinc-600">{m.depot_search_placeholder()}</p>
      <Button
        variant="outline"
        class="border-[#f59e0b]/50 text-[#f59e0b] hover:bg-[#f59e0b] hover:text-black"
        onclick={clearSearch}
      >
        Clear Filters
      </Button>
    </div>
  {:else}
    <Accordion.Root type="multiple" value={activeCategories} class="space-y-6">
      <!-- Category 1: Locomotives -->
      {#if filteredLocomotives.length > 0}
        <Accordion.Item
          value="locomotives"
          class="overflow-hidden rounded-xl border border-white/10 bg-white/2"
        >
          <Accordion.Trigger
            class="group w-full px-6 py-4 transition-all duration-300 hover:no-underline"
          >
            <div class="flex w-full items-center gap-4">
              <div
                class="flex h-10 w-10 items-center justify-center rounded-lg bg-white/5 text-[#f59e0b] transition-all group-hover:bg-[#f59e0b]/20 group-hover:shadow-[0_0_15px_rgba(245,158,11,0.2)]"
              >
                <TrainFront size={20} />
              </div>
              <div class="flex flex-col items-start gap-0.5">
                <span class="font-mono text-[10px] tracking-widest text-zinc-500 uppercase"
                  >Category_01</span
                >
                <h3 class="text-lg font-bold tracking-tight text-white">
                  {m.depot_locomotives_title()}
                </h3>
              </div>
              <div class="ml-auto flex items-center gap-3">
                <Badge
                  variant="outline"
                  class="border-[#f59e0b]/30 bg-[#f59e0b]/10 px-3 py-1 font-mono text-[#f59e0b]"
                >
                  {filteredLocomotives.length} UNITS
                </Badge>
              </div>
            </div>
          </Accordion.Trigger>
          <Accordion.Content class="border-t border-white/5 px-0 pt-0">
            <DepotTable items={filteredLocomotives} />
          </Accordion.Content>
        </Accordion.Item>
      {/if}

      <!-- Category 2: Railcars & EMU/DMU -->
      {#if filteredRailcarsEmuDmu.length > 0}
        <Accordion.Item
          value="railcarsEmuDmu"
          class="overflow-hidden rounded-xl border border-white/10 bg-white/2"
        >
          <Accordion.Trigger
            class="group w-full px-6 py-4 transition-all duration-300 hover:no-underline"
          >
            <div class="flex w-full items-center gap-4">
              <div
                class="flex h-10 w-10 items-center justify-center rounded-lg bg-white/5 text-[#f59e0b] transition-all group-hover:bg-[#f59e0b]/20 group-hover:shadow-[0_0_15px_rgba(245,158,11,0.2)]"
              >
                <TramFront size={20} />
              </div>
              <div class="flex flex-col items-start gap-0.5">
                <span class="font-mono text-[10px] tracking-widest text-zinc-500 uppercase"
                  >Category_02</span
                >
                <h3 class="text-lg font-bold tracking-tight text-white">
                  {m.depot_railcars_and_emu_title()}
                </h3>
              </div>
              <div class="ml-auto flex items-center gap-3">
                <Badge
                  variant="outline"
                  class="border-[#f59e0b]/30 bg-[#f59e0b]/10 px-3 py-1 font-mono text-[#f59e0b]"
                >
                  {filteredRailcarsEmuDmu.length} UNITS
                </Badge>
              </div>
            </div>
          </Accordion.Trigger>
          <Accordion.Content class="border-t border-white/5 px-0 pt-0">
            <DepotTable items={filteredRailcarsEmuDmu} />
          </Accordion.Content>
        </Accordion.Item>
      {/if}

      <!-- Category 3: Passenger Cars -->
      {#if filteredPassengerCars.length > 0}
        <Accordion.Item
          value="passengerCars"
          class="overflow-hidden rounded-xl border border-white/10 bg-white/2"
        >
          <Accordion.Trigger
            class="group w-full px-6 py-4 transition-all duration-300 hover:no-underline"
          >
            <div class="flex w-full items-center gap-4">
              <div
                class="flex h-10 w-10 items-center justify-center rounded-lg bg-white/5 text-[#f59e0b] transition-all group-hover:bg-[#f59e0b]/20 group-hover:shadow-[0_0_15px_rgba(245,158,11,0.2)]"
              >
                <Users size={20} />
              </div>
              <div class="flex flex-col items-start gap-0.5">
                <span class="font-mono text-[10px] tracking-widest text-zinc-500 uppercase"
                  >Category_03</span
                >
                <h3 class="text-lg font-bold tracking-tight text-white">
                  {m.depot_passenger_cars_title()}
                </h3>
              </div>
              <div class="ml-auto flex items-center gap-3">
                <Badge
                  variant="outline"
                  class="border-[#f59e0b]/30 bg-[#f59e0b]/10 px-3 py-1 font-mono text-[#f59e0b]"
                >
                  {filteredPassengerCars.length} UNITS
                </Badge>
              </div>
            </div>
          </Accordion.Trigger>
          <Accordion.Content class="border-t border-white/5 px-0 pt-0">
            <DepotTable items={filteredPassengerCars} />
          </Accordion.Content>
        </Accordion.Item>
      {/if}

      <!-- Category 4: Freight Cars -->
      {#if filteredFreightCars.length > 0}
        <Accordion.Item
          value="freightCars"
          class="overflow-hidden rounded-xl border border-white/10 bg-white/2"
        >
          <Accordion.Trigger
            class="group w-full px-6 py-4 transition-all duration-300 hover:no-underline"
          >
            <div class="flex w-full items-center gap-4">
              <div
                class="flex h-10 w-10 items-center justify-center rounded-lg bg-white/5 text-[#f59e0b] transition-all group-hover:bg-[#f59e0b]/20 group-hover:shadow-[0_0_15px_rgba(245,158,11,0.2)]"
              >
                <BoxIcon size={20} />
              </div>
              <div class="flex flex-col items-start gap-0.5">
                <span class="font-mono text-[10px] tracking-widest text-zinc-500 uppercase"
                  >Category_04</span
                >
                <h3 class="text-lg font-bold tracking-tight text-white">
                  {m.depot_freight_cars_title()}
                </h3>
              </div>
              <div class="ml-auto flex items-center gap-3">
                <Badge
                  variant="outline"
                  class="border-[#f59e0b]/30 bg-[#f59e0b]/10 px-3 py-1 font-mono text-[#f59e0b]"
                >
                  {filteredFreightCars.length} UNITS
                </Badge>
              </div>
            </div>
          </Accordion.Trigger>
          <Accordion.Content class="border-t border-white/5 px-0 pt-0">
            <DepotTable items={filteredFreightCars} />
          </Accordion.Content>
        </Accordion.Item>
      {/if}
    </Accordion.Root>

    <!-- System Status Bar -->
    <footer class="mt-12 flex items-center justify-between border-t border-white/10 pt-6">
      <div class="flex items-center gap-6">
        <div class="flex flex-col gap-0.5">
          <span class="text-[9px] font-bold tracking-[0.2em] text-zinc-600 uppercase"
            >Depot_Memory</span
          >
          <div class="flex items-center gap-2">
            <div class="h-1.5 w-32 overflow-hidden rounded-full border border-white/5 bg-zinc-900">
              <div class="h-full bg-[#f59e0b] shadow-[0_0_8px_#f59e0b]" style="width: 45%;"></div>
            </div>
            <span class="font-mono text-[10px] text-zinc-500">45%</span>
          </div>
        </div>

        <div class="h-8 w-px bg-white/5"></div>

        <div class="flex gap-4">
          <div class="flex flex-col">
            <span class="text-[9px] font-bold tracking-[0.2em] text-zinc-600 uppercase"
              >Total_Units</span
            >
            <span class="font-mono text-sm font-bold text-white">{totalFiltered}</span>
          </div>
          <div class="flex flex-col">
            <span class="text-[9px] font-bold tracking-[0.2em] text-zinc-600 uppercase"
              >Active_DCC</span
            >
            <span class="font-mono text-sm font-bold text-[#f59e0b]">— READY —</span>
          </div>
        </div>
      </div>

      <div class="flex items-center gap-2 text-zinc-600">
        <div class="h-1.5 w-1.5 animate-pulse rounded-full bg-[#f59e0b]"></div>
        <span class="text-[9px] font-bold tracking-[0.2em] uppercase">System_Online</span>
      </div>
    </footer>
  {/if}
</div>
