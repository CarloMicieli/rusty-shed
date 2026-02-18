<script lang="ts">
  import { X, ClipboardList } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { Button } from '$lib/components';
  import { getMaintenanceState } from '../MaintenanceState.svelte';
  import type { MaintenanceType } from '$lib/bindings';
  import { toaster } from '$lib/toaster';

  let { open, onClose } = $props<{ open: boolean; onClose: () => void }>();

  const maintenanceState = getMaintenanceState();
  const cards = $derived(maintenanceState.cards);

  let selectedCardId = $state<string | null>(null);
  let datePerformed = $state<string>(new Date().toISOString().split('T')[0]);
  let maintenanceType = $state<string | null>(null);
  let notes = $state<string>('');
  let isSubmitting = $state(false);
  let error = $state<string | null>(null);

  const isFormValid = $derived(selectedCardId !== null && datePerformed !== '');

  // Maintenance type options
  const maintenanceTypes: Array<{ value: MaintenanceType; label: string }> = [
    { value: 'WHEEL_CLEANING', label: m.maintenance_type_wheel_cleaning() },
    { value: 'TRACK_CLEANING', label: m.maintenance_type_track_cleaning() },
    { value: 'CONTACT_CLEANING', label: m.maintenance_type_contact_cleaning() },
    { value: 'LUBRICATION', label: m.maintenance_type_lubrication() },
    { value: 'GENERAL_INSPECTION', label: m.maintenance_type_inspection() },
    { value: 'DETAIL_REPAIR', label: m.maintenance_type_repair() }
  ];

  // Format a card label: prefer rolling stock ID suffix + due date
  function formatCard(card: {
    id: string;
    ownedRollingStockId: string;
    nextMaintenanceDate: string | null;
  }): string {
    // Extract the short UUID suffix from the owned rolling stock TRN
    const idParts = card.ownedRollingStockId.split(':');
    const shortId = idParts[idParts.length - 1]?.slice(0, 8) ?? card.ownedRollingStockId;
    const due = card.nextMaintenanceDate ? `Due ${card.nextMaintenanceDate}` : 'No due date';
    return `${shortId}… — ${due}`;
  }

  // Prevent background scrolling when modal is open
  $effect(() => {
    if (open) {
      document.body.style.overflow = 'hidden';
      return () => {
        document.body.style.overflow = '';
      };
    }
  });

  async function handleSubmit() {
    if (!isFormValid || !selectedCardId || !datePerformed) return;

    isSubmitting = true;
    error = null;

    try {
      await maintenanceState.addMaintenanceEvent({
        id: crypto.randomUUID(),
        maintenanceCardId: selectedCardId,
        datePerformed,
        maintenanceType,
        notes: notes.trim() || null
      });

      toaster.success({
        id: crypto.randomUUID(),
        title: m.maintenance_add_event_success(),
        duration: 3000
      });

      resetForm();
      onClose();
    } catch (err) {
      error = err instanceof Error ? err.message : m.maintenance_add_event_error();
      console.error('[AddMaintenanceEventModal] Submit error:', err);
    } finally {
      isSubmitting = false;
    }
  }

  function resetForm() {
    selectedCardId = null;
    datePerformed = new Date().toISOString().split('T')[0];
    maintenanceType = null;
    notes = '';
    error = null;
  }

  function handleClose() {
    resetForm();
    onClose();
  }
</script>

