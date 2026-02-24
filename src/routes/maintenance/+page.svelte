<script lang="ts">
  import { onMount } from 'svelte';
  import { Plus, CalendarPlus, Activity, CheckCircle2, Factory } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { Button } from '$lib/components';
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

  // Modal states
  let showAddCardModal = $state(false);
  let showAddEventModal = $state(false);

  // Stats calculation
  const stats = $derived.by(() => {
    const total = cards.length;
    let upcoming = 0;
    let completed = 0;
    const now = new Date().getTime();

    cards.forEach((c) => {
      // Upcoming
      if (c.nextMaintenanceDate && new Date(c.nextMaintenanceDate).getTime() > now) {
        upcoming++;
      }
      // Completed (sum of events + last maintenance)
      completed += c.events ? c.events.length : 0;
      // Assuming lastMaintenanceDate implies at least one completion if event list is empty or partial?
      // But typically events array is the source of truth for history.
    });

    return {
      active: total,
      upcoming,
      completed
    };
  });

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

<div class="space-y-8 pb-10">
  <!-- Header & Quick Actions -->
  <div class="flex flex-col gap-6 md:flex-row md:items-center md:justify-between">
    <div class="space-y-1">
      <h1 class="text-4xl font-extrabold tracking-tight text-white">
        {m.maintenance_title()}
      </h1>
      <p class="max-w-2xl text-zinc-400">
        {m.maintenance_subtitle()}
      </p>
    </div>

    <!-- Quick Action Bar -->
    <div class="flex flex-wrap items-center gap-3">
      <button
        type="button"
        class="inline-flex items-center gap-2 rounded-lg border border-zinc-700/50 bg-zinc-800 px-5 py-2.5 font-medium text-white transition-colors hover:bg-zinc-700"
        onclick={handleAddEvent}
      >
        <CalendarPlus class="h-4 w-4" />
        {m.maintenance_add_event_button()}
      </button>

      <button
        type="button"
        class="inline-flex items-center gap-2 rounded-lg bg-amber-500 px-5 py-2.5 font-bold text-black shadow-[0_0_15px_rgba(245,158,11,0.1)] transition-all hover:-translate-y-0.5 hover:bg-amber-400 hover:shadow-[0_0_20px_rgba(245,158,11,0.3)]"
        onclick={handleAddCard}
      >
        <Plus class="h-4 w-4 stroke-[3]" />
        {m.maintenance_add_card_button()}
      </button>
    </div>
  </div>

  <!-- Workshop Progress / Stats -->
  {#if hasCards}
    <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
      <!-- Active Services -->
      <div class="flex items-center gap-4 rounded-xl border border-white/10 bg-[#0c0c0c] p-5">
        <div class="rounded-lg bg-blue-500/10 p-3">
          <Factory class="h-6 w-6 text-blue-500" />
        </div>
        <div>
          <div class="bold text-[10px] tracking-widest text-zinc-500 uppercase">
            Active Services
          </div>
          <div class="font-mono text-2xl font-bold text-white">{stats.active}</div>
        </div>
      </div>

      <!-- Upcoming Inspections -->
      <div class="flex items-center gap-4 rounded-xl border border-white/10 bg-[#0c0c0c] p-5">
        <div class="rounded-lg bg-amber-500/10 p-3">
          <Activity class="h-6 w-6 text-amber-500" />
        </div>
        <div>
          <div class="bold text-[10px] tracking-widest text-zinc-500 uppercase">
            Upcoming Inspections
          </div>
          <div class="font-mono text-2xl font-bold text-white">{stats.upcoming}</div>
        </div>
      </div>

      <!-- Completed Tasks -->
      <div class="flex items-center gap-4 rounded-xl border border-white/10 bg-[#0c0c0c] p-5">
        <div class="rounded-lg bg-emerald-500/10 p-3">
          <CheckCircle2 class="h-6 w-6 text-emerald-500" />
        </div>
        <div>
          <div class="bold text-[10px] tracking-widest text-zinc-500 uppercase">
            Completed Tasks
          </div>
          <div class="font-mono text-2xl font-bold text-white">{stats.completed}</div>
        </div>
      </div>
    </div>
  {/if}

  <!-- Content Area -->
  <div class="min-h-[400px]">
    {#if isLoading}
      <div class="flex h-64 flex-col items-center justify-center space-y-4">
        <div
          class="h-8 w-8 animate-spin rounded-full border-4 border-amber-500 border-t-transparent"
        ></div>
        <p class="font-mono text-sm text-zinc-500">{m.maintenance_loading()}</p>
      </div>
    {:else if error}
      <div class="space-y-4 rounded-xl border border-red-500/20 bg-red-500/5 p-8 text-center">
        <p class="font-bold text-red-400">{m.maintenance_error_load()}</p>
        <p class="font-mono text-sm text-zinc-400">{error}</p>
        <Button variant="outline" onclick={handleRetry}>
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
