<script lang="ts">
  import { onMount } from 'svelte';
  import { TrainFront, TramFront, Users, Box as BoxIcon } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import * as Accordion from '$lib/components/ui/accordion';
  import { PageHeader } from '$lib/components';
  import { getDepotContext } from '$lib/features/depot/DepotState.svelte';

  // Custom Components
  import DepotControls from '$lib/features/depot/components/DepotControls.svelte';
  import DepotCategory from '$lib/features/depot/components/DepotCategory.svelte';
  import AddCollectionItemDrawer from '$lib/features/collection/components/AddCollectionItemDrawer.svelte';

  const depot = getDepotContext();
  let searchInput = $state('');
  let showDrawer = $state(false);

  const isDepotEmpty = $derived(
    !depot.isLoading &&
      depot.locomotives.length === 0 &&
      depot.railcarsEmuDmu.length === 0 &&
      depot.passengerCars.length === 0 &&
      depot.freightCars.length === 0
  );

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

{#snippet EmptyState()}
  <div class="space-y-4 rounded-lg border border-white/10 bg-black/20 p-4">
    <div
      class="flex flex-col items-center justify-center gap-8 rounded-3xl border border-white/5 bg-layout-surface/50 px-4 py-24 text-center"
    >
      <div class="relative">
        <div class="absolute inset-0 rounded-full bg-zinc-500/10 blur-3xl"></div>
        <div
          class="relative flex h-32 w-32 items-center justify-center rounded-full border border-white/10 bg-zinc-900/50"
        >
          <TrainFront size={56} class="text-zinc-600 opacity-50" />
        </div>
      </div>

      <div class="flex max-w-sm flex-col items-center gap-3 text-center">
        <h3 class="text-2xl font-bold text-zinc-200">
          {m.depot_empty_title()}
        </h3>
        <p class="text-sm leading-relaxed text-zinc-500">
          {m.depot_empty_caption()}
        </p>
      </div>

      <button
        type="button"
        class="group relative mt-2 inline-flex cursor-pointer items-center gap-3 overflow-hidden rounded-full bg-amber-500 px-8 py-4 font-bold tracking-wide text-black transition-all hover:scale-105 hover:bg-amber-400 hover:shadow-[0_0_20px_rgba(245,158,11,0.4)] active:scale-95"
        onclick={() => (showDrawer = true)}
      >
        <div
          class="absolute inset-0 translate-y-full bg-white/20 transition-transform duration-300 group-hover:translate-y-0"
        ></div>
        <TrainFront class="h-5 w-5" />
        <span>{m.collection_add_item()}</span>
      </button>
    </div>
  </div>
{/snippet}

<div class="flex flex-col">
  <div
    class="-mx-4 -mt-4 mb-6 border-b border-border bg-card/50 px-6 py-4 lg:-mx-8 lg:-mt-8 lg:mb-8"
  >
    <PageHeader title={m.depot_title()} subtitle={m.app_depot()} description={m.depot_subtitle()} />
  </div>

  <div class="space-y-8">
    {#if !isDepotEmpty}
      <DepotControls bind:searchInput onClear={() => (searchInput = '')} />
    {/if}

    {#if depot.isLoading}{:else if depot.error}{:else if isDepotEmpty}
      {@render EmptyState()}
    {:else}
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
    {/if}
  </div>
</div>

<AddCollectionItemDrawer
  open={showDrawer}
  onClose={() => (showDrawer = false)}
  onSuccess={() => {
    showDrawer = false;
    void depot.load();
  }}
/>
