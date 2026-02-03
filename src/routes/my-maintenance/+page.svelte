<script lang="ts">
  import { onMount } from 'svelte';
  import { Plus, CalendarPlus } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
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
  <div class="space-y-1">
    <p class="text-sm tracking-[0.2em] text-surface-400 uppercase">{m.app_maintenance()}</p>
    <h1 class="h2 font-bold">{m.maintenance_title()}</h1>
    <p class="text-sm text-surface-400">{m.maintenance_subtitle()}</p>
  </div>

  <!-- Quick Actions -->
  <div class="rounded-xl border border-surface-700/60 bg-surface-900 p-4">
    <p class="mb-3 text-xs tracking-[0.2em] text-surface-400 uppercase">
      {m.maintenance_quick_actions()}
    </p>
    <div class="flex flex-wrap gap-3">
      <button type="button" class="variant-filled-primary btn gap-2" onclick={handleAddCard}>
        <Plus size={18} />
        {m.maintenance_add_card_button()}
      </button>
      <button type="button" class="variant-filled-secondary btn gap-2" onclick={handleAddEvent}>
        <CalendarPlus size={18} />
        {m.maintenance_add_event_button()}
      </button>
    </div>
  </div>

  <!-- Content Area -->
  <div class="space-y-4">
    {#if isLoading}
      <div class="card p-8 text-center">
        <p class="text-surface-400">{m.maintenance_loading()}</p>
      </div>
    {:else if error}
      <div class="variant-filled-error space-y-4 card p-8 text-center">
        <p class="font-semibold">{m.maintenance_error_load()}</p>
        <p class="text-sm">{error}</p>
        <button type="button" class="variant-filled btn" onclick={handleRetry}>
          {m.maintenance_error_retry()}
        </button>
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
