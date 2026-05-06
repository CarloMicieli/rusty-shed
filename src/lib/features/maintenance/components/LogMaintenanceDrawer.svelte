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
  import { superForm } from 'sveltekit-superforms';
  import { zod } from '$lib/vendor/superforms-adapters';
  import { maintenanceSchema } from '$lib/schemas/maintenance-form';
  import { DrawerShell, DrawerHeader, DrawerFooter } from '$lib/components/drawer';
  import SearchableSelect from '$lib/components/SearchableSelect.svelte';
  import { getMaintenanceTypes } from '../utils/maintenanceTypes';

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

  function createInitialFormState() {
    return {
      selectedLocoId: null as string | null,
      datePerformed: getTodayLocal(),
      maintenanceType: null as string | null,
      notes: '',
      initialCondition: '',
      nextMaintenanceDate: null as string | null
    };
  }

  let isSubmitting = $state(false);
  let isLoadingRs = $state(false);
  let error = $state<string | null>(null);
  let rollingStocks = $state<OwnedRollingStockView[]>([]);
  let maintenanceCards = $state<MaintenanceCardView[]>([]);
  let formEl: HTMLFormElement | undefined = $state();

  const { form, errors, tainted, enhance, reset, isTainted } = superForm(
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    createInitialFormState() as any,
    {
      SPA: true,
      dataType: 'json',
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      validators: zod(maintenanceSchema as any),
      onUpdate: async ({ form: fd }) => {
        if (!fd.valid) return;

        isSubmitting = true;
        error = null;

        try {
          let cardId: string;
          const locoId = $form.selectedLocoId as string;

          if (hasMaintenanceCard && existingCardForLoco) {
            cardId = existingCardForLoco.id;
          } else {
            const cardResult = await commands.addMaintenanceCard(locoId);
            if (cardResult.status !== 'ok') {
              throw new Error(JSON.stringify(cardResult.error));
            }
            cardId = cardResult.data;
          }

          const trimmedCondition = !hasMaintenanceCard
            ? ($form.initialCondition as string).trim()
            : '';
          let notesText = ($form.notes as string).trim();
          if (trimmedCondition) {
            notesText = trimmedCondition + (notesText ? '\n' + notesText : '');
          }

          const nextMaintenanceDate =
            (($form.nextMaintenanceDate as string | null) ?? '').trim() || null;

          const eventResult = await commands.addMaintenanceEvent({
            maintenanceCardId: cardId,
            datePerformed: $form.datePerformed as string,
            maintenanceType: $form.maintenanceType as MaintenanceType | null,
            notes: notesText || null,
            nextMaintenanceDate
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
    }
  );

  const maintenanceTypes = getMaintenanceTypes();

  // Searchable combobox options derived from loaded rolling stocks
  const rsOptions = $derived(
    rollingStocks.map((rs) => ({
      value: rs.id,
      label: [rs.railwayCompanyName, rs.series, rs.roadNumber].filter(Boolean).join(' - ') || rs.id
    }))
  );

  // Cross-reference selected loco against existing maintenance cards
  const existingCardForLoco = $derived(
    maintenanceCards.find(
      (c) => c.ownedRollingStockId === ($form.selectedLocoId as string | null)
    ) ?? null
  );

  // Smart-switch: does the selected loco already have a maintenance card?
  const hasMaintenanceCard = $derived(existingCardForLoco !== null);

  const hasChanges = $derived(isTainted($tainted));

  // Reset form synchronously before render so isDirty is false when the
  // drawer first becomes visible — prevents a spurious Discard dialog on
  // re-open after a prior dirty session.
  $effect.pre(() => {
    if (open) {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      reset({ data: createInitialFormState() as any });
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
    if ($form.selectedLocoId) error = null;
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

  function handleSubmit() {
    formEl?.requestSubmit();
  }
</script>

<DrawerShell
  {open}
  {onClose}
  size="xl"
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

  <form bind:this={formEl} use:enhance class="space-y-5">
    <!-- Hidden submit button to enable Enter-to-submit keyboard navigability -->
    <button type="submit" class="hidden" aria-hidden="true" tabindex="-1"></button>

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
          value={($form.selectedLocoId as string | null) ?? ''}
          placeholder={m.log_maintenance_select_loco_placeholder()}
          onSelect={(val) => {
            $form.selectedLocoId = val || null;
          }}
        />
      {/if}
      {#if $errors.selectedLocoId?.[0]}
        <p class="text-xs text-destructive">{$errors.selectedLocoId[0]}</p>
      {/if}
    </div>

    <!-- Dynamic form unfolding: appears after a loco is chosen -->
    {#if ($form.selectedLocoId as string | null) !== null}
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
              value={$form.initialCondition as string}
              oninput={(e) => ($form.initialCondition = (e.target as HTMLTextAreaElement).value)}
            ></textarea>
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
            class="flex h-10 w-full rounded-md border border-border bg-card px-3 py-2 font-mono text-sm text-foreground ring-offset-background transition-all focus:border-primary/50 focus:ring-2 focus:ring-ring focus:outline-none {$errors.datePerformed
              ? 'border-destructive'
              : ''}"
            value={$form.datePerformed as string}
            oninput={(e) => ($form.datePerformed = (e.target as HTMLInputElement).value)}
          />
          {#if $errors.datePerformed?.[0]}
            <p class="text-xs text-destructive">{$errors.datePerformed[0]}</p>
          {/if}
        </div>

        <div class="space-y-2">
          <label
            for="next-maintenance-date"
            class="text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase"
          >
            {m.maintenance_schedule_next_date_label()}
          </label>
          <input
            id="next-maintenance-date"
            type="date"
            class="flex h-10 w-full rounded-md border border-border bg-card px-3 py-2 font-mono text-sm text-foreground ring-offset-background transition-all focus:border-primary/50 focus:ring-2 focus:ring-ring focus:outline-none"
            min={$form.datePerformed as string}
            value={($form.nextMaintenanceDate as string | null) ?? ''}
            oninput={(e) => {
              const value = (e.target as HTMLInputElement).value;
              $form.nextMaintenanceDate = value || null;
            }}
          />
          <p class="text-xs leading-relaxed text-muted-foreground">
            {m.maintenance_schedule_next_date_hint()}
          </p>
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
            value={($form.maintenanceType as string | null) ?? ''}
            onchange={(e) => {
              const v = (e.target as HTMLSelectElement).value;
              $form.maintenanceType = v || null;
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
            value={$form.notes as string}
            oninput={(e) => ($form.notes = (e.target as HTMLTextAreaElement).value)}
          ></textarea>
        </div>
      </div>
    {/if}
  </form>

  {#snippet footer({ requestClose })}
    <DrawerFooter
      cancelLabel={m.maintenance_add_event_cancel()}
      submitLabel={m.log_maintenance_log_button()}
      onCancel={requestClose}
      onSubmit={handleSubmit}
      submitting={isSubmitting}
    />
  {/snippet}
</DrawerShell>
