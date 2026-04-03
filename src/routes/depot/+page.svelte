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
  import EmptyState from '$lib/components/EmptyState.svelte';

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

{#snippet DepotEmptyState()}
  <EmptyState
    icon={TrainFront}
    title={m.depot_empty_title()}
    description={m.depot_empty_caption()}
    ctaLabel={m.collection_add_item()}
    onCta={() => (showDrawer = true)}
  />
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
      {@render DepotEmptyState()}
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
