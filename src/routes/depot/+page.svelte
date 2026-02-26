<script lang="ts">
  import { onMount } from 'svelte';
  import { TrainFront, TramFront, Users, Box as BoxIcon, Funnel, Plus } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import * as Accordion from '$lib/components/ui/accordion';
  import { Button, PageHeader } from '$lib/components';
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

<div class="flex flex-col">
  <div
    class="-mx-4 -mt-4 mb-6 border-b border-border bg-card/50 px-6 py-4 lg:-mx-8 lg:-mt-8 lg:mb-8"
  >
    <PageHeader title={m.depot_title()} subtitle={m.app_depot()} description={m.depot_subtitle()}>
      {#snippet actions()}
        <Button variant="outline" size="sm"
          ><Funnel size={16} class="mr-2" /> {m.depot_filter_button()}</Button
        >
        <Button variant="default" size="sm"
          ><Plus size={18} class="mr-2" /> {m.actions_add_railway_model()}</Button
        >
      {/snippet}
    </PageHeader>
  </div>

  <div class="space-y-8">
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
</div>
