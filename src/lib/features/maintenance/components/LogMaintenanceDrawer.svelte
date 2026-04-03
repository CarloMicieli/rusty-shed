<script lang="ts">
  import { slide } from 'svelte/transition';
  import { Wrench } from 'lucide-svelte';
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
  import {
    DrawerShell,
    DrawerHeader,
    DrawerFooter,
    createDrawerForm
  } from '$lib/components/drawer';
  import SearchableSelect from '$lib/components/SearchableSelect.svelte';

  interface Props {
    open: boolean;
    onClose: () => void;
    onSuccess?: () => void;
  }

  let { open, onClose, onSuccess }: Props = $props();

  function getTodayLocal(): string {
    const now = new Date();
    const y = now.getFullYear();
    const mo = String(now.getMonth() + 1).padStart(2, '0');
    const d = String(now.getDate()).padStart(2, '0');
    return `${y}-${mo}-${d}`;
  }

  const f = createDrawerForm({
    initial: () => ({
      selectedLocoId: null as string | null,
      datePerformed: getTodayLocal(),
      maintenanceType: null as string | null,
      notes: '',
      initialCondition: '',
      lastRunDate: getTodayLocal(),
      serviceInterval: ''
    }),
    validate: (v) => ({
      selectedLocoId: !v.selectedLocoId ? m.error_required() : undefined,
      datePerformed: !v.datePerformed ? m.error_required() : undefined
    })
  });

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

  // Searchable combobox options derived from loaded rolling stocks
  const rsOptions = $derived(
    rollingStocks.map((rs) => ({
      value: rs.id,
      label: [rs.railwayCompanyName, rs.series, rs.roadNumber].filter(Boolean).join(' - ') || rs.id
    }))
  );

  // Cross-reference selected loco against existing maintenance cards
  const existingCardForLoco = $derived(
    maintenanceCards.find((c) => c.ownedRollingStockId === f.values.selectedLocoId) ?? null
  );

  // Smart-switch: does the selected loco already have a maintenance card?
  const hasMaintenanceCard = $derived(existingCardForLoco !== null);

  const isFormValid = $derived(f.values.selectedLocoId !== null && f.values.datePerformed !== '');

  // Reset form synchronously before render so isDirty is false when the
  // drawer first becomes visible — prevents a spurious Discard dialog on
  // re-open after a prior dirty session.
  $effect.pre(() => {
    if (open) {
      f.reset();
      error = null;
    }
  });

  // Load reference data after render (async is fine here).
  $effect(() => {
    if (open) {
      void loadRollingStocks();
      void loadMaintenanceCards();
    }
  });

  // Clear error when loco selection changes
  $effect(() => {
    if (f.values.selectedLocoId) error = null;
  });

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

  async function loadMaintenanceCards() {
    const result = await commands.getMaintenanceDashboard();
    if (result.status === 'ok') {
      maintenanceCards = result.data;
    }
  }

  async function handleSubmit() {
    if (!isFormValid || !f.values.selectedLocoId || !f.values.datePerformed) return;

    isSubmitting = true;
    error = null;

    try {
      let cardId: string;

      if (hasMaintenanceCard && existingCardForLoco) {
        // Existing card: use it directly
        cardId = existingCardForLoco.id;
      } else {
        // No card yet: create one first
        const cardResult = await commands.addMaintenanceCard(f.values.selectedLocoId);
        if (cardResult.status !== 'ok') {
          throw new Error(JSON.stringify(cardResult.error));
        }
        cardId = cardResult.data;
      }

      // Build notes: prepend initialCondition when creating a new card
      const trimmedCondition = !hasMaintenanceCard ? f.values.initialCondition.trim() : '';
      let notesText = f.values.notes.trim();
      if (trimmedCondition) {
        notesText = trimmedCondition + (notesText ? '\n' + notesText : '');
      }

      // Log the maintenance event
      const eventResult = await commands.addMaintenanceEvent({
        maintenanceCardId: cardId,
        datePerformed: f.values.datePerformed,
        maintenanceType: f.values.maintenanceType,
        notes: notesText || null
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
  size="xl"
  hasChanges={f.isDirty}
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
    <!-- Subject-First Selector: single searchable combobox -->
    <div class="space-y-2">
      <label
        for="loco-select"
        class="text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase"
      >
        {m.log_maintenance_select_loco_label()}
      </label>
      {#if isLoadingRs}
        <div class="flex h-12 items-center px-4 text-sm text-muted-foreground">
          <span
            class="mr-2 h-4 w-4 animate-spin rounded-full border-2 border-border border-t-primary"
          ></span>
          {m.app_loading()}
        </div>
      {:else}
        <SearchableSelect
          id="loco-select"
          options={rsOptions}
          value={f.values.selectedLocoId ?? ''}
          placeholder={m.log_maintenance_select_loco_placeholder()}
          onSelect={(val) => {
            f.values.selectedLocoId = val || null;
          }}
        />
      {/if}
    </div>

    <!-- Dynamic form unfolding: appears after a loco is chosen -->
    {#if f.values.selectedLocoId !== null}
      <!-- Status Lamp -->
      <div transition:slide={{ duration: 200 }} class="flex items-center gap-2">
        <span
          class="h-2.5 w-2.5 rounded-full {hasMaintenanceCard
            ? 'bg-green-500 shadow-[0_0_6px_2px_rgba(34,197,94,0.4)]'
            : 'bg-amber-500 shadow-[0_0_6px_2px_rgba(245,158,11,0.4)]'}"
        ></span>
        <span
          class="text-[11px] font-semibold tracking-widest uppercase {hasMaintenanceCard
            ? 'text-green-400'
            : 'text-amber-400'}"
        >
          {hasMaintenanceCard
            ? m.log_maintenance_status_active_card()
            : m.log_maintenance_status_new_card_required()}
        </span>
      </div>

      <!-- Initialize Maintenance Card section (only when no card exists) -->
      {#if !hasMaintenanceCard}
        <div
          transition:slide={{ duration: 250 }}
          class="space-y-4 rounded-sm border-t-2 border-primary/30 bg-background/40 px-4 pt-4 pb-4"
        >
          <h3 class="font-bebas text-xl tracking-widest text-foreground uppercase">
            {m.log_maintenance_init_card_section()}
          </h3>

          <!-- Initial Condition -->
          <div class="space-y-2">
            <label
              for="initial-condition"
              class="text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase"
            >
              {m.log_maintenance_initial_condition_label()}
            </label>
            <textarea
              id="initial-condition"
              class="flex min-h-[72px] w-full resize-none rounded-md border border-border bg-card px-3 py-2 text-sm text-foreground ring-offset-background transition-all focus:border-primary/50 focus:ring-2 focus:ring-ring focus:outline-none"
              placeholder={m.log_maintenance_initial_condition_placeholder()}
              value={f.values.initialCondition}
              oninput={(e) => (f.values.initialCondition = (e.target as HTMLTextAreaElement).value)}
            ></textarea>
          </div>

          <!-- Last Run Date -->
          <div class="space-y-2">
            <label
              for="last-run-date"
              class="text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase"
            >
              {m.log_maintenance_last_run_date_label()}
            </label>
            <input
              id="last-run-date"
              type="date"
              class="flex h-10 w-full rounded-md border border-border bg-card px-3 py-2 font-mono text-sm text-foreground ring-offset-background transition-all focus:border-primary/50 focus:ring-2 focus:ring-ring focus:outline-none"
              value={f.values.lastRunDate}
              oninput={(e) => (f.values.lastRunDate = (e.target as HTMLInputElement).value)}
            />
          </div>

          <!-- Service Interval -->
          <div class="space-y-2">
            <label
              for="service-interval"
              class="text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase"
            >
              {m.log_maintenance_service_interval_label()}
            </label>
            <input
              id="service-interval"
              type="number"
              min="1"
              class="flex h-10 w-full rounded-md border border-border bg-card px-3 py-2 font-mono text-sm text-foreground ring-offset-background transition-all focus:border-primary/50 focus:ring-2 focus:ring-ring focus:outline-none"
              placeholder={m.log_maintenance_service_interval_placeholder()}
              value={f.values.serviceInterval}
              oninput={(e) => (f.values.serviceInterval = (e.target as HTMLInputElement).value)}
            />
          </div>
        </div>
      {/if}

      <!-- Add New Event section -->
      <div
        transition:slide={{ duration: 250 }}
        class="space-y-4 rounded-sm border-t-2 border-primary/30 bg-background/40 px-4 pt-4 pb-4"
      >
        <h3 class="font-bebas text-xl tracking-widest text-foreground uppercase">
          {m.log_maintenance_add_event_section()}
        </h3>

        <!-- Date Performed -->
        <div class="space-y-2">
          <label
            for="date-performed"
            class="text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase"
          >
            {m.maintenance_add_event_date_label()}
          </label>
          <input
            id="date-performed"
            type="date"
            class="flex h-10 w-full rounded-md border border-border bg-card px-3 py-2 font-mono text-sm text-foreground ring-offset-background transition-all focus:border-primary/50 focus:ring-2 focus:ring-ring focus:outline-none"
            value={f.values.datePerformed}
            oninput={(e) => (f.values.datePerformed = (e.target as HTMLInputElement).value)}
            required
          />
        </div>

        <!-- Maintenance Type -->
        <div class="space-y-2">
          <label
            for="maintenance-type"
            class="text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase"
          >
            {m.maintenance_add_event_type_label()}
          </label>
          <select
            id="maintenance-type"
            class="flex h-10 w-full rounded-md border border-border bg-card px-3 py-2 text-sm text-foreground ring-offset-background transition-all focus:border-primary/50 focus:ring-2 focus:ring-ring focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
            value={f.values.maintenanceType ?? ''}
            onchange={(e) => {
              const v = (e.target as HTMLSelectElement).value;
              f.values.maintenanceType = v || null;
            }}
          >
            <option value="">{m.maintenance_add_event_type_placeholder()}</option>
            {#each maintenanceTypes as mt (mt.value)}
              <option value={mt.value}>{mt.label}</option>
            {/each}
          </select>
        </div>

        <!-- Notes -->
        <div class="space-y-2">
          <label
            for="event-notes"
            class="text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase"
          >
            {m.maintenance_add_event_notes_label()}
          </label>
          <textarea
            id="event-notes"
            class="flex min-h-[80px] w-full resize-none rounded-md border border-border bg-card px-3 py-2 text-sm text-foreground ring-offset-background transition-all focus:border-primary/50 focus:ring-2 focus:ring-ring focus:outline-none"
            placeholder={m.maintenance_add_event_notes_placeholder()}
            value={f.values.notes}
            oninput={(e) => (f.values.notes = (e.target as HTMLTextAreaElement).value)}
          ></textarea>
        </div>
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