{#if open}
  <!-- Backdrop -->
  <div
    class="fixed inset-0 z-[100] flex items-center justify-center bg-black/80 p-4 backdrop-blur-md"
    aria-modal="true"
    role="dialog"
  >
    <!-- Modal container -->
    <div
      class="relative w-full max-w-lg overflow-hidden rounded-2xl border border-amber-500/20 bg-[#0c0c0c] shadow-2xl"
    >
      <!-- Background context icon -->
      <div class="pointer-events-none absolute -top-6 -right-6 text-white/5">
        <ClipboardList size={120} strokeWidth={1} />
      </div>

      <!-- Header -->
      <div class="flex items-center justify-between border-b border-white/5 p-6">
        <div class="flex items-center gap-3">
          <div class="rounded-lg bg-amber-500/10 p-2 text-amber-500">
            <ClipboardList size={20} />
          </div>
          <h3 class="text-lg font-bold text-zinc-100">{m.maintenance_add_event_title()}</h3>
        </div>
        <button
          type="button"
          class="text-zinc-500 transition-colors hover:text-white"
          onclick={handleClose}
          aria-label="Close"
        >
          <X size={20} />
        </button>
      </div>

      <!-- Form -->
      <form
        onsubmit={(e) => {
          e.preventDefault();
          handleSubmit();
        }}
        class="space-y-5 p-6"
      >
        <!-- Maintenance Card selector -->
        <div class="space-y-2">
          <label
            for="event-card-select"
            class="text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
          >
            {m.maintenance_add_event_select_card()}
          </label>
          <select
            id="event-card-select"
            class="flex h-10 w-full rounded-md border border-white/10 bg-zinc-950 px-3 py-2 text-sm text-zinc-100 ring-offset-black transition-all focus:border-amber-500/50 focus:ring-2 focus:ring-amber-500 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
            value={selectedCardId ?? ''}
            onchange={(e) => {
              selectedCardId = (e.target as HTMLSelectElement).value || null;
            }}
          >
            <option value="" disabled>{m.maintenance_add_event_card_placeholder()}</option>
            {#each cards as card (card.id)}
              <option value={card.id} class="bg-zinc-950 text-zinc-100">
                {formatCard(card)}
              </option>
            {/each}
          </select>
          {#if cards.length === 0}
            <p class="text-[11px] text-amber-500/70">
              No maintenance cards available. Create a card first.
            </p>
          {/if}
        </div>

        <!-- Date Performed -->
        <div class="space-y-2">
          <label
            for="event-date-performed"
            class="text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
          >
            {m.maintenance_add_event_date_label()}
          </label>
          <input
            id="event-date-performed"
            type="date"
            bind:value={datePerformed}
            max={new Date().toISOString().split('T')[0]}
            class="flex h-10 w-full rounded-md border border-white/10 bg-zinc-950 px-3 py-2 text-sm text-zinc-100 [color-scheme:dark] ring-offset-black transition-all focus:border-amber-500/50 focus:ring-2 focus:ring-amber-500 focus:outline-none"
          />
        </div>

        <!-- Maintenance Type -->
        <div class="space-y-2">
          <label
            for="event-maintenance-type"
            class="text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
          >
            {m.maintenance_add_event_type_label()}
          </label>
          <select
            id="event-maintenance-type"
            class="flex h-10 w-full rounded-md border border-white/10 bg-zinc-950 px-3 py-2 text-sm text-zinc-100 ring-offset-black transition-all focus:border-amber-500/50 focus:ring-2 focus:ring-amber-500 focus:outline-none"
            value={maintenanceType ?? ''}
            onchange={(e) => {
              maintenanceType = (e.target as HTMLSelectElement).value || null;
            }}
          >
            <option value="">{m.maintenance_add_event_type_placeholder()}</option>
            {#each maintenanceTypes as type (type.value)}
              <option value={type.value} class="bg-zinc-950 text-zinc-100">{type.label}</option>
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
            rows={3}
            placeholder={m.maintenance_add_event_notes_placeholder()}
            bind:value={notes}
            class="scrollbar-thin scrollbar-track-zinc-900 scrollbar-thumb-zinc-700 w-full resize-none rounded-md border border-white/10 bg-zinc-950 px-3 py-2 text-sm text-zinc-100 ring-offset-black transition-all placeholder:text-zinc-600 focus:border-amber-500/50 focus:ring-2 focus:ring-amber-500 focus:outline-none"
          ></textarea>
        </div>

        <!-- Error -->
        {#if error}
          <div class="rounded-lg border border-red-500/20 bg-red-500/10 p-3 text-xs text-red-400">
            {error}
          </div>
        {/if}

        <!-- Validation hints -->
        {#if !selectedCardId && cards.length > 0}
          <p class="text-xs font-medium text-amber-500/70">
            {m.maintenance_add_event_validation_card()}
          </p>
        {/if}

        <!-- Actions -->
        <div class="flex items-center justify-end gap-4 border-t border-white/5 pt-5">
          <Button variant="ghost" onclick={handleClose} class="text-zinc-500 hover:text-zinc-100">
            {m.maintenance_add_event_cancel()}
          </Button>
          <Button
            type="submit"
            variant="rusty"
            disabled={!isFormValid || isSubmitting}
            class="min-w-[120px]"
          >
            {#if isSubmitting}
              <span
                class="mr-2 h-4 w-4 animate-spin rounded-full border-2 border-black border-t-transparent"
              ></span>
              {m.app_loading()}
            {:else}
              {m.maintenance_add_event_submit()}
            {/if}
          </Button>
        </div>
      </form>
    </div>
  </div>
{/if}
