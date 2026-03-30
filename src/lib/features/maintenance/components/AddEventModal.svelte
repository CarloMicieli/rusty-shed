<script lang="ts">
  import { X, ClipboardList } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import * as Dialog from '$lib/components/ui/dialog';
  import { Button, DatePickerField } from '$lib/components';
  import { CalendarDate } from '@internationalized/date';
  import { getMaintenanceDetailState } from '../MaintenanceDetailState.svelte';
  import type { MaintenanceType } from '$lib/bindings';

  interface Props {
    open: boolean;
    onClose: () => void;
    maintenanceCardId: string;
  }

  const { open, onClose, maintenanceCardId }: Props = $props();

  const maintenanceDetailState = getMaintenanceDetailState();

  let datePerformed = $state<string | null>(new Date().toISOString().split('T')[0]);

  const today = $derived.by(() => {
    const n = new Date();
    return new CalendarDate(n.getFullYear(), n.getMonth() + 1, n.getDate());
  });
  let maintenanceType = $state<string | null>(null);
  let notes = $state<string>('');
  let isSubmitting = $state(false);
  let error = $state<string | null>(null);

  const isFormValid = $derived(datePerformed !== '' && datePerformed !== null);

  const maintenanceTypes: Array<{ value: MaintenanceType; label: string }> = [
    { value: 'WHEEL_CLEANING', label: m.maintenance_type_wheel_cleaning() },
    { value: 'TRACK_CLEANING', label: m.maintenance_type_track_cleaning() },
    { value: 'CONTACT_CLEANING', label: m.maintenance_type_contact_cleaning() },
    { value: 'LUBRICATION', label: m.maintenance_type_lubrication() },
    { value: 'GENERAL_INSPECTION', label: m.maintenance_type_inspection() },
    { value: 'DETAIL_REPAIR', label: m.maintenance_type_repair() }
  ];

  async function handleSubmit() {
    if (!isFormValid || !datePerformed) return;

    isSubmitting = true;
    error = null;

    const date = datePerformed;

    try {
      await maintenanceDetailState.addEvent({
        maintenanceCardId,
        datePerformed: date,
        maintenanceType,
        notes: notes.trim() || null
      });

      resetForm();
      onClose();
    } catch (err) {
      error = err instanceof Error ? err.message : m.maintenance_add_event_error();
      console.error('[AddEventModal] Submit error:', err);
    } finally {
      isSubmitting = false;
    }
  }

  function resetForm() {
    const n = new Date();
    datePerformed = `${n.getFullYear()}-${String(n.getMonth() + 1).padStart(2, '0')}-${String(n.getDate()).padStart(2, '0')}`;
    maintenanceType = null;
    notes = '';
    error = null;
  }

  function handleClose() {
    resetForm();
    onClose();
  }
</script>

<Dialog.Root
  {open}
  onOpenChange={(o) => {
    if (!o) handleClose();
  }}
>
  <Dialog.Content
    showCloseButton={false}
    class="max-w-lg overflow-hidden rounded-2xl border-amber-500/20 bg-layout-surface p-0"
  >
    <div class="pointer-events-none absolute -top-6 -right-6 text-white/5">
      <ClipboardList size={120} strokeWidth={1} />
    </div>

    <!-- Header -->
    <Dialog.Header class="flex-row items-center justify-between border-b border-white/5 p-6">
      <div class="flex items-center gap-3">
        <div class="rounded-lg bg-amber-500/10 p-2 text-amber-500">
          <ClipboardList size={20} />
        </div>
        <Dialog.Title class="text-lg font-bold text-zinc-100">
          {m.maintenance_add_event_title()}
        </Dialog.Title>
      </div>
      <button
        type="button"
        class="text-zinc-500 transition-colors hover:text-white"
        onclick={handleClose}
        aria-label={m.dialog_close_button()}
      >
        <X size={20} />
      </button>
    </Dialog.Header>

    <!-- Form -->
    <form
      onsubmit={(e) => {
        e.preventDefault();
        handleSubmit();
      }}
      class="space-y-5 p-6"
    >
      <!-- Date Performed -->
      <div class="space-y-2">
        <label
          for="add-event-date"
          class="text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
        >
          {m.maintenance_add_event_date_label()}
        </label>
        <DatePickerField id="add-event-date" bind:value={datePerformed} maxValue={today} />
        {#if !datePerformed}
          <p class="text-xs font-medium text-red-500">{m.maintenance_event_date_required()}</p>
        {/if}
      </div>

      <!-- Maintenance Type -->
      <div class="space-y-2">
        <label
          for="add-event-type"
          class="text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
        >
          {m.maintenance_add_event_type_label()}
        </label>
        <select
          id="add-event-type"
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
          for="add-event-notes"
          class="text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
        >
          {m.maintenance_add_event_notes_label()}
        </label>
        <textarea
          id="add-event-notes"
          rows={3}
          placeholder={m.maintenance_add_event_notes_placeholder()}
          bind:value={notes}
          class="scrollbar-thin scrollbar-track-zinc-900 scrollbar-thumb-zinc-700 w-full resize-none rounded-md border border-white/10 bg-zinc-950 px-3 py-2 text-sm text-zinc-100 ring-offset-black transition-all placeholder:text-zinc-600 focus:border-amber-500/50 focus:ring-2 focus:ring-amber-500 focus:outline-none"
        ></textarea>
      </div>

      {#if error}
        <div class="rounded-lg border border-red-500/20 bg-red-500/10 p-3 text-xs text-red-400">
          {error}
        </div>
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
  </Dialog.Content>
</Dialog.Root>
