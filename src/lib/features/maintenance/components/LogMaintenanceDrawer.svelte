<script lang="ts">
  import { Wrench, AlertTriangle, Info } from 'lucide-svelte';
  import {
    commands,
    type MaintenanceType,
    type MaintenanceCardView,
    type OwnedRollingStockView,
    type CollectionView
  } from '$lib/bindings';
  import { safeInvoke } from '$lib/shared/services/TauriAdapter';
  import { toaster } from '$lib/toaster';
  import * as m from '$lib/paraglide/messages.js';
  import { DrawerShell, DrawerHeader, DrawerFooter } from '$lib/components/drawer';

  interface Props {
    open: boolean;
    onClose: () => void;
    onSuccess?: () => void;
  }

  let { open, onClose, onSuccess }: Props = $props();

  // Form state
  let mode = $state<'rolling-stock' | 'card'>('rolling-stock');
  let selectedRsId = $state<string | null>(null);
  let selectedCardId = $state<string | null>(null);
  let datePerformed = $state<string>(getTodayLocal());
  let maintenanceType = $state<string | null>(null);
  let notes = $state('');
  let isSubmitting = $state(false);
  let isLoadingRs = $state(false);
  let error = $state<string | null>(null);
  let rollingStocks = $state<OwnedRollingStockView[]>([]);
  let maintenanceCards = $state<MaintenanceCardView[]>([]);

  // Maintenance type options
  const maintenanceTypes: Array<{ value: MaintenanceType; label: string }> = [
    { value: 'WHEEL_CLEANING', label: m.maintenance_type_wheel_cleaning() },
    { value: 'TRACK_CLEANING', label: m.maintenance_type_track_cleaning() },
    { value: 'CONTACT_CLEANING', label: m.maintenance_type_contact_cleaning() },
    { value: 'LUBRICATION', label: m.maintenance_type_lubrication() },
    { value: 'GENERAL_INSPECTION', label: m.maintenance_type_inspection() },
    { value: 'DETAIL_REPAIR', label: m.maintenance_type_repair() }
  ];

  // Derived: cross-reference selected RS against loaded cards
  const existingCardForRs = $derived(
    maintenanceCards.find((c) => c.ownedRollingStockId === selectedRsId) ?? null
  );

  // Derived: selected card's display info for road number pill
  const selectedCard = $derived(maintenanceCards.find((c) => c.id === selectedCardId) ?? null);

  const hasSelection = $derived(
    (mode === 'rolling-stock' && selectedRsId !== null) ||
      (mode === 'card' && selectedCardId !== null)
  );

  const isFormValid = $derived(hasSelection && datePerformed !== '');
  const hasChanges = $derived(hasSelection || notes.trim() !== '');

  // Watch for open/close — load data
  $effect(() => {
    if (open) {
      resetForm();
      void loadRollingStocks();
      void loadMaintenanceCards();
    }
  });

  // Clear error when RS selection changes
  $effect(() => {
    if (selectedRsId) error = null;
  });

  function getTodayLocal(): string {
    const now = new Date();
    const y = now.getFullYear();
    const mo = String(now.getMonth() + 1).padStart(2, '0');
    const d = String(now.getDate()).padStart(2, '0');
    return `${y}-${mo}-${d}`;
  }

  async function loadRollingStocks() {
    isLoadingRs = true;
    try {
      const result = await safeInvoke<CollectionView>('get_collection');
      if (!result.ok) {
        throw new Error(JSON.stringify(result.error));
      }
      rollingStocks = result.data.items.flatMap((item) => item.rollingStocks);
    } catch (err) {
      console.error('[LogMaintenanceDrawer] Failed to load rolling stocks:', err);
    } finally {
      isLoadingRs = false;
    }
  }

  function formatRollingStock(rs: OwnedRollingStockView): string {
    const parts = [rs.railwayCompanyName, rs.series, rs.roadNumber].filter(Boolean);
    return parts.join(' - ') || rs.id;
  }

  async function loadMaintenanceCards() {
    const result = await commands.getMaintenanceDashboard();
    if (result.status === 'ok') {
      maintenanceCards = result.data;
    }
  }

  function formatCardLabel(card: MaintenanceCardView): string {
    const info = card.displayInfo;
    if (!info) return card.id;
    const parts = [info.seriesCode, info.roadNumber].filter(Boolean);
    return parts.join(' ') || card.id;
  }

  function resetForm() {
    mode = 'rolling-stock';
    selectedRsId = null;
    selectedCardId = null;
    datePerformed = getTodayLocal();
    maintenanceType = null;
    notes = '';
    isSubmitting = false;
    error = null;
  }

  function handleModeChange(newMode: 'rolling-stock' | 'card') {
    mode = newMode;
    selectedRsId = null;
    selectedCardId = null;
    error = null;
  }

  async function handleSubmit() {
    if (!isFormValid || !datePerformed) return;

    isSubmitting = true;
    error = null;

    try {
      let cardId: string;

      if (mode === 'rolling-stock') {
        if (!selectedRsId) return;
        // Step 1: create (or get existing) maintenance card
        const cardResult = await commands.addMaintenanceCard(selectedRsId);
        if (cardResult.status !== 'ok') {
          throw new Error(JSON.stringify(cardResult.error));
        }
        cardId = cardResult.data;
      } else {
        if (!selectedCardId) return;
        cardId = selectedCardId;
      }

      // Step 2: log the event
      const eventResult = await commands.addMaintenanceEvent({
        id: crypto.randomUUID(),
        maintenanceCardId: cardId,
        datePerformed,
        maintenanceType,
        notes: notes.trim() || null
      });
      if (eventResult.status !== 'ok') {
        throw new Error(JSON.stringify(eventResult.error));
      }

      toaster.success({
        id: crypto.randomUUID(),
        title: m.maintenance_add_event_success(),
        duration: 3000
      });

      onSuccess?.();
      onClose();
    } catch (err) {
      error = err instanceof Error ? err.message : m.maintenance_add_event_error();
      console.error('[LogMaintenanceDrawer] Submit error:', err);
    } finally {
      isSubmitting = false;
    }
  }
