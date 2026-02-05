<script lang="ts">
  import { onMount } from 'svelte';
  import { Plus, CalendarPlus } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { Button, PageHeader } from '$lib/components';
  import MaintenanceState, {
    setMaintenanceState
  } from '$lib/features/maintenance/MaintenanceState.svelte';
  import MaintenanceCardList from '$lib/features/maintenance/components/MaintenanceCardList.svelte';
  import EmptyMaintenanceState from '$lib/features/maintenance/components/EmptyMaintenanceState.svelte';
  import AddMaintenanceCardModal from '$lib/features/maintenance/components/AddMaintenanceCardModal.svelte';
  import AddMaintenanceEventModal from '$lib/features/maintenance/components/AddMaintenanceEventModal.svelte';

  // Initialize state
  const maintenanceState = new MaintenanceState();
  setMaintenanceState(maintenanceState);

  // Reactive bindings
  const cards = $derived(maintenanceState.cards);
  const isLoading = $derived(maintenanceState.isLoading);
  const error = $derived(maintenanceState.error);
  const hasCards = $derived(maintenanceState.hasCards);

  // Modal states (to be implemented in Phase 4 & 5)
  let showAddCardModal = $state(false);
  let showAddEventModal = $state(false);

  // Quick action handlers
  function handleAddCard() {
    showAddCardModal = true;
  }

  function handleAddEvent() {
    showAddEventModal = true;
  }

  function handleRetry() {
    void maintenanceState.retry();
  }

  onMount(() => {
    void maintenanceState.loadDashboard();
  });
</script>

<svelte:head>
  <title>{m.maintenance_title()}</title>
</svelte:head>

<div class="mx-auto max-w-4xl space-y-6 p-4 pt-4">
  <!-- Page Header -->
  <PageHeader
    title={m.maintenance_title()}
    subtitle={m.app_maintenance()}
    description={m.maintenance_subtitle()}
  />

  <!-- Quick Actions -->
  <div class="border-surface-700/60 bg-surface-900 rounded-xl border p-4">
    <p class="text-surface-400 mb-3 text-xs tracking-[0.2em] uppercase">
      {m.maintenance_quick_actions()}
    </p>
    <div class="flex flex-wrap gap-3">
      <Button type="button" variant="default" class="gap-2" onclick={handleAddCard}>
        <Plus size={18} />
        {m.maintenance_add_card_button()}
      </Button>
      <Button type="button" variant="secondary" class="gap-2" onclick={handleAddEvent}>
        <CalendarPlus size={18} />
        {m.maintenance_add_event_button()}
      </Button>
    </div>
  </div>

  <!-- Content Area -->
  <div class="space-y-4">
    {#if isLoading}
      <div class="card p-8 text-center">
        <p class="text-surface-400">{m.maintenance_loading()}</p>
      </div>
    {:else if error}
      <div class="variant-filled-error card space-y-4 p-8 text-center">
        <p class="font-semibold">{m.maintenance_error_load()}</p>
        <p class="text-sm">{error}</p>
        <Button type="button" variant="default" onclick={handleRetry}>
          {m.maintenance_error_retry()}
        </Button>
      </div>
    {:else if hasCards}
      <MaintenanceCardList {cards} />
    {:else}
      <EmptyMaintenanceState onCreate={handleAddCard} />
    {/if}
  </div>
</div>

<!-- Add Maintenance Card Modal -->
{#if showAddCardModal}
  <AddMaintenanceCardModal open={showAddCardModal} onClose={() => (showAddCardModal = false)} />
{/if}

<!-- Add Maintenance Event Modal -->
{#if showAddEventModal}
  <AddMaintenanceEventModal open={showAddEventModal} onClose={() => (showAddEventModal = false)} />
{/if}
