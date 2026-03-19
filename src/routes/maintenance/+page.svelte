<script lang="ts">
  import { onMount } from 'svelte';
  import { Plus, Activity, CheckCircle2, Factory } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { Button, PageHeader } from '$lib/components';
  import MaintenanceState, {
    setMaintenanceState
  } from '$lib/features/maintenance/MaintenanceState.svelte';
  import MaintenanceCardList from '$lib/features/maintenance/components/MaintenanceCardList.svelte';
  import EmptyMaintenanceState from '$lib/features/maintenance/components/EmptyMaintenanceState.svelte';
  import AddMaintenanceCardModal from '$lib/features/maintenance/components/AddMaintenanceCardModal.svelte';
  import LogMaintenanceDrawer from '$lib/features/maintenance/components/LogMaintenanceDrawer.svelte';

  // Initialize state
  const maintenanceState = new MaintenanceState();
  setMaintenanceState(maintenanceState);

  // Reactive bindings
  const cards = $derived(maintenanceState.cards);
  const isLoading = $derived(maintenanceState.isLoading);
  const error = $derived(maintenanceState.error);
  const hasCards = $derived(maintenanceState.hasCards);

  // Modal/drawer states
  let showAddCardModal = $state(false);
  let showLogDrawer = $state(false);

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

<div class="flex flex-col">
  <div
    class="-mx-4 -mt-4 mb-6 rounded-tl-[24px] border-b border-border bg-card/50 px-6 py-4 lg:-mx-8 lg:-mt-8 lg:mb-8"
  >
    <PageHeader
      title={m.maintenance_title()}
      subtitle={m.app_maintenance()}
      description={m.maintenance_subtitle()}
    >
      {#snippet actions()}
        {#if hasCards}
          <Button
            variant="rusty"
            size="sm"
            onclick={handleAddCard}
            class="shadow-lg shadow-amber-500/10"
          >
            <Plus class="mr-2 h-4 w-4" />
            {m.maintenance_add_card_button()}
          </Button>
          <Button
            variant="rusty"
            size="sm"
            onclick={() => (showLogDrawer = true)}
            class="shadow-lg shadow-amber-500/10"
          >
            <Plus class="mr-2 h-4 w-4" />
            {m.maintenance_add_event_button()}
          </Button>
        {/if}
      {/snippet}
    </PageHeader>
  </div>

  <div class="space-y-8 pb-10">
    <!-- Workshop Progress / Stats -->
    {#if hasCards}
      <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
        <!-- Active Services -->
        <div class="flex items-center gap-4 rounded-xl border border-border bg-card p-5">
          <div class="rounded-lg bg-blue-500/10 p-3">
            <Factory class="h-6 w-6 text-blue-500" />
          </div>
          <div>
            <div class="text-[10px] font-bold tracking-widest text-muted-foreground uppercase">
              Active Services
            </div>
            <div class="font-mono text-2xl font-bold">{stats.active}</div>
          </div>
        </div>

        <!-- Upcoming Inspections -->
        <div class="flex items-center gap-4 rounded-xl border border-border bg-card p-5">
          <div class="rounded-lg bg-primary/10 p-3">
            <Activity class="h-6 w-6 text-primary" />
          </div>
          <div>
            <div class="text-[10px] font-bold tracking-widest text-muted-foreground uppercase">
              Upcoming Inspections
            </div>
            <div class="font-mono text-2xl font-bold">{stats.upcoming}</div>
          </div>
        </div>

        <!-- Completed Tasks -->
        <div class="flex items-center gap-4 rounded-xl border border-border bg-card p-5">
          <div class="rounded-lg bg-emerald-500/10 p-3">
            <CheckCircle2 class="h-6 w-6 text-emerald-500" />
          </div>
          <div>
            <div class="text-[10px] font-bold tracking-widest text-muted-foreground uppercase">
              Completed Tasks
            </div>
            <div class="font-mono text-2xl font-bold">{stats.completed}</div>
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
          <p class="font-mono text-sm text-muted-foreground">{m.maintenance_loading()}</p>
        </div>
      {:else if error}
        <div
          class="space-y-4 rounded-xl border border-destructive/30 bg-destructive/10 p-8 text-center"
        >
          <p class="font-bold text-destructive">{m.maintenance_error_load()}</p>
          <p class="font-mono text-sm text-muted-foreground">{error}</p>
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
</div>

<!-- Add Maintenance Card Modal -->
{#if showAddCardModal}
  <AddMaintenanceCardModal open={showAddCardModal} onClose={() => (showAddCardModal = false)} />
{/if}

<!-- Log Maintenance Event Drawer -->
<LogMaintenanceDrawer
  open={showLogDrawer}
  onClose={() => (showLogDrawer = false)}
  onSuccess={() => void maintenanceState.loadDashboard()}
/>
