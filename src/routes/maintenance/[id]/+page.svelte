<script lang="ts">
  import { onMount } from 'svelte';
  import { resolve } from '$app/paths';
  import { page } from '$app/stores';
  import { ChevronLeft, CalendarPlus, Calendar, CheckCircle2, History } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { Button } from '$lib/components';
  import MaintenanceDetailState, {
    setMaintenanceDetailState
  } from '$lib/features/maintenance/MaintenanceDetailState.svelte';
  import MaintenanceEventTimeline from '$lib/features/maintenance/components/MaintenanceEventTimeline.svelte';
  import AddEventModal from '$lib/features/maintenance/components/AddEventModal.svelte';
  import { goto } from '$app/navigation';
  import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';

  const maintenanceDetailState = new MaintenanceDetailState();
  setMaintenanceDetailState(maintenanceDetailState);

  const card = $derived(maintenanceDetailState.card);
  const isLoading = $derived(maintenanceDetailState.isLoading);
  const error = $derived(maintenanceDetailState.error);

  let showAddEventModal = $state(false);

  const formatDate = (dateStr: string | null): string => {
    if (!dateStr) return 'N/A';
    const date = new Date(dateStr);
    return date
      .toLocaleDateString(regionalManager.locale, {
        year: 'numeric',
        month: '2-digit',
        day: '2-digit'
      })
      .replace(/\//g, '.');
  };

  onMount(() => {
    const id = decodeURIComponent($page.params['id'] ?? '');
    void maintenanceDetailState.loadCard(id).then(() => {
      if (!maintenanceDetailState.card && !maintenanceDetailState.isLoading) {
        void goto(resolve('/maintenance'));
      }
    });
  });
</script>

<svelte:head>
  <title>
    {card
      ? `${card.displayInfo?.manufacturerName ?? '—'} ${card.displayInfo?.productCode ?? ''}`
      : m.maintenance_title()}
  </title>
</svelte:head>

<div class="flex flex-col gap-6">
  <!-- Back navigation -->
  <a
    href={resolve('/maintenance')}
    class="mb-6 flex h-10 w-10 items-center justify-center rounded-sm border border-border bg-card text-muted-foreground transition-colors hover:bg-background hover:text-foreground"
    aria-label={m.maintenance_title()}
  >
    <ChevronLeft size={22} />
  </a>

  {#if isLoading}
    <div class="flex h-64 flex-col items-center justify-center gap-4">
      <div
        class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"
      ></div>
      <p class="font-mono text-sm text-muted-foreground">{m.maintenance_loading()}</p>
    </div>
  {:else if error}
    <div
      class="space-y-4 rounded-sm border border-destructive/30 bg-destructive/10 p-8 text-center"
    >
      <p class="font-bold text-destructive">{m.maintenance_error_load()}</p>
      <p class="font-mono text-sm text-muted-foreground">{error}</p>
      <Button
        variant="outline"
        onclick={() => {
          const id = decodeURIComponent($page.params['id'] ?? '');
          void maintenanceDetailState.loadCard(id);
        }}
      >
        {m.maintenance_error_retry()}
      </Button>
    </div>
  {:else if card}
    <!-- Card Header -->
    <div class="rounded-sm border border-border bg-card p-6">
      <div class="flex items-start justify-between gap-4">
        <div class="space-y-1">
          <div class="font-bebas text-3xl leading-none tracking-widest text-primary uppercase">
            {m.maintenance_card_title()}
          </div>

          <!-- Tier 2: Serial number row — category in white, road number in amber -->
          {#if card.displayInfo?.rollingStockCategory || card.displayInfo?.roadNumber || card.displayInfo?.seriesCode}
            <div class="font-mono text-[11px] font-bold tracking-wider">
              {#if card.displayInfo?.rollingStockCategory}
                <span class="text-foreground"
                  >{card.displayInfo.rollingStockCategory.toUpperCase()}</span
                >
              {/if}
              {#if card.displayInfo?.roadNumber}
                <span class="ml-2 text-primary">{card.displayInfo.roadNumber}</span>
              {:else if card.displayInfo?.seriesCode}
                <span class="ml-2 text-primary">{card.displayInfo.seriesCode}</span>
              {/if}
            </div>
          {/if}

          <!-- Tier 3: Subdued subtitle with manufacturer · product code -->
          {#if card.displayInfo?.manufacturerName || card.displayInfo?.productCode}
            <div class="text-xs text-muted-foreground">
              {[card.displayInfo?.manufacturerName, card.displayInfo?.productCode]
                .filter(Boolean)
                .join(' · ')}
            </div>
          {/if}
        </div>
        <div class="flex items-center gap-3">
          <Button variant="default" size="sm" onclick={() => (showAddEventModal = true)}>
            <CalendarPlus class="mr-2 h-4 w-4" />
            {m.maintenance_add_event_button()}
          </Button>
        </div>
      </div>
    </div>

    {#snippet statCard(
      label: string,
      value: string | number,
      Icon: typeof CheckCircle2,
      iconClass: string
    )}
      <div class="flex items-center gap-4 rounded-sm border border-border bg-card p-5">
        <div class="rounded-sm border border-border bg-background/50 p-3">
          <Icon class={`h-5 w-5 ${iconClass}`} />
        </div>
        <div>
          <div class="text-[10px] font-bold tracking-widest text-muted-foreground uppercase">
            {label}
          </div>
          <div class="font-mono text-lg font-bold text-foreground">{value}</div>
        </div>
      </div>
    {/snippet}

    <!-- Stats Row -->
    <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
      {@render statCard(
        m.maintenance_card_last_maintenance(),
        formatDate(card.lastMaintenanceDate),
        CheckCircle2,
        'text-primary'
      )}

      {@render statCard(
        m.maintenance_card_due_date(),
        formatDate(card.nextMaintenanceDate),
        Calendar,
        'text-primary'
      )}

      {@render statCard(
        m.maintenance_card_total_events(),
        card.events.length,
        CalendarPlus,
        'text-primary'
      )}
    </div>

    <!-- Event Timeline -->
    <div class="space-y-4">
      <div class="flex items-center gap-3">
        <History size={20} class="text-muted-foreground" />
        <h3 class="font-bebas text-lg tracking-widest text-foreground uppercase">
          {m.maintenance_history_title()}
        </h3>
      </div>
      <MaintenanceEventTimeline
        events={card.events}
        onDeleteEvent={(id) => void maintenanceDetailState.deleteEvent(id)}
      />
    </div>
  {/if}
</div>

<!-- Add Event Modal -->
{#if card && showAddEventModal}
  <AddEventModal
    open={showAddEventModal}
    maintenanceCardId={card.id}
    onClose={() => (showAddEventModal = false)}
  />
{/if}
