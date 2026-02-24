<script lang="ts">
  import { onMount } from 'svelte';
  import { TrainFront, TramFront, Users, Box as BoxIcon, Funnel, Plus } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import * as Accordion from '$lib/components/ui/accordion';
  import { Button } from '$lib/components';
  import { getDepotContext } from '$lib/features/depot/DepotState.svelte';

  // Custom Components
  import DepotControls from '$lib/features/depot/components/DepotControls.svelte';
  import DepotCategory from '$lib/features/depot/components/DepotCategory.svelte';
  import DepotStatusFooter from '$lib/features/depot/components/DepotStatusFooter.svelte';

  const depot = getDepotContext();
  let searchInput = $state('');

  // Svelte 5 Effect for debouncing search
  $effect(() => {
    const timeout = setTimeout(() => depot.setQuery(searchInput), 150);
    return () => clearTimeout(timeout);
  });

  // Data Configuration for the View
  const categoryMap = $derived([
    {
      id: '01',
      val: 'locomotives',
      title: m.depot_locomotives_title(),
      icon: TrainFront,
      data: depot.filteredLocomotives
    },
    {
      id: '02',
      val: 'railcarsEmuDmu',
      title: m.depot_railcars_and_emu_title(),
      icon: TramFront,
      data: depot.filteredRailcarsEmuDmu
    },
    {
      id: '03',
      val: 'passengerCars',
      title: m.depot_passenger_cars_title(),
      icon: Users,
      data: depot.filteredPassengerCars
    },
    {
      id: '04',
      val: 'freightCars',
      title: m.depot_freight_cars_title(),
      icon: BoxIcon,
      data: depot.filteredFreightCars
    }
  ]);

  onMount(() => {
    void depot.load();
  });
</script>

<div class="min-h-screen space-y-8 bg-[#0c0c0c] p-6 text-zinc-100">
  <header class="flex flex-col gap-6 md:flex-row md:items-end md:justify-between">
    <div class="space-y-1">
      <div class="flex items-center gap-2">
        <div class="h-1 w-8 bg-[#f59e0b]"></div>
        <span class="text-[10px] font-bold tracking-[0.2em] text-[#f59e0b] uppercase"
          >System Depot</span
        >
      </div>
      <h1 class="text-4xl font-bold tracking-tight text-white">{m.depot_title()}</h1>
    </div>
    <div class="flex gap-3">
      <Button variant="outline"><Funnel size={16} class="mr-2" /> Filter</Button>
      <Button class="bg-[#f59e0b] text-black"><Plus size={18} /> Add Model</Button>
    </div>
  </header>

  <DepotControls
    bind:searchInput
    bind:viewMode={depot.viewMode}
    onClear={() => (searchInput = '')}
  />

  {#if depot.isLoading}{:else if depot.error}{:else}
    <Accordion.Root
      type="multiple"
      value={['locomotives', 'railcarsEmuDmu', 'passengerCars', 'freightCars']}
      class="space-y-6"
    >
      {#each categoryMap as cat (cat.id)}
        <DepotCategory
          value={cat.val}
          title={cat.title}
          icon={cat.icon}
          items={cat.data}
          categoryId={`Category_${cat.id}`}
        />
      {/each}
    </Accordion.Root>

    <DepotStatusFooter total={depot.totalFiltered} />
  {/if}
</div>