</script>

<DrawerShell
  {open}
  {onClose}
  size="md"
  {hasChanges}
  labelledby="log-maintenance-drawer-title"
  {error}
>
  {#snippet header({ requestClose })}
    <DrawerHeader
      id="log-maintenance-drawer-title"
      title={m.log_maintenance_drawer_title()}
      subtitle={m.log_maintenance_drawer_subtitle()}
      icon={Wrench}
      onClose={requestClose}
    />
  {/snippet}

  <div class="space-y-5">
    <!-- Mode toggle -->
    <div class="flex rounded-lg border border-white/10 bg-zinc-900 p-1">
      <button
        type="button"
        class="flex-1 rounded-md px-3 py-2 text-sm font-medium transition-colors {mode ===
        'rolling-stock'
          ? 'bg-amber-500/20 text-amber-400'
          : 'text-zinc-400 hover:text-zinc-200'}"
        onclick={() => handleModeChange('rolling-stock')}
      >
        {m.log_maintenance_mode_new_card()}
      </button>
      <button
        type="button"
        class="flex-1 rounded-md px-3 py-2 text-sm font-medium transition-colors {mode === 'card'
          ? 'bg-amber-500/20 text-amber-400'
          : 'text-zinc-400 hover:text-zinc-200'}"
        onclick={() => handleModeChange('card')}
      >
        {m.log_maintenance_mode_existing_card()}
      </button>
    </div>

    <!-- Mode A: Rolling stock selector -->
    {#if mode === 'rolling-stock'}
      <div class="space-y-2">
        <label
          for="rs-select"
          class="text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
        >
          {m.maintenance_create_card_select_rolling_stock()}
        </label>
        {#if isLoadingRs}
          <div class="flex h-10 items-center px-3 text-sm text-zinc-500">
            <span
              class="mr-2 h-4 w-4 animate-spin rounded-full border-2 border-zinc-500 border-t-transparent"
            ></span>
            {m.app_loading()}
          </div>
        {:else}
          <select
            id="rs-select"
            class="flex h-10 w-full rounded-md border border-white/10 bg-zinc-950 px-3 py-2 text-sm text-zinc-100 ring-offset-black transition-all focus:border-amber-500/50 focus:ring-2 focus:ring-amber-500 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
            value={selectedRsId ?? ''}
            onchange={(e) => {
              selectedRsId = (e.target as HTMLSelectElement).value || null;
            }}
          >
            <option value="" disabled>{m.maintenance_create_card_placeholder()}</option>
            {#each rollingStocks as rs (rs.id)}
              <option value={rs.id} class="bg-zinc-950 text-zinc-100">
                {formatRollingStock(rs)}
              </option>
            {/each}
          </select>
        {/if}

        <!-- Contextual pill after RS selection -->
        {#if selectedRsId !== null}
          {#if existingCardForRs !== null}
            <div
              class="flex items-start gap-2 rounded-lg border border-amber-500/30 bg-amber-500/10 px-3 py-2 text-xs text-amber-400"
            >
              <AlertTriangle class="mt-0.5 h-3.5 w-3.5 shrink-0" />
              <span>{m.log_maintenance_existing_card_warning()}</span>
            </div>
          {:else}
            <div
              class="flex items-start gap-2 rounded-lg border border-zinc-700 bg-zinc-800/50 px-3 py-2 text-xs text-zinc-400"
            >
              <Info class="mt-0.5 h-3.5 w-3.5 shrink-0" />
              <span>{m.log_maintenance_new_card_info()}</span>
            </div>
          {/if}
        {/if}
      </div>
    {/if}

    <!-- Mode B: Maintenance card selector -->
    {#if mode === 'card'}
      <div class="space-y-2">
        <label
          for="card-select"
          class="text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
        >
          {m.maintenance_add_event_select_card()}
        </label>
        <select
          id="card-select"
          class="flex h-10 w-full rounded-md border border-white/10 bg-zinc-950 px-3 py-2 text-sm text-zinc-100 ring-offset-black transition-all focus:border-amber-500/50 focus:ring-2 focus:ring-amber-500 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
          value={selectedCardId ?? ''}
          onchange={(e) => {
            selectedCardId = (e.target as HTMLSelectElement).value || null;
            error = null;
          }}
        >
          <option value="" disabled>{m.maintenance_add_event_card_placeholder()}</option>
          {#each maintenanceCards as card (card.id)}
            <option value={card.id} class="bg-zinc-950 text-zinc-100">
              {formatCardLabel(card)}
            </option>
          {/each}
        </select>

        <!-- Road number info pill -->
        {#if selectedCard !== null && selectedCard.displayInfo !== null}
          <div
            class="flex items-center gap-2 rounded-lg border border-zinc-700 bg-zinc-800/50 px-3 py-2 text-xs text-zinc-400"
          >
            <Info class="h-3.5 w-3.5 shrink-0" />
            <span
              >{m.log_maintenance_road_number_info()}:
              {[selectedCard.displayInfo.seriesCode, selectedCard.displayInfo.roadNumber]
                .filter(Boolean)
                .join(' ')}</span
            >
          </div>
        {/if}
      </div>
    {/if}

    <!-- Event fields: progressive disclosure after selection -->
    {#if hasSelection}
      <!-- Date Performed -->
      <div class="space-y-2">
        <label
          for="date-performed"
          class="text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
        >
          {m.maintenance_add_event_date_label()}
        </label>
        <input
          id="date-performed"
          type="date"
          class="flex h-10 w-full rounded-md border border-white/10 bg-zinc-950 px-3 py-2 text-sm text-zinc-100 ring-offset-black transition-all focus:border-amber-500/50 focus:ring-2 focus:ring-amber-500 focus:outline-none"
          value={datePerformed}
          oninput={(e) => (datePerformed = (e.target as HTMLInputElement).value)}
          required
        />
      </div>

      <!-- Maintenance Type -->
      <div class="space-y-2">
        <label
          for="maintenance-type"
          class="text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
        >
          {m.maintenance_add_event_type_label()}
        </label>
        <select
          id="maintenance-type"
          class="flex h-10 w-full rounded-md border border-white/10 bg-zinc-950 px-3 py-2 text-sm text-zinc-100 ring-offset-black transition-all focus:border-amber-500/50 focus:ring-2 focus:ring-amber-500 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
          value={maintenanceType ?? ''}
          onchange={(e) => {
            const v = (e.target as HTMLSelectElement).value;
            maintenanceType = v || null;
          }}
        >
          <option value="">{m.maintenance_add_event_type_placeholder()}</option>
          {#each maintenanceTypes as mt (mt.value)}
            <option value={mt.value} class="bg-zinc-950 text-zinc-100">{mt.label}</option>
          {/each}
        </select>
      </div>

      <!-- Notes -->
      <div class="space-y-2">
        <label
          for="event-notes"
          class="text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
        >
          {m.maintenance_add_event_notes_label()}
        </label>
        <textarea
          id="event-notes"
          class="flex min-h-[80px] w-full resize-none rounded-md border border-white/10 bg-zinc-950 px-3 py-2 text-sm text-zinc-100 ring-offset-black transition-all focus:border-amber-500/50 focus:ring-2 focus:ring-amber-500 focus:outline-none"
          placeholder={m.maintenance_add_event_notes_placeholder()}
          value={notes}
          oninput={(e) => (notes = (e.target as HTMLTextAreaElement).value)}
        ></textarea>
      </div>
    {/if}
  </div>

  {#snippet footer({ requestClose })}
    <DrawerFooter
      cancelLabel={m.maintenance_add_event_cancel()}
      submitLabel={m.log_maintenance_log_button()}
      onCancel={requestClose}
      onSubmit={handleSubmit}
      submitting={isSubmitting}
      disabled={!isFormValid}
    />
  {/snippet}
</DrawerShell>
